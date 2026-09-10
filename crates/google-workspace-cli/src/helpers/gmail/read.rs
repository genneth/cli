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
use std::io::{self, Write};

/// Handle the `+read` subcommand.
pub(super) async fn handle_read(
    _doc: &crate::discovery::RestDescription,
    matches: &ArgMatches,
) -> Result<(), GwsError> {
    let message_id = matches.get_one::<String>("id");
    let thread_id = matches.get_one::<String>("thread-id");
    if matches.get_flag("dry-run") {
        println!(
            "{}",
            json!({"dryRun":true,"id":message_id,"threadId":thread_id})
        );
        return Ok(());
    }
    let token = auth::get_token(&[GMAIL_READONLY_SCOPE])
        .await
        .map_err(|e| GwsError::Auth(format!("Gmail auth failed: {e}")))?;
    let client = crate::client::build_client()?;
    let messages = if let Some(id) = thread_id {
        content::fetch_thread(&client, &token, id).await?
    } else {
        vec![
            content::fetch_message(&client, &token, message_id.context("Missing message id")?)
                .await?,
        ]
    };

    let format = matches.get_one::<String>("format").unwrap();
    let show_headers = matches.get_flag("headers");
    let use_html = matches.get_flag("html");

    let mut stdout = io::stdout().lock();

    if format == "json" {
        let value = if let Some(id) = thread_id {
            json!({"thread_id":id,"messages":messages,"complete":true})
        } else {
            serde_json::to_value(&messages[0]).context("Failed to serialize message")?
        };
        let json_output =
            serde_json::to_string_pretty(&value).context("Failed to serialize message to JSON")?;
        writeln!(stdout, "{}", json_output).context("Failed to write JSON output")?;
        return Ok(());
    }

    for message in &messages {
        write_message(
            &mut stdout,
            message,
            show_headers || thread_id.is_some(),
            use_html,
        )?;
    }
    Ok(())
}

pub(super) fn write_message(
    stdout: &mut impl Write,
    message: &content::ReadMessage,
    show_headers: bool,
    use_html: bool,
) -> Result<(), GwsError> {
    let original = &message.message;
    if show_headers {
        writeln!(stdout, "Gmail ID: {}", sanitize_for_terminal(&message.id))
            .context("Failed to write ID")?;
        // Format structured fields into display strings for header output.
        let from_str = original.from.to_string();
        let to_str = format_mailbox_list(&original.to);
        let cc_str = original
            .cc
            .as_ref()
            .map(|cc| format_mailbox_list(cc))
            .unwrap_or_default();

        let headers_to_show: [(&str, &str); 5] = [
            ("From", &from_str),
            ("To", &to_str),
            ("Cc", &cc_str),
            ("Subject", &original.subject),
            ("Date", original.date.as_deref().unwrap_or_default()),
        ];
        for (name, value) in headers_to_show {
            if value.is_empty() {
                continue;
            }
            // Replace newlines to prevent header spoofing in the output, then sanitize.
            let sanitized_value = sanitize_for_terminal(&value.replace(['\r', '\n'], " "));
            writeln!(stdout, "{}: {}", name, sanitized_value)
                .with_context(|| format!("Failed to write '{name}' header"))?;
        }
        writeln!(stdout, "---").context("Failed to write header separator")?;
    }

    if !original.parts.is_empty() {
        writeln!(stdout, "Attachments:").context("Failed to write attachments header")?;
        for part in &original.parts {
            writeln!(
                stdout,
                "  - {} ({} bytes)",
                sanitize_for_terminal(&part.filename),
                part.size
            )
            .context("Failed to write attachment details")?;
        }
        writeln!(stdout, "---").context("Failed to write attachments separator")?;
    }

    let body = if use_html {
        original
            .body_html
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&original.body_text)
    } else {
        &original.body_text
    };

    writeln!(stdout, "{}", sanitize_for_terminal(body)).context("Failed to write message body")?;

    Ok(())
}

/// Format a slice of Mailbox as a displayable comma-separated string.
fn format_mailbox_list(mailboxes: &[Mailbox]) -> String {
    mailboxes
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

use crate::output::sanitize_for_terminal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_for_terminal() {
        let malicious = "Subject: \x1b]0;MALICIOUS\x07Hello\nWorld\r\t";
        let sanitized = sanitize_for_terminal(malicious);
        // ANSI escape sequences (control chars) should be removed
        assert!(!sanitized.contains('\x1b'));
        assert!(!sanitized.contains('\x07'));
        // CR is also stripped (can be abused for terminal overwrite attacks)
        assert!(!sanitized.contains('\r'));
        // Newline and tab should be preserved
        assert!(sanitized.contains("Hello"));
        assert!(sanitized.contains('\n'));
        assert!(sanitized.contains('\t'));
    }

    #[test]
    fn test_format_mailbox_list_empty() {
        assert_eq!(format_mailbox_list(&[]), "");
    }

    #[test]
    fn test_format_mailbox_list_single() {
        let mailboxes = Mailbox::parse_list("alice@example.com");
        let result = format_mailbox_list(&mailboxes);
        assert!(result.contains("alice@example.com"));
    }

    #[test]
    fn test_format_mailbox_list_multiple() {
        let mailboxes = Mailbox::parse_list("alice@example.com, Bob <bob@example.com>");
        let result = format_mailbox_list(&mailboxes);
        assert!(result.contains("alice@example.com"));
        assert!(result.contains("bob@example.com"));
    }
}
