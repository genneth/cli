// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use anyhow::Context;
use std::path::PathBuf;

/// Options for the `+attachment` subcommand, derived from CLI arguments.
#[derive(Debug, PartialEq, Eq)]
struct AttachmentConfig {
    message_id: String,
    target: TargetOption,
    output_dir: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
enum TargetOption {
    /// Download a single attachment by exact filename, optionally mapping to a specific output path
    Name {
        filename: String,
        output_file: Option<PathBuf>,
    },
    /// Download all attachments
    All,
}

impl AttachmentConfig {
    fn parse(matches: &ArgMatches) -> Result<Self, anyhow::Error> {
        let message_id = matches
            .get_one::<String>("message-id")
            .cloned()
            .context("Missing required --message-id argument")?;

        let all = matches.get_flag("all");
        let name = matches.get_one::<String>("name");

        let target = if all {
            TargetOption::All
        } else if let Some(n) = name {
            let output_file = matches.get_one::<String>("output").map(PathBuf::from);
            TargetOption::Name {
                filename: n.clone(),
                output_file,
            }
        } else {
            anyhow::bail!("Must specify either --name <FILENAME> or --all");
        };

        let output_dir = matches
            .get_one::<String>("output-dir")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));

        crate::validate::validate_safe_output_dir(
            output_dir.to_str().context("Invalid output directory")?,
        )?;
        if let TargetOption::Name {
            output_file: Some(path),
            ..
        } = &target
        {
            crate::validate::validate_safe_output_dir(
                path.to_str().context("Invalid output path")?,
            )?;
            if path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
            {
                anyhow::bail!("--output must not contain parent traversal");
            }
        }

        Ok(Self {
            message_id,
            target,
            output_dir,
        })
    }
}

/// Handle the `+attachment` subcommand.
pub(super) async fn handle_attachment(
    _doc: &crate::discovery::RestDescription,
    matches: &ArgMatches,
) -> Result<(), GwsError> {
    let config = AttachmentConfig::parse(matches)?;
    let output_root = crate::validate::validate_safe_output_dir(
        config
            .output_dir
            .to_str()
            .context("Invalid output directory")?,
    )?;
    if matches.get_flag("dry-run") {
        println!(
            "{}",
            json!({"dryRun":true,"messageId":config.message_id,"outputDir":output_root})
        );
        return Ok(());
    }
    let token = auth::get_token(&[GMAIL_READONLY_SCOPE])
        .await
        .map_err(|e| GwsError::Auth(format!("Gmail authentication failed: {e}")))?;
    let client = crate::client::build_client()?;
    let message = content::fetch_message(&client, &token, &config.message_id).await?;
    let matching_parts: Vec<_> = message
        .attachments
        .iter()
        .filter(|part| match &config.target {
            TargetOption::All => true,
            TargetOption::Name { filename, .. } => part.filename.eq_ignore_ascii_case(filename),
        })
        .collect();
    if matching_parts.is_empty() && matches!(config.target, TargetOption::Name { .. }) {
        return Err(anyhow::anyhow!("No attachment matches the requested filename").into());
    }
    // Plan every destination before any write. Existing files and symlinks are never overwritten.
    let mut reserved = std::collections::HashSet::new();
    let mut downloads = Vec::new();
    let mut total_bytes = 0usize;
    for (index, part) in matching_parts.iter().enumerate() {
        let path = resolve_destination_path(&config, part, matching_parts.len(), index)?;
        let path = crate::validate::validate_safe_output_dir(
            path.to_str().context("Invalid output path")?,
        )?;
        if !path.starts_with(&output_root) {
            return Err(GwsError::Validation(
                "Attachment output escapes --output-dir".into(),
            ));
        }
        let path = unique_destination(&path, &mut reserved)?;
        let bytes = match &part.data {
            Some(data) => data.clone(),
            None => {
                fetch_attachment_data(&client, &token, &config.message_id, &part.attachment_id)
                    .await?
            }
        };
        total_bytes += bytes.len();
        if total_bytes as u64 > MAX_TOTAL_ATTACHMENT_BYTES {
            return Err(GwsError::Validation(
                "Total attachment download exceeds 25 MB".into(),
            ));
        }
        downloads.push((part, path, bytes));
    }
    let mut files = Vec::new();
    for (part, path, bytes) in downloads {
        let parent = path.parent().context("Missing output parent")?;
        std::fs::create_dir_all(parent).context("Cannot create attachment directory")?;
        let mut temp =
            tempfile::NamedTempFile::new_in(parent).context("Cannot create attachment file")?;
        use std::io::Write;
        temp.write_all(&bytes)
            .context("Cannot write attachment bytes")?;
        temp.persist_noclobber(&path).map_err(|e| {
            anyhow::anyhow!("Cannot save {} without overwriting: {e}", path.display())
        })?;
        files.push(json!({"message_id":config.message_id,"attachment_id":part.attachment_id,
            "filename":part.filename,"content_type":part.content_type,"size":bytes.len(),"path":path}));
    }
    if matches
        .get_one::<String>("format")
        .map(String::as_str)
        .unwrap_or("json")
        == "text"
    {
        for file in &files {
            println!(
                "Saved to {}",
                sanitize_for_terminal(file["path"].as_str().unwrap_or(""))
            );
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({"files":files,"complete":true}))
                .context("Cannot serialize downloads")?
        );
    }
    Ok(())
}

