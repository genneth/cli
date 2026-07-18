---
name: gws-forms
description: "Read and write Google Forms."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws forms --help"
---

# forms (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws forms <resource> <method> [flags]
```

## API Resources

### forms

  - `batchUpdate` — Change the form with a batch of updates.
    - Required path params: formId
    - Request body type: `BatchUpdateFormRequest`
    - Response type: `BatchUpdateFormResponse`
  - `create` — Create a new form using the title given in the provided form message in the request. *Important:* Only the form.info.title and form.info.document_title fields are copied to the new form. All other fields including the form description, items and settings are disallowed. To create a new form and add items, you must first call forms.create to create an empty form with a title and (optional) document title, and then call forms.update to add the items.
    - Request body type: `Form`
    - Response type: `Form`
  - `get` — Get a form.
    - Required path params: formId
    - Response type: `Form`
  - `setPublishSettings` — Updates the publish settings of a form. Legacy forms aren't supported because they don't have the `publish_settings` field.
    - Required path params: formId
    - Request body type: `SetPublishSettingsRequest`
    - Response type: `SetPublishSettingsResponse`

### forms.responses

  - `get` — Get one response from the form.
    - Required path params: formId, responseId
    - Response type: `FormResponse`
  - `list` — List a form's responses.
    - Required path params: formId
    - Response type: `ListFormResponsesResponse`

### forms.watches

  - `create` — Create a new watch. If a watch ID is provided, it must be unused. For each invoking project, the per form limit is one watch per Watch.EventType. A watch expires seven days after it is created (see Watch.expire_time).
    - Required path params: formId
    - Request body type: `CreateWatchRequest`
    - Response type: `Watch`
  - `delete` — Delete a watch.
    - Required path params: formId, watchId
    - Response type: `Empty`
  - `list` — Return a list of the watches owned by the invoking project. The maximum number of watches is two: For each invoker, the limit is one for each event type per form.
    - Required path params: formId
    - Response type: `ListWatchesResponse`
  - `renew` — Renew an existing watch for seven days. The state of the watch after renewal is `ACTIVE`, and the `expire_time` is seven days from the renewal. Renewing a watch in an error state (e.g. `SUSPENDED`) succeeds if the error is no longer present, but fail otherwise. After a watch has expired, RenewWatch returns `NOT_FOUND`.
    - Required path params: formId, watchId
    - Request body type: `RenewWatchRequest`
    - Response type: `Watch`

## Common Schemas

### BatchUpdateFormRequest

*Description: A batch of updates to perform on a form. All the specified updates are made or none of them are.*

| Field | Type | Description |
|---|---|---|
| `includeFormInResponse` | boolean | Whether to return an updated version of the model in the response. |
| `requests` | array of `Request` | Required. The update requests of this batch. |
| `writeControl` | `WriteControl` | Provides control over how write requests are executed. |

### BatchUpdateFormResponse

*Description: Response to a BatchUpdateFormRequest.*

| Field | Type | Description |
|---|---|---|
| `form` | `Form` | Based on the bool request field `include_form_in_response`, a form with all applied mutations/updates is returned or not. This may be later than the revision ID created by these changes. |
| `replies` | array of `Response` | The reply of the updates. This maps 1:1 with the update requests, although replies to some requests may be empty. |
| `writeControl` | `WriteControl` | The updated write control after applying the request. |

### CreateWatchRequest

*Description: Create a new watch.*

| Field | Type | Description |
|---|---|---|
| `watch` | `Watch` | Required. The watch object. No ID should be set on this object; use `watch_id` instead. |
| `watchId` | string | The ID to use for the watch. If specified, the ID must not already be in use. If not specified, an ID is generated. This value should be 4-63 characters, and valid characters are /a-z-/. |

### Empty

*Description: A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }*

*(No fields)*

### Form

*Description: A Google Forms document. A form is created in Drive, and deleting a form or changing its access protections is done via the [Drive API](https://developers.google.com/drive/api/v3/about-sdk).*

| Field | Type | Description |
|---|---|---|
| `formId` | string | Output only. The form ID. |
| `info` | `Info` | Required. The title and description of the form. |
| `items` | array of `Item` | Required. A list of the form's items, which can include section headers, questions, embedded media, etc. |
| `linkedSheetId` | string | Output only. The ID of the linked Google Sheet which is accumulating responses from this Form (if such a Sheet exists). |
| `publishSettings` | `PublishSettings` | Output only. The publishing settings for a form. This field isn't set for legacy forms because they don't have the publish_settings field. All newly created forms support publish settings. |
| `responderUri` | string | Output only. The form URI to share with responders. This opens a page that allows the user to submit responses but not edit the questions. |
| `revisionId` | string | Output only. The revision ID of the form. Used in the WriteControl in update requests to identify the revision on which the changes are based. |
| `settings` | `FormSettings` | The form's settings. This must be updated with UpdateSettingsRequest; it is ignored during CreateForm and UpdateFormInfoRequest. |

### FormResponse

*Description: A form response.*

| Field | Type | Description |
|---|---|---|
| `answers` | object | Output only. The actual answers to the questions, keyed by question_id. |
| `createTime` | string (format: google-datetime) | Output only. Timestamp for the first time the response was submitted. |
| `formId` | string | Output only. The form ID. |
| `lastSubmittedTime` | string (format: google-datetime) | Output only. Timestamp for the most recent time the response was submitted. Does not track changes to grades. |
| `respondentEmail` | string | Output only. The email address (if collected) for the respondent. |
| `responseId` | string | Output only. The response ID. |
| `totalScore` | number (format: double) | Output only. The total number of points the respondent received for their submission Only set if the form was a quiz and the response was graded. |

### ListFormResponsesResponse

*Description: Response to a ListFormResponsesRequest.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | If set, there are more responses. To get the next page of responses, provide this as `page_token` in a future request. |
| `responses` | array of `FormResponse` | The returned form responses. Note: The `formId` field is not returned in the `FormResponse` object for list requests. |

### ListWatchesResponse

*Description: The response of a ListWatchesRequest.*

| Field | Type | Description |
|---|---|---|
| `watches` | array of `Watch` | The returned watches. |

### RenewWatchRequest

*Description: Renew an existing Watch for seven days.*

*(No fields)*

### SetPublishSettingsRequest

*Description: Updates the publish settings of a Form.*

| Field | Type | Description |
|---|---|---|
| `publishSettings` | `PublishSettings` | Required. The desired publish settings to apply to the form. |
| `updateMask` | string (format: google-fieldmask) | Optional. The `publish_settings` fields to update. This field mask accepts the following values: * `publish_state`: Updates or replaces all `publish_state` settings. |

### SetPublishSettingsResponse

*Description: The response of a SetPublishSettings request.*

| Field | Type | Description |
|---|---|---|
| `formId` | string | Required. The ID of the Form. This is same as the Form.form_id field. |
| `publishSettings` | `PublishSettings` | The publish settings of the form. |

### Watch

*Description: A watch for events for a form. When the designated event happens, a notification will be published to the specified target. The notification's attributes will include a `formId` key that has the ID of the watched form and an `eventType` key that has the string of the type.*

| Field | Type | Description |
|---|---|---|
| `createTime` | string (format: google-datetime) | Output only. Timestamp of when this was created. |
| `errorType` | string | Output only. The most recent error type for an attempted delivery. To begin watching the form again a call can be made to watches.renew which also clears this error information. |
| `eventType` | string | Required. Which event type to watch for. |
| `expireTime` | string (format: google-datetime) | Output only. Timestamp for when this will expire. Each watches.renew call resets this to seven days in the future. |
| `id` | string | Output only. The ID of this watch. See notes on CreateWatchRequest.watch_id. |
| `state` | string | Output only. The current state of the watch. Additional details about suspended watches can be found by checking the `error_type`. |
| `target` | `WatchTarget` | Required. Where to send the notification. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws forms --help

# Inspect a method's required params, types, and defaults
gws schema forms.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

