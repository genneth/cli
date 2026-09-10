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

//! Gmail `+triage` helper — lists unread messages with sender, subject, date.
//!
//! Read-only: fetches unread message metadata (From, Subject, Date) and
//! optionally includes label IDs. Supports custom Gmail search queries
//! via `--query` and configurable result limits via `--max`.

use super::*;

/// Handle the `+triage` subcommand.
pub async fn handle_triage(matches: &ArgMatches) -> Result<(), GwsError> {
    let max: u32 = matches
        .get_one::<String>("max")
        .and_then(|s| s.parse().ok())
        .unwrap_or(20);
    let query = matches
        .get_one::<String>("query")
        .map(|s| s.as_str())
        .unwrap_or("is:unread");
    let output_format = matches
        .get_one::<String>("format")
        .map(|s| crate::formatter::OutputFormat::from_str(s))
        .unwrap_or(crate::formatter::OutputFormat::Table);

    // Authenticate — use gmail.readonly instead of gmail.modify because triage
    // is read-only and the `q` query parameter is not supported under the
    // gmail.metadata scope.  When a token carries both metadata and modify
    // scopes the API may resolve to the metadata path and reject `q` with 403.
    // gmail.readonly always supports `q`.
    let token = auth::get_token(&[GMAIL_READONLY_SCOPE])
        .await
        .map_err(|e| GwsError::Auth(format!("Gmail auth failed: {e}")))?;

    let client = crate::client::build_client()?;

    if !(1..=1000).contains(&max) {
        return Err(GwsError::Validation("--max must be 1–1000".into()));
    }
    let page = search::list_message_ids(
        &client,
        &token,
        content::MESSAGES_URL,
        &json!({"q":query}),
        max,
        10,
    )
    .await?;
    use futures_util::stream::{self, StreamExt, TryStreamExt};
    let results: Vec<Value> = stream::iter(page.ids)
        .map(|id| {
            let client = &client;
            let token = &token;
            async move {
                let value = crate::helpers::retrieval::get_json(
                    client,
                    &format!(
                        "{}/{}",
                        content::MESSAGES_URL,
                        crate::validate::encode_path_segment(&id)
                    ),
                    token,
                    &[
                        ("format", "metadata"),
                        ("metadataHeaders", "From"),
                        ("metadataHeaders", "Subject"),
                        ("metadataHeaders", "Date"),
                    ],
                )
                .await?;
                triage_entry(&value)
            }
        })
        .buffered(10)
        .try_collect()
        .await?;
    if results.is_empty() {
        eprintln!("{}", no_messages_msg(query));
    }
    let output = json!({"messages":results,"query":query,
        "complete":page.next_page_token.is_none(), "nextPageToken":page.next_page_token});
    println!(
        "{}",
        crate::formatter::format_value(&output, &output_format)
    );
    Ok(())
}

fn triage_entry(value: &Value) -> Result<Value, GwsError> {
    let id = value
        .get("id")
        .and_then(Value::as_str)
        .context("Metadata response missing Gmail id")?;
    let headers = value
        .pointer("/payload/headers")
        .and_then(Value::as_array)
        .context("Metadata response missing headers")?;
    let parsed = parse_message_headers(headers);
    Ok(
        json!({"id":id, "threadId":value["threadId"], "from":parsed.from,
        "subject":parsed.subject,"date":parsed.date,"labels":value.get("labelIds").cloned().unwrap_or(json!([]))}),
    )
}

