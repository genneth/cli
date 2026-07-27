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
use std::path::PathBuf;
use anyhow::Context;

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
    let dry_run = matches.get_flag("dry-run");

    if dry_run {
        println!("Dry run: would download attachments for message {}", config.message_id);
        match &config.target {
            TargetOption::All => println!("Target: all attachments"),
            TargetOption::Name { filename, output_file } => {
                println!("Target name: {}", filename);
                if let Some(out) = output_file {
                    println!("Destination: {}", out.display());
                }
            }
        }
        return Ok(());
    }

    // Authenticate and acquire token
    let token = auth::get_token(&[GMAIL_READONLY_SCOPE])
        .await
        .map_err(|e| GwsError::Auth(format!("Gmail authentication failed: {e}")))?;
    
    let client = crate::client::build_client()?;

    // Retrieve original message metadata to scan its attachments list
    let original = fetch_message_metadata(&client, &token, &config.message_id).await?;

    // Filter parts matching target option
    let matching_parts: Vec<&OriginalPart> = original
        .parts
        .iter()
        .filter(|part| match &config.target {
            TargetOption::All => !part.attachment_id.is_empty(),
            TargetOption::Name { filename, .. } => part.filename.eq_ignore_ascii_case(filename),
        })
        .collect();

    if matching_parts.is_empty() {
        match &config.target {
            TargetOption::All => {
                println!("No attachments found in message {}", config.message_id);
                return Ok(());
            }
            TargetOption::Name { filename, .. } => {
                return Err(GwsError::Other(anyhow::anyhow!(
                    "No attachment found matching name '{}' in message {}",
                    filename,
                    config.message_id
                )));
            }
        }
    }

    // Ensure output directory exists (prevent filesystem writes to invalid paths)
    if !config.output_dir.exists() {
        std::fs::create_dir_all(&config.output_dir)
            .context(format!(
                "Failed to create output directory {}",
                config.output_dir.display()
            ))?;
    }

    for (idx, part) in matching_parts.iter().enumerate() {
        let dest = resolve_destination_path(&config, part, matching_parts.len(), idx)?;

        println!("Downloading attachment '{}' (size: {} bytes)", part.filename, part.size);

        let data = fetch_attachment_data(&client, &token, &config.message_id, &part.attachment_id).await?;

        // Exhaustive error check: verify destination is not a directory
        if dest.is_dir() {
            return Err(anyhow::anyhow!("Cannot write to {} because it is an existing directory", dest.display()).into());
        }

        std::fs::write(&dest, &data)
            .context(format!("Failed to write downloaded bytes to file {}", dest.display()))?;

        println!("Saved to {}", dest.display());
    }

    Ok(())
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
                    let stem = out.file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "attachment".to_string());
                    let ext = out.extension()
                        .map(|e| format!(".{}", e.to_string_lossy()))
                        .unwrap_or_default();
                    
                    Ok(config.output_dir.join(format!("{}_{}{}", stem, index + 1, ext)))
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

    fn build_test_command() -> Command {
        Command::new("test")
            .arg(Arg::new("message-id").long("message-id").short('m'))
            .arg(Arg::new("name").long("name").short('n'))
            .arg(Arg::new("all").long("all").short('a').action(ArgAction::SetTrue))
            .arg(Arg::new("output").long("output").short('o'))
            .arg(Arg::new("output-dir").long("output-dir").short('d'))
            .arg(Arg::new("dry-run").long("dry-run").action(ArgAction::SetTrue))
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
            "/tmp/invoice.pdf",
        ]);

        let config = AttachmentConfig::parse(&matches).unwrap();
        assert_eq!(config.message_id, "msg-123");
        assert_eq!(
            config.target,
            TargetOption::Name {
                filename: "invoice.pdf".to_string(),
                output_file: Some(PathBuf::from("/tmp/invoice.pdf")),
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
            "/tmp/downloads",
        ]);

        let config = AttachmentConfig::parse(&matches).unwrap();
        assert_eq!(config.message_id, "msg-123");
        assert_eq!(config.target, TargetOption::All);
        assert_eq!(config.output_dir, PathBuf::from("/tmp/downloads"));
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
            content_id: None,
        };

        // Conflict: multiple matching files (total_matches = 2, index = 1) -> should add suffix
        let dest = resolve_destination_path(&config, &part, 2, 1).unwrap();
        assert_eq!(dest, PathBuf::from("/tmp/downloads/my_photo_2.jpg"));
    }
}
