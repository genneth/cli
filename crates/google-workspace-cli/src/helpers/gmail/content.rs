//! Shared message retrieval. The API envelope owns Gmail IDs; RFC headers remain separate.
use super::*;
use crate::helpers::retrieval;

pub(super) const MESSAGES_URL: &str = "https://gmail.googleapis.com/gmail/v1/users/me/messages";

#[derive(Serialize)]
pub(super) struct ReadMessage {
    pub id: String,
    pub rfc_message_id: String,
    pub label_ids: Vec<String>,
    pub internal_date: Option<String>,
    pub snippet: String,
    pub attachments: Vec<OriginalPart>,
    #[serde(flatten)]
    pub message: OriginalMessage,
}

fn parse_read_message(raw: &Value) -> Result<ReadMessage, GwsError> {
    let id = raw
        .get("id")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .context("Message response missing Gmail id")?
        .to_owned();
    if !raw.get("payload").is_some_and(Value::is_object) {
        return Err(anyhow::anyhow!("Message {id} has no full MIME payload").into());
    }
    let message = parse_message(raw)?;
    Ok(ReadMessage {
        id,
        rfc_message_id: message.message_id.clone(),
        label_ids: raw
            .get("labelIds")
            .map(|v| serde_json::from_value(v.clone()))
            .transpose()
            .context("Invalid labelIds")?
            .unwrap_or_default(),
        internal_date: raw
            .get("internalDate")
            .and_then(Value::as_str)
            .map(str::to_owned),
        snippet: raw
            .get("snippet")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned(),
        attachments: message.parts.clone(),
        message,
    })
}

pub(super) async fn fetch_message(
    client: &reqwest::Client,
    token: &str,
    id: &str,
) -> Result<ReadMessage, GwsError> {
    let raw = retrieval::get_json(
        client,
        &format!(
            "{MESSAGES_URL}/{}",
            crate::validate::encode_path_segment(id)
        ),
        token,
        &[("format", "full")],
    )
    .await?;
    if raw.get("id").and_then(Value::as_str) != Some(id) {
        return Err(anyhow::anyhow!("Gmail returned a different message id").into());
    }
    hydrate_message(client, token, raw).await
}

/// Describe just the external body fetches; attached messages are opaque files.
fn body_requests(part: &Value, path: &str, requests: &mut Vec<(String, String)>) {
    if is_body_part(part) {
        if let Some(id) = part.pointer("/body/attachmentId").and_then(Value::as_str) {
            if part.pointer("/body/data").is_none() {
                requests.push((format!("{path}/body"), id.to_owned()));
            }
        }
    } else if part
        .get("mimeType")
        .and_then(Value::as_str)
        .is_some_and(|s| s.starts_with("multipart/"))
        && part
            .get("filename")
            .and_then(Value::as_str)
            .unwrap_or("")
            .is_empty()
        && part.pointer("/body/attachmentId").is_none()
    {
        if let Some(children) = part.get("parts").and_then(Value::as_array) {
            for (i, child) in children.iter().enumerate() {
                body_requests(child, &format!("{path}/parts/{i}"), requests);
            }
        }
    }
}

pub(super) async fn hydrate_message(
    client: &reqwest::Client,
    token: &str,
    mut raw: Value,
) -> Result<ReadMessage, GwsError> {
    hydrate_message_at(client, token, &mut raw, MESSAGES_URL).await
}

async fn hydrate_message_at(
    client: &reqwest::Client,
    token: &str,
    raw: &mut Value,
    messages_url: &str,
) -> Result<ReadMessage, GwsError> {
    let id = raw
        .get("id")
        .and_then(Value::as_str)
        .context("Missing Gmail message id")?
        .to_owned();
    let mut requests = Vec::new();
    body_requests(&raw["payload"], "/payload", &mut requests);
    for (path, attachment_id) in requests {
        let bytes = fetch_attachment_at(
            client,
            token,
            &format!(
                "{}/{}/attachments/{}",
                messages_url,
                crate::validate::encode_path_segment(&id),
                crate::validate::encode_path_segment(&attachment_id)
            ),
        )
        .await?;
        let body = raw
            .pointer_mut(&path)
            .context("MIME body disappeared during hydration")?;
        body["data"] = json!(URL_SAFE.encode(bytes));
    }
    parse_read_message(raw)
}