/// Returns the human-readable "no messages" diagnostic string.
/// Extracted so the test can reference the exact same message without duplication.
fn no_messages_msg(query: &str) -> String {
    format!(
        "No messages found matching query: {}",
        crate::output::sanitize_for_terminal(query)
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn metadata_keeps_ids_and_uses_documented_messages_envelope() {
        let entry = super::triage_entry(&serde_json::json!({"id":"g", "threadId":"t",
            "payload":{"headers":[{"name":"subject","value":"hello"}]}}))
        .unwrap();
        let output = serde_json::json!({"messages":[entry]});
        assert_eq!(output["messages"][0]["subject"], "hello");
        assert_eq!(output["messages"][0]["id"], "g");
        assert_eq!(output["messages"][0]["threadId"], "t");
        assert!(super::triage_entry(&serde_json::json!({})).is_err());
    }
    use super::no_messages_msg;
    use clap::{Arg, ArgAction, Command};

    /// Build a clap command matching the +triage definition so we can
    /// unit-test argument parsing without needing a live GmailHelper.
    fn triage_cmd() -> Command {
        Command::new("triage")
            .arg(
                Arg::new("max")
                    .long("max")
                    .default_value("20")
                    .value_name("N"),
            )
            .arg(Arg::new("query").long("query").value_name("QUERY"))
            .arg(Arg::new("labels").long("labels").action(ArgAction::SetTrue))
            .arg(Arg::new("format").long("format").value_name("FMT"))
    }

    #[test]
    fn defaults_max_to_20_and_query_to_unread() {
        let m = triage_cmd().try_get_matches_from(["triage"]).unwrap();
        let max: u32 = m
            .get_one::<String>("max")
            .and_then(|s| s.parse().ok())
            .unwrap_or(20);
        let query = m
            .get_one::<String>("query")
            .map(|s| s.as_str())
            .unwrap_or("is:unread");
        assert_eq!(max, 20);
        assert_eq!(query, "is:unread");
    }

    #[test]
    fn explicit_max_overrides_default() {
        let m = triage_cmd()
            .try_get_matches_from(["triage", "--max", "5"])
            .unwrap();
        let max: u32 = m
            .get_one::<String>("max")
            .and_then(|s| s.parse().ok())
            .unwrap_or(20);
        assert_eq!(max, 5);
    }

    #[test]
    fn non_numeric_max_falls_back_to_20() {
        let m = triage_cmd()
            .try_get_matches_from(["triage", "--max", "abc"])
            .unwrap();
        let max: u32 = m
            .get_one::<String>("max")
            .and_then(|s| s.parse().ok())
            .unwrap_or(20);
        assert_eq!(max, 20);
    }

    #[test]
    fn custom_query_overrides_default() {
        let m = triage_cmd()
            .try_get_matches_from(["triage", "--query", "from:boss"])
            .unwrap();
        let query = m
            .get_one::<String>("query")
            .map(|s| s.as_str())
            .unwrap_or("is:unread");
        assert_eq!(query, "from:boss");
    }

    #[test]
    fn labels_flag_defaults_to_false() {
        let m = triage_cmd().try_get_matches_from(["triage"]).unwrap();
        assert!(!m.get_flag("labels"));
    }

    #[test]
    fn labels_flag_set_when_passed() {
        let m = triage_cmd()
            .try_get_matches_from(["triage", "--labels"])
            .unwrap();
        assert!(m.get_flag("labels"));
    }

    #[test]
    fn format_defaults_to_table_when_absent() {
        let m = triage_cmd().try_get_matches_from(["triage"]).unwrap();
        let fmt = m
            .get_one::<String>("format")
            .map(|s| crate::formatter::OutputFormat::from_str(s))
            .unwrap_or(crate::formatter::OutputFormat::Table);
        assert!(matches!(fmt, crate::formatter::OutputFormat::Table));
    }

    #[test]
    fn format_json_when_specified() {
        let m = triage_cmd()
            .try_get_matches_from(["triage", "--format", "json"])
            .unwrap();
        let fmt = m
            .get_one::<String>("format")
            .map(|s| crate::formatter::OutputFormat::from_str(s))
            .unwrap_or(crate::formatter::OutputFormat::Table);
        assert!(matches!(fmt, crate::formatter::OutputFormat::Json));
    }

    #[test]
    fn empty_result_message_is_not_json() {
        // Verify that no_messages_msg() produces a human-readable string that
        // belongs on stderr, not stdout. If it were valid JSON it could corrupt
        // pipe workflows like `gws gmail +triage | jq`.
        let msg = no_messages_msg("label:inbox");
        assert!(serde_json::from_str::<serde_json::Value>(&msg).is_err());
    }
}