fn unique_destination(
    path: &std::path::Path,
    reserved: &mut std::collections::HashSet<PathBuf>,
) -> Result<PathBuf, GwsError> {
    for index in 0..10_000 {
        let candidate = if index == 0 {
            path.to_owned()
        } else {
            let stem = path
                .file_stem()
                .context("Missing attachment filename")?
                .to_string_lossy();
            let extension = path
                .extension()
                .map(|s| format!(".{}", s.to_string_lossy()))
                .unwrap_or_default();
            path.with_file_name(format!("{stem}_{index}{extension}"))
        };
        match std::fs::symlink_metadata(&candidate) {
            Ok(_) => continue,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(anyhow::Error::from(e).into()),
        }
        if reserved.insert(candidate.clone()) {
            return Ok(candidate);
        }
    }
    Err(anyhow::anyhow!("Too many attachment filename collisions").into())
}

/// Resolves the final destination path for a given part, handling naming conflicts.
fn resolve_destination_path(
    config: &AttachmentConfig,
    part: &OriginalPart,
    total_matches: usize,
    index: usize,
) -> Result<PathBuf, GwsError> {
    match &config.target {
        TargetOption::All => {
            // Downloading all attachments -> place them under output_dir using their original filenames
            Ok(config.output_dir.join(&part.filename))
        }
        TargetOption::Name { output_file, .. } => {
            if let Some(out) = output_file {
                // If a specific output path was requested
                if total_matches > 1 {
                    // Conflict resolution: if multiple attachments have the same requested filename,
                    // append index suffix to prevent files overwriting each other
                    let stem = out
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "attachment".to_string());
                    let ext = out
                        .extension()
                        .map(|e| format!(".{}", e.to_string_lossy()))
                        .unwrap_or_default();

                    Ok(config
                        .output_dir
                        .join(format!("{}_{}{}", stem, index + 1, ext)))
                } else {
                    Ok(config.output_dir.join(out))
                }
            } else {
                // No specific output filename requested -> use the original filename
                Ok(config.output_dir.join(&part.filename))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn duplicate_and_existing_names_get_distinct_destinations() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("invoice.pdf");
        std::fs::write(&path, b"existing").unwrap();
        let mut reserved = std::collections::HashSet::new();
        let first = unique_destination(&path, &mut reserved).unwrap();
        let second = unique_destination(&path, &mut reserved).unwrap();
        assert_eq!(first.file_name().unwrap(), "invoice_1.pdf");
        assert_eq!(second.file_name().unwrap(), "invoice_2.pdf");
        assert_eq!(std::fs::read(path).unwrap(), b"existing");
    }

    #[cfg(unix)]
    #[test]
    fn dangling_symlinks_are_not_download_targets() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("invoice.pdf");
        std::os::unix::fs::symlink(dir.path().join("missing"), &path).unwrap();
        let actual = unique_destination(&path, &mut Default::default()).unwrap();
        assert_ne!(actual, path);
        assert!(path.is_symlink());
    }

    fn build_test_command() -> Command {
        Command::new("test")
            .arg(Arg::new("message-id").long("message-id").short('m'))
            .arg(Arg::new("name").long("name").short('n'))
            .arg(
                Arg::new("all")
                    .long("all")
                    .short('a')
                    .action(ArgAction::SetTrue),
            )
            .arg(Arg::new("output").long("output").short('o'))
            .arg(Arg::new("output-dir").long("output-dir").short('d'))
            .arg(
                Arg::new("dry-run")
                    .long("dry-run")
                    .action(ArgAction::SetTrue),
            )
    }

    #[test]
    fn output_paths_reject_absolute_traversal_and_controls() {
        for path in ["/tmp/out", "../../out", "bad\nname"] {
            let matches = build_test_command().get_matches_from([
                "test",
                "--message-id",
                "m",
                "--all",
                "--output-dir",
                path,
            ]);
            assert!(
                AttachmentConfig::parse(&matches).is_err(),
                "accepted {path:?}"
            );
            let matches = build_test_command().get_matches_from([
                "test",
                "--message-id",
                "m",
                "--name",
                "a.pdf",
                "--output",
                path,
            ]);
            assert!(
                AttachmentConfig::parse(&matches).is_err(),
                "accepted {path:?}"
            );
        }
    }

    #[test]
    fn test_parse_config_valid_name() {
        let cmd = build_test_command();
        let matches = cmd.get_matches_from(vec![
            "test",
            "--message-id",
            "msg-123",
            "--name",
            "invoice.pdf",
            "--output",
            "invoice.pdf",
        ]);

        let config = AttachmentConfig::parse(&matches).unwrap();
        assert_eq!(config.message_id, "msg-123");
        assert_eq!(
            config.target,
            TargetOption::Name {
                filename: "invoice.pdf".to_string(),
                output_file: Some(PathBuf::from("invoice.pdf")),
            }
        );
        assert_eq!(config.output_dir, PathBuf::from("."));
    }

    #[test]
    fn test_parse_config_valid_all() {
        let cmd = build_test_command();
        let matches = cmd.get_matches_from(vec![
            "test",
            "-m",
            "msg-123",
            "--all",
            "--output-dir",
            "downloads",
        ]);

        let config = AttachmentConfig::parse(&matches).unwrap();
        assert_eq!(config.message_id, "msg-123");
        assert_eq!(config.target, TargetOption::All);
        assert_eq!(config.output_dir, PathBuf::from("downloads"));
    }

    #[test]
    fn test_parse_config_missing_required() {
        let cmd = build_test_command();
        let matches = cmd.get_matches_from(vec!["test", "--name", "invoice.pdf"]);
        assert!(AttachmentConfig::parse(&matches).is_err());
    }

    #[test]
    fn test_parse_config_missing_targets() {
        let cmd = build_test_command();
        let matches = cmd.get_matches_from(vec!["test", "-m", "msg-123"]);
        assert!(AttachmentConfig::parse(&matches).is_err());
    }

    #[test]
    fn test_resolve_destination_path_all() {
        let config = AttachmentConfig {
            message_id: "msg-123".to_string(),
            target: TargetOption::All,
            output_dir: PathBuf::from("/tmp/downloads"),
        };
        let part = OriginalPart {
            filename: "photo.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            size: 100,
            attachment_id: "att-1".to_string(),
            data: None,
            content_id: None,
        };

        let dest = resolve_destination_path(&config, &part, 1, 0).unwrap();
        assert_eq!(dest, PathBuf::from("/tmp/downloads/photo.jpg"));
    }

    #[test]
    fn test_resolve_destination_path_name_with_conflict() {
        let config = AttachmentConfig {
            message_id: "msg-123".to_string(),
            target: TargetOption::Name {
                filename: "photo.jpg".to_string(),
                output_file: Some(PathBuf::from("my_photo.jpg")),
            },
            output_dir: PathBuf::from("/tmp/downloads"),
        };
        let part = OriginalPart {
            filename: "photo.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            size: 100,
            attachment_id: "att-1".to_string(),
            data: None,
            content_id: None,
        };

        // Conflict: multiple matching files (total_matches = 2, index = 1) -> should add suffix
        let dest = resolve_destination_path(&config, &part, 2, 1).unwrap();
        assert_eq!(dest, PathBuf::from("/tmp/downloads/my_photo_2.jpg"));
    }
}
