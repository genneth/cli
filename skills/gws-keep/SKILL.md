---
name: gws-keep
description: "Manage Google Keep notes."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws keep --help"
---

# keep (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws keep <resource> <method> [flags]
```

## API Resources

### media

  - `download` — Gets an attachment. To download attachment media via REST requires the alt=media query parameter. Returns a 400 bad request error if attachment media is not available in the requested MIME type.
    - Required path params: name
    - Response type: `Attachment`

### notes

  - `create` — Creates a new note.
    - Request body type: `Note`
    - Response type: `Note`
  - `delete` — Deletes a note. Caller must have the `OWNER` role on the note to delete. Deleting a note removes the resource immediately and cannot be undone. Any collaborators will lose access to the note.
    - Required path params: name
    - Response type: `Empty`
  - `get` — Gets a note.
    - Required path params: name
    - Response type: `Note`
  - `list` — Lists notes. Every list call returns a page of results with `page_size` as the upper bound of returned items. A `page_size` of zero allows the server to choose the upper bound. The ListNotesResponse contains at most `page_size` entries. If there are more things left to list, it provides a `next_page_token` value. (Page tokens are opaque values.) To get the next page of results, copy the result's `next_page_token` into the next request's `page_token`.
    - Response type: `ListNotesResponse`

### notes.permissions

  - `batchCreate` — Creates one or more permissions on the note. Only permissions with the `WRITER` role may be created. If adding any permission fails, then the entire request fails and no changes are made.
    - Required path params: parent
    - Request body type: `BatchCreatePermissionsRequest`
    - Response type: `BatchCreatePermissionsResponse`
  - `batchDelete` — Deletes one or more permissions on the note. The specified entities will immediately lose access. A permission with the `OWNER` role can't be removed. If removing a permission fails, then the entire request fails and no changes are made. Returns a 400 bad request error if a specified permission does not exist on the note.
    - Required path params: parent
    - Request body type: `BatchDeletePermissionsRequest`
    - Response type: `Empty`

## Common Schemas

### Attachment

*Description: An attachment to a note.*

| Field | Type | Description |
|---|---|---|
| `mimeType` | array of string | The MIME types (IANA media types) in which the attachment is available. |
| `name` | string | The resource name; |

### BatchCreatePermissionsRequest

*Description: The request to add one or more permissions on the note. Currently, only the `WRITER` role may be specified. If adding a permission fails, then the entire request fails and no changes are made.*

| Field | Type | Description |
|---|---|---|
| `requests` | array of `CreatePermissionRequest` | The request message specifying the resources to create. |

### BatchCreatePermissionsResponse

*Description: The response for creating permissions on a note.*

| Field | Type | Description |
|---|---|---|
| `permissions` | array of `Permission` | Permissions created. |

### BatchDeletePermissionsRequest

*Description: The request to remove one or more permissions from a note. A permission with the `OWNER` role can't be removed. If removing a permission fails, then the entire request fails and no changes are made. Returns a 400 bad request error if a specified permission does not exist on the note.*

| Field | Type | Description |
|---|---|---|
| `names` | array of string | Required. The names of the permissions to delete. Format: `notes/{note}/permissions/{permission}` |

### Empty

*Description: A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }*

*(No fields)*

### ListNotesResponse

*Description: The response when listing a page of notes.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Next page's `page_token` field. |
| `notes` | array of `Note` | A page of notes. |

### Note

*Description: A single note.*

| Field | Type | Description |
|---|---|---|
| `attachments` | array of `Attachment` | Output only. The attachments attached to this note. |
| `body` | `Section` | The body of the note. |
| `createTime` | string (format: google-datetime) | Output only. When this note was created. |
| `name` | string | Output only. The resource name of this note. See general note on identifiers in KeepService. |
| `permissions` | array of `Permission` | Output only. The list of permissions set on the note. Contains at least one entry for the note owner. |
| `title` | string | The title of the note. Length must be less than 1,000 characters. |
| `trashTime` | string (format: google-datetime) | Output only. When this note was trashed. If `trashed`, the note is eventually deleted. If the note is not trashed, this field is not set (and the trashed field is `false`). |
| `trashed` | boolean | Output only. `true` if this note has been trashed. If trashed, the note is eventually deleted. |
| `updateTime` | string (format: google-datetime) | Output only. When this note was last modified. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws keep --help

# Inspect a method's required params, types, and defaults
gws schema keep.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

