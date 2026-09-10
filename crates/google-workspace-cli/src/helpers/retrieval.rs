//! Read-only HTTP and bounded pagination shared by composed retrieval helpers.
use crate::error::GwsError;
use anyhow::Context;
use serde_json::{json, Value};

pub(super) async fn get_json(
    client: &reqwest::Client,
    url: &str,
    token: &str,
    query: &[(&str, &str)],
) -> Result<Value, GwsError> {
    let response =
        crate::client::send_with_retry(|| client.get(url).bearer_auth(token).query(query))
            .await
            .context("Retrieval request failed")?;
    let status = response.status();
    if !status.is_success() {
        return Err(GwsError::Api {
            code: status.as_u16(),
            message: response
                .text()
                .await
                .unwrap_or_else(|_| "Unreadable error response".into()),
            reason: "retrieval_failed".into(),
            enable_url: None,
        });
    }
    let value: Value = response.json().await.context("Invalid retrieval JSON")?;
    if !value.is_object() {
        return Err(anyhow::anyhow!("Expected an API response object").into());
    }
    Ok(value)
}

/// Returns every page or an error. Reaching a safety bound is never an empty success.
pub(super) async fn list_all(
    client: &reqwest::Client,
    url: &str,
    token: &str,
    query: &[(&str, &str)],
    key: &str,
) -> Result<Vec<Value>, GwsError> {
    let mut items = Vec::new();
    let mut next: Option<String> = None;
    let mut seen = std::collections::HashSet::new();
    for _ in 0..100 {
        let mut params = query.to_vec();
        if let Some(ref page) = next {
            params.push(("pageToken", page));
        }
        let value = get_json(client, url, token, &params).await?;
        items.extend(page_items(&value, key)?);
        if items.len() > 100_000 {
            return Err(anyhow::anyhow!(
                "Retrieval exceeds 100000 items; narrow the requested interval"
            )
            .into());
        }
        next = page_token(&value)?;
        match next.as_ref() {
            None => return Ok(items),
            Some(page) => {
                if !seen.insert(page.clone()) {
                    return Err(anyhow::anyhow!("API repeated a pagination token").into());
                }
            }
        }
    }
    Err(anyhow::anyhow!("Retrieval exceeds 100 pages; narrow the requested interval").into())
}

pub(super) fn page_items(value: &Value, key: &str) -> Result<Vec<Value>, GwsError> {
    match value.get(key) {
        None => Ok(Vec::new()),
        Some(Value::Array(items)) => Ok(items.clone()),
        Some(_) => Err(anyhow::anyhow!("Expected {key} array in API response").into()),
    }
}

pub(super) fn page_token(value: &Value) -> Result<Option<String>, GwsError> {
    match value.get("nextPageToken") {
        None => Ok(None),
        Some(Value::String(s)) if s.is_empty() => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(_) => Err(anyhow::anyhow!("Invalid nextPageToken in API response").into()),
    }
}

/// Keep source identifiers alongside display fields so results can be acted upon.
pub(super) fn event_summary(event: &Value, calendar_id: &str) -> Value {
    json!({"id":event["id"], "calendarId":calendar_id,
        "summary":event.get("summary").and_then(Value::as_str).unwrap_or("(No title)"),
        "start":event["start"].get("dateTime").or_else(|| event["start"].get("date")).cloned().unwrap_or(Value::Null),
        "end":event["end"].get("dateTime").or_else(|| event["end"].get("date")).cloned().unwrap_or(Value::Null),
        "location":event["location"], "htmlLink":event["htmlLink"]})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn follows_pages_and_preserves_every_item() {
        let mut server = mockito::Server::new_async().await;
        let first = server
            .mock("GET", "/items")
            .match_query(mockito::Matcher::Missing)
            .with_body(r#"{"items":[{"id":"a"}],"nextPageToken":"two"}"#)
            .create_async()
            .await;
        let second = server
            .mock("GET", "/items")
            .match_query(mockito::Matcher::UrlEncoded(
                "pageToken".into(),
                "two".into(),
            ))
            .with_body(r#"{"items":[{"id":"b"}]}"#)
            .create_async()
            .await;
        let items = list_all(
            &reqwest::Client::new(),
            &format!("{}/items", server.url()),
            "test",
            &[],
            "items",
        )
        .await
        .unwrap();
        assert_eq!(items, vec![json!({"id":"a"}), json!({"id":"b"})]);
        first.assert_async().await;
        second.assert_async().await;
    }

    #[tokio::test]
    async fn failed_second_page_is_an_error_not_partial_success() {
        let mut server = mockito::Server::new_async().await;
        let _a = server
            .mock("GET", "/items")
            .match_query(mockito::Matcher::Missing)
            .with_body(r#"{"items":[1],"nextPageToken":"two"}"#)
            .create_async()
            .await;
        let _b = server
            .mock("GET", "/items")
            .match_query(mockito::Matcher::UrlEncoded(
                "pageToken".into(),
                "two".into(),
            ))
            .with_status(403)
            .with_body("denied")
            .create_async()
            .await;
        assert!(list_all(
            &reqwest::Client::new(),
            &format!("{}/items", server.url()),
            "test",
            &[],
            "items"
        )
        .await
        .is_err());
    }

    #[test]
    fn empty_and_invalid_pages_are_distinct() {
        assert!(page_items(&json!({}), "items").unwrap().is_empty());
        assert!(page_items(&json!({"items":null}), "items").is_err());
        assert!(page_token(&json!({"nextPageToken":5})).is_err());
    }

    #[test]
    fn summaries_keep_event_and_calendar_ids() {
        let summary = event_summary(&json!({"id":"e", "start":{"date":"2026-09-10"}}), "c");
        assert_eq!(summary["id"], "e");
        assert_eq!(summary["calendarId"], "c");
        assert_eq!(summary["start"], "2026-09-10");
    }
}