pub(super) async fn fetch_thread(
    client: &reqwest::Client,
    token: &str,
    id: &str,
) -> Result<Vec<ReadMessage>, GwsError> {
    let raw = retrieval::get_json(
        client,
        &format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/threads/{}",
            crate::validate::encode_path_segment(id)
        ),
        token,
        &[("format", "full")],
    )
    .await?;
    let messages = raw
        .get("messages")
        .and_then(Value::as_array)
        .context("Thread response missing messages")?
        .clone();
    use futures_util::stream::{self, StreamExt, TryStreamExt};
    stream::iter(messages)
        .map(|raw| hydrate_message(client, token, raw))
        .buffered(5)
        .try_collect()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn read_does_not_require_reply_headers_and_keeps_gmail_id() {
        let message = parse_read_message(&json!({"id":"gmail-id", "labelIds":["INBOX"],
            "snippet":"not the body", "payload":{"mimeType":"text/plain", "body":{"data":URL_SAFE.encode("hello")}}})).unwrap();
        let value = serde_json::to_value(message).unwrap();
        assert_eq!(value["id"], "gmail-id");
        assert_eq!(value["body_text"], "hello");
        assert_eq!(value["rfc_message_id"], "");
        assert_eq!(value["attachments"], json!([]));
    }

    #[test]
    fn json_exposes_attachment_metadata_but_not_bytes() {
        let message = parse_read_message(&json!({"id":"g", "payload":{
            "mimeType":"application/pdf", "filename":"a.pdf", "body":{"data":URL_SAFE.encode("secret bytes"),"size":12}}})).unwrap();
        let value = serde_json::to_value(message).unwrap();
        assert_eq!(value["attachments"][0]["filename"], "a.pdf");
        assert!(value["attachments"][0].get("data").is_none());
        assert_eq!(value["body_text"], "");
    }

    #[test]
    fn external_text_is_fetched_but_attached_messages_are_not_walked() {
        let payload = json!({"mimeType":"multipart/mixed", "parts":[
            {"mimeType":"text/plain","body":{"attachmentId":"body-id","size":123}},
            {"mimeType":"message/rfc822","filename":"a.eml","body":{"attachmentId":"file"},
             "parts":[{"mimeType":"text/plain","body":{"attachmentId":"nested"}}]}
        ]});
        let mut requests = vec![];
        body_requests(&payload, "/payload", &mut requests);
        assert_eq!(
            requests,
            vec![("/payload/parts/0/body".into(), "body-id".into())]
        );
    }

    #[test]
    fn charset_and_invalid_body_are_handled_explicitly() {
        let mut raw = json!({"id":"g","payload":{"mimeType":"text/plain",
            "headers":[{"name":"Content-Type","value":"text/plain; charset=windows-1252"}],
            "body":{"data":URL_SAFE.encode([0x63,0x61,0x66,0xe9])}}});
        assert_eq!(parse_read_message(&raw).unwrap().message.body_text, "café");
        raw["payload"]["body"]["data"] = json!("%%%bad");
        assert!(parse_read_message(&raw).is_err());
    }

    #[tokio::test]
    async fn external_body_is_decoded_as_text_and_fetch_failure_propagates() {
        let mut server = mockito::Server::new_async().await;
        let payload = json!({"id":"g", "payload":{"mimeType":"text/plain",
            "body":{"attachmentId":"body","size":11}}});
        let mock = server
            .mock("GET", "/g/attachments/body")
            .with_body(json!({"data":URL_SAFE.encode("entire text")}).to_string())
            .create_async()
            .await;
        let message = hydrate_message_at(
            &reqwest::Client::new(),
            "test",
            &mut payload.clone(),
            &server.url(),
        )
        .await
        .unwrap();
        assert_eq!(message.message.body_text, "entire text");
        assert!(message.attachments.is_empty());
        mock.assert_async().await;
        let _failure = server
            .mock("GET", "/g/attachments/body")
            .with_status(403)
            .create_async()
            .await;
        assert!(hydrate_message_at(
            &reqwest::Client::new(),
            "test",
            &mut payload.clone(),
            &server.url()
        )
        .await
        .is_err());
    }

    #[test]
    fn lowercase_headers_keep_distinct_rfc_and_gmail_ids() {
        let message = parse_read_message(&json!({"id":"gmail", "payload":{"mimeType":"text/plain",
            "headers":[{"name":"message-id","value":"<rfc@example.com>"}],"body":{"data":""}}}))
        .unwrap();
        assert_eq!(message.id, "gmail");
        assert_eq!(message.rfc_message_id, "rfc@example.com");
    }

    proptest! {
        #[test]
        fn padded_and_unpadded_base64_preserve_bytes(bytes in proptest::collection::vec(any::<u8>(), 0..2048)) {
            let encoded = URL_SAFE.encode(&bytes);
            prop_assert_eq!(decode_binary(&encoded).unwrap(), bytes.clone());
            prop_assert_eq!(decode_binary(encoded.trim_end_matches('=')).unwrap(), bytes);
        }

        #[test]
        fn remote_names_always_stay_one_component(raw in ".{0,100}") {
            let name = sanitize_remote_filename(&raw, 0, "application/octet-stream");
            prop_assert_eq!(std::path::Path::new(&name).components().count(), 1);
            prop_assert!(!name.contains(['/', '\\']));
            prop_assert!(!matches!(name.as_str(), "." | ".."));
        }
    }
}
