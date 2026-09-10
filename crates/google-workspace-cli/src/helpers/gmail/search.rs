//! Search is list-then-fetch orchestration, with explicit bounds and continuation.
use super::*;
use crate::helpers::retrieval;
use futures_util::stream::{self, StreamExt, TryStreamExt};

pub(super) fn command() -> Command {
    Command::new("+search")
        .about("[Helper] Search Gmail and retrieve complete readable messages")
        .arg(Arg::new("params").long("params").default_value("{}")
            .help("Gmail messages.list parameters as JSON, e.g. {\"q\":\"from:alice\"}"))
        .arg(Arg::new("max-messages").long("max-messages").default_value("20")
            .value_parser(clap::value_parser!(u32).range(1..=1000))
            .help("Maximum messages to retrieve across pages (1–1000)"))
        .arg(Arg::new("page-limit").long("page-limit").default_value("10")
            .value_parser(clap::value_parser!(u32).range(1..=100))
            .help("Maximum search pages to fetch (1–100)"))
        .arg(Arg::new("format").long("format").default_value("json").value_parser(["json", "text"])
            .help("Output decoded messages as JSON or readable text"))
        .arg(Arg::new("dry-run").long("dry-run").action(ArgAction::SetTrue)
            .help("Validate search options without reading mail"))
        .after_help("EXAMPLES:
  gws gmail +search --params '{\"q\":\"from:alice has:attachment\"}' --max-messages 10
  gws gmail +search --params '{\"q\":\"newer_than:7d\"}' --format json

TIPS:
  Returns messages with full decoded bodies, Gmail IDs, thread IDs, and attachment metadata.
  complete=false and nextPageToken indicate more search results; resume with pageToken in --params.
  A failed fetch fails the command; omitted messages are never reported as a successful empty result.
  For a whole conversation use gws gmail +read --thread-id THREAD_ID.")
}

pub(super) struct MessagePage {
    pub ids: Vec<String>,
    pub next_page_token: Option<String>,
}

fn query_params(params: &Value) -> Result<Vec<(String, String)>, GwsError> {
    let object = params
        .as_object()
        .context("--params must be a JSON object")?;
    let mut query = Vec::new();
    for (key, value) in object {
        match key.as_str() {
            "q" | "pageToken" => query.push((key.clone(), value.as_str().context("q and pageToken must be strings")?.into())),
            "includeSpamTrash" => query.push((key.clone(), value.as_bool().context("includeSpamTrash must be boolean")?.to_string())),
            "labelIds" => {
                for label in value.as_array().context("labelIds must be an array")? {
                    query.push((key.clone(), label.as_str().context("labelIds must contain strings")?.into()));
                }
            }
            "maxResults" => {
                let max = value.as_u64().filter(|v| (1..=500).contains(v)).context("maxResults must be 1–500")?;
                query.push((key.clone(), max.to_string()));
            }
            _ => return Err(anyhow::anyhow!("Unsupported messages.list parameter {key}; use the Discovery command for raw responses").into()),
        }
    }
    Ok(query)
}

pub(super) async fn list_message_ids(
    client: &reqwest::Client,
    token: &str,
    url: &str,
    params: &Value,
    max: u32,
    pages: u32,
) -> Result<MessagePage, GwsError> {
    let mut query = query_params(params)?;
    let per_page = params
        .get("maxResults")
        .and_then(Value::as_u64)
        .unwrap_or(100)
        .min(500) as u32;
    let mut ids = Vec::new();
    let mut next_page_token = None;
    let mut seen = std::collections::HashSet::new();
    for _ in 0..pages {
        let remaining = max as usize - ids.len();
        if remaining == 0 {
            break;
        }
        let requested = (per_page as usize).min(remaining);
        query.retain(|(key, _)| key != "maxResults");
        query.push(("maxResults".into(), requested.to_string()));
        let refs: Vec<_> = query
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let value = retrieval::get_json(client, url, token, &refs).await?;
        let messages = retrieval::page_items(&value, "messages")?;
        if messages.len() > requested {
            return Err(anyhow::anyhow!("API returned more messages than requested").into());
        }
        for message in messages {
            ids.push(
                message
                    .get("id")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                    .context("Search result missing Gmail id")?
                    .to_owned(),
            );
        }
        next_page_token = retrieval::page_token(&value)?;
        let Some(ref next) = next_page_token else {
            break;
        };
        if !seen.insert(next.clone()) {
            return Err(anyhow::anyhow!("Repeated search pagination token").into());
        }
        query.retain(|(key, _)| key != "pageToken");
        query.push(("pageToken".into(), next.clone()));
    }
    Ok(MessagePage {
        ids,
        next_page_token,
    })
}

pub(super) async fn handle_search(matches: &ArgMatches) -> Result<(), GwsError> {
    let params: Value = serde_json::from_str(matches.get_one::<String>("params").unwrap())
        .context("Invalid --params JSON")?;
    query_params(&params)?;
    let max = *matches.get_one::<u32>("max-messages").unwrap();
    let pages = *matches.get_one::<u32>("page-limit").unwrap();
    if matches.get_flag("dry-run") {
        println!(
            "{}",
            json!({"dryRun":true,"params":params,"maxMessages":max,"pageLimit":pages})
        );
        return Ok(());
    }
    let token = auth::get_token(&[GMAIL_READONLY_SCOPE]).await
        .map_err(|e| GwsError::Auth(format!("Gmail auth failed: {e}")))?;
    let client = crate::client::build_client()?;
    let page =
        list_message_ids(&client, &token, content::MESSAGES_URL, &params, max, pages).await?;
    let messages: Vec<_> = stream::iter(page.ids)
        .map(|id| {
            let client = &client;
            let token = &token;
            async move { content::fetch_message(client, token, &id).await }
        })
        .buffered(5)
        .try_collect()
        .await?;
    let complete = page.next_page_token.is_none();
    if matches.get_one::<String>("format").unwrap() == "text" {
        for message in &messages {
            read::write_message(&mut std::io::stdout(), message, true, false)?;
        }
        if !complete {
            eprintln!(
                "More results available; resume with pageToken: {}",
                sanitize_for_terminal(page.next_page_token.as_deref().unwrap())
            );
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({"messages":messages, "complete":complete,
            "nextPageToken":page.next_page_token}))
            .context("Cannot serialize search results")?
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_validation_rejects_ambiguous_parameters() {
        assert!(query_params(&json!([])).is_err());
        assert!(query_params(&json!({"maxResults":0})).is_err());
        assert!(query_params(&json!({"labelIds":[3]})).is_err());
        assert!(query_params(&json!({"fields":"snippet"})).is_err());
        assert_eq!(
            query_params(&json!({"labelIds":["a","b"]})).unwrap().len(),
            2
        );
    }

    #[tokio::test]
    async fn bounded_search_reports_continuation_and_empty_page_is_valid() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/messages")
            .match_query(mockito::Matcher::UrlEncoded(
                "maxResults".into(),
                "1".into(),
            ))
            .with_body(r#"{"messages":[{"id":"one"}],"nextPageToken":"continue"}"#)
            .create_async()
            .await;
        let result = list_message_ids(
            &reqwest::Client::new(),
            "test",
            &format!("{}/messages", server.url()),
            &json!({}),
            1,
            10,
        )
        .await
        .unwrap();
        assert_eq!(result.ids, vec!["one"]);
        assert_eq!(result.next_page_token.as_deref(), Some("continue"));
        mock.assert_async().await;
        let _empty = server
            .mock("GET", "/empty")
            .match_query(mockito::Matcher::Any)
            .with_body("{}")
            .create_async()
            .await;
        let result = list_message_ids(
            &reqwest::Client::new(),
            "test",
            &format!("{}/empty", server.url()),
            &json!({}),
            20,
            10,
        )
        .await
        .unwrap();
        assert!(result.ids.is_empty());
        assert!(result.next_page_token.is_none());
    }
}
