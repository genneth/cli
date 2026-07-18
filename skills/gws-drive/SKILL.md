---
name: gws-drive
description: "Google Drive: Manage files, folders, and shared drives."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws drive --help"
---

# drive (v3)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws drive <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+upload`](../gws-drive-upload/SKILL.md) | Upload a file with automatic metadata |

## API Resources

### about

  - `get` — Gets information about the user, the user's Drive, and system capabilities. For more information, see [Return user info](https://developers.google.com/workspace/drive/api/guides/user-info). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).
    - Response type: `About`

### accessproposals

  - `get` — Retrieves an access proposal by ID. For more information, see [Manage pending access proposals](https://developers.google.com/workspace/drive/api/guides/pending-access).
    - Required path params: fileId, proposalId
    - Response type: `AccessProposal`
  - `list` — List the access proposals on a file. For more information, see [Manage pending access proposals](https://developers.google.com/workspace/drive/api/guides/pending-access). Note: Only approvers are able to list access proposals on a file. If the user isn't an approver, a 403 error is returned.
    - Required path params: fileId
    - Response type: `ListAccessProposalsResponse`
  - `resolve` — Approves or denies an access proposal. For more information, see [Manage pending access proposals](https://developers.google.com/workspace/drive/api/guides/pending-access).
    - Required path params: fileId, proposalId
    - Request body type: `ResolveAccessProposalRequest`

### approvals

  - `approve` — Approves an approval. For more information, see [Manage approvals](https://developers.google.com/workspace/drive/api/guides/approvals). This is used to update the ReviewerResponse of the requesting user with a Response of `APPROVED`. If this is the last required reviewer response, this also completes the approval and sets the approval Status to `APPROVED`.
    - Required path params: approvalId, fileId
    - Request body type: `ApproveApprovalRequest`
    - Response type: `Approval`
  - `cancel` — Cancels an approval. For more information, see [Manage approvals](https://developers.google.com/workspace/drive/api/guides/approvals). Updates the approval Status to `CANCELLED`. This can be called by any user with the `writer` permission on the file while the approval Status is `IN_PROGRESS`.
    - Required path params: approvalId, fileId
    - Request body type: `CancelApprovalRequest`
    - Response type: `Approval`
  - `comment` — Comments on an approval. For more information, see [Manage approvals](https://developers.google.com/workspace/drive/api/guides/approvals). This sends a notification to both the initiator and the reviewers. Additionally, a message is also added to the approval activity log.
    - Required path params: approvalId, fileId
    - Request body type: `CommentApprovalRequest`
    - Response type: `Approval`
  - `decline` — Declines an approval. For more information, see [Manage approvals](https://developers.google.com/workspace/drive/api/guides/approvals). This is used to update the ReviewerResponse of the requesting user with a Response of `DECLINED`. This also completes the approval and sets the approval Status to `DECLINED`.
    - Required path params: approvalId, fileId
    - Request body type: `DeclineApprovalRequest`
    - Response type: `Approval`
  - `get` — Gets an approval by ID. For more information, see [Manage approvals](https://developers.google.com/workspace/drive/api/guides/approvals).
    - Required path params: approvalId, fileId
    - Response type: `Approval`
  - `list` — Lists the approvals on a file. For more information, see [Manage approvals](https://developers.google.com/workspace/drive/api/guides/approvals).
    - Required path params: fileId
    - Response type: `ApprovalList`
  - `reassign` — Reassigns the reviewers on an approval. For more information, see [Manage approvals](https://developers.google.com/workspace/drive/api/guides/approvals). Adds or replaces reviewers in the ReviewerResponse of the approval. This can be called by any user with the `writer` permission on the file while the approval Status is `IN_PROGRESS` and the Response for the reviewer being reassigned is `NO_RESPONSE`.
    - Required path params: approvalId, fileId
    - Request body type: `ReassignApprovalRequest`
    - Response type: `Approval`
  - `start` — Starts an approval on a file. For more information, see [Manage approvals](https://developers.google.com/workspace/drive/api/guides/approvals).
    - Required path params: fileId
    - Request body type: `StartApprovalRequest`
    - Response type: `Approval`

### apps

  - `get` — Gets a specific app. For more information, see [Return user info](https://developers.google.com/workspace/drive/api/guides/user-info).
    - Required path params: appId
    - Response type: `App`
  - `list` — Lists a user's installed apps. For more information, see [Return user info](https://developers.google.com/workspace/drive/api/guides/user-info).
    - Response type: `AppList`

### changes

  - `getStartPageToken` — Gets the starting pageToken for listing future changes. For more information, see [Retrieve changes](https://developers.google.com/workspace/drive/api/guides/manage-changes).
    - Response type: `StartPageToken`
  - `list` — Lists the changes for a user or shared drive. For more information, see [Retrieve changes](https://developers.google.com/workspace/drive/api/guides/manage-changes).
    - Required query params: pageToken
    - Response type: `ChangeList`
  - `watch` — Subscribes to changes for a user. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).
    - Required query params: pageToken
    - Request body type: `Channel`
    - Response type: `Channel`

### channels

  - `stop` — Stops watching resources through this channel. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).
    - Request body type: `Channel`

### comments

  - `create` — Creates a comment on a file. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).
    - Required path params: fileId
    - Request body type: `Comment`
    - Response type: `Comment`
  - `delete` — Deletes a comment. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).
    - Required path params: commentId, fileId
  - `get` — Gets a comment by ID. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).
    - Required path params: commentId, fileId
    - Response type: `Comment`
  - `list` — Lists a file's comments. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).
    - Required path params: fileId
    - Response type: `CommentList`
  - `update` — Updates a comment with patch semantics. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).
    - Required path params: commentId, fileId
    - Request body type: `Comment`
    - Response type: `Comment`

### drives

  - `create` — Creates a shared drive. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).
    - Required query params: requestId
    - Request body type: `Drive`
    - Response type: `Drive`
  - `get` — Gets a shared drive's metadata by ID. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).
    - Required path params: driveId
    - Response type: `Drive`
  - `hide` — Hides a shared drive from the default view. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).
    - Required path params: driveId
    - Response type: `Drive`
  - `list` — Lists the user's shared drives. This method accepts the `q` parameter, which is a search query combining one or more search terms. For more information, see the [Search for shared drives](https://developers.google.com/workspace/drive/api/guides/search-shareddrives) guide.
    - Response type: `DriveList`
  - `unhide` — Restores a shared drive to the default view. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).
    - Required path params: driveId
    - Response type: `Drive`
  - `update` — Updates the metadata for a shared drive. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).
    - Required path params: driveId
    - Request body type: `Drive`
    - Response type: `Drive`

### files

  - `copy` — Creates a copy of a file and applies any requested updates with patch semantics. For more information, see [Create and manage files](https://developers.google.com/workspace/drive/api/guides/create-file).
    - Required path params: fileId
    - Request body type: `File`
    - Response type: `File`
  - `create` — Creates a file. For more information, see [Create and manage files](https://developers.google.com/workspace/drive/api/guides/create-file). This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:* `*/*` (Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded.
    - Request body type: `File`
    - Response type: `File`
  - `download` — Downloads the content of a file. For more information, see [Download and export files](https://developers.google.com/workspace/drive/api/guides/manage-downloads). Operations are valid for 24 hours from the time of creation.
    - Required path params: fileId
    - Response type: `Operation`
  - `export` — Exports a Google Workspace document to the requested MIME type and returns exported byte content. For more information, see [Download and export files](https://developers.google.com/workspace/drive/api/guides/manage-downloads). Note that the exported content is limited to 10 MB.
    - Required path params: fileId
    - Required query params: mimeType
  - `generateCseToken` — Generates a CSE token which can be used to create or update CSE files.
    - Response type: `GenerateCseTokenResponse`
  - `generateIds` — Generates a set of file IDs which can be provided in create or copy requests. For more information, see [Create and manage files](https://developers.google.com/workspace/drive/api/guides/create-file).
    - Response type: `GeneratedIds`
  - `get` — Gets a file's metadata or content by ID. For more information, see [Search for files and folders](https://developers.google.com/workspace/drive/api/guides/search-files). If you provide the URL parameter `alt=media`, then the response includes the file contents in the response body. Downloading content with `alt=media` only works if the file is stored in Drive.
    - Required path params: fileId
    - Response type: `File`
  - `list` — Lists the user's files. For more information, see [Search for files and folders](https://developers.google.com/workspace/drive/api/guides/search-files). This method accepts the `q` parameter, which is a search query combining one or more search terms. This method returns *all* files by default, including trashed files. If you don't want trashed files to appear in the list, use the `trashed=false` query parameter to remove trashed files from the results.
    - Response type: `FileList`
  - `listLabels` — Lists the labels on a file. For more information, see [List labels on a file](https://developers.google.com/workspace/drive/api/guides/list-labels).
    - Required path params: fileId
    - Response type: `LabelList`
  - `modifyLabels` — Modifies the set of labels applied to a file. For more information, see [Set a label field on a file](https://developers.google.com/workspace/drive/api/guides/set-label). Returns a list of the labels that were added or modified.
    - Required path params: fileId
    - Request body type: `ModifyLabelsRequest`
    - Response type: `ModifyLabelsResponse`
  - `update` — Updates a file's metadata, content, or both. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might be changed automatically, such as `modifiedDate`. This method supports patch semantics. This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:* `*/*` (Specify a valid MIME type, rather than the literal `*/*` value.
    - Required path params: fileId
    - Request body type: `File`
    - Response type: `File`
  - `watch` — Subscribes to changes to a file. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).
    - Required path params: fileId
    - Request body type: `Channel`
    - Response type: `Channel`

### operations

  - `get` — Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    - Required path params: name
    - Response type: `Operation`

### permissions

  - `create` — Creates a permission for a file or shared drive. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing). **Warning:** Concurrent permissions operations on the same file aren't supported; only the last update is applied.
    - Required path params: fileId
    - Request body type: `Permission`
    - Response type: `Permission`
  - `delete` — Deletes a permission. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing). **Warning:** Concurrent permissions operations on the same file aren't supported; only the last update is applied.
    - Required path params: fileId, permissionId
  - `get` — Gets a permission by ID. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing).
    - Required path params: fileId, permissionId
    - Response type: `Permission`
  - `list` — Lists a file's or shared drive's permissions. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing).
    - Required path params: fileId
    - Response type: `PermissionList`
  - `update` — Updates a permission with patch semantics. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing). **Warning:** Concurrent permissions operations on the same file aren't supported; only the last update is applied.
    - Required path params: fileId, permissionId
    - Request body type: `Permission`
    - Response type: `Permission`

### replies

  - `create` — Creates a reply to a comment. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).
    - Required path params: commentId, fileId
    - Request body type: `Reply`
    - Response type: `Reply`
  - `delete` — Deletes a reply. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).
    - Required path params: commentId, fileId, replyId
  - `get` — Gets a reply by ID. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).
    - Required path params: commentId, fileId, replyId
    - Response type: `Reply`
  - `list` — Lists a comment's replies. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).
    - Required path params: commentId, fileId
    - Response type: `ReplyList`
  - `update` — Updates a reply with patch semantics. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).
    - Required path params: commentId, fileId, replyId
    - Request body type: `Reply`
    - Response type: `Reply`

### revisions

  - `delete` — Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted. For more information, see [Manage file revisions](https://developers.google.com/drive/api/guides/manage-revisions).
    - Required path params: fileId, revisionId
  - `get` — Gets a revision's metadata or content by ID. For more information, see [Manage file revisions](https://developers.google.com/workspace/drive/api/guides/manage-revisions).
    - Required path params: fileId, revisionId
    - Response type: `Revision`
  - `list` — Lists a file's revisions. For more information, see [Manage file revisions](https://developers.google.com/workspace/drive/api/guides/manage-revisions). **Important:** The list of revisions returned by this method might be incomplete for files with a large revision history, including frequently edited Google Docs, Sheets, and Slides. Older revisions might be omitted from the response, meaning the first revision returned may not be the oldest existing revision.
    - Required path params: fileId
    - Response type: `RevisionList`
  - `update` — Updates a revision with patch semantics. For more information, see [Manage file revisions](https://developers.google.com/workspace/drive/api/guides/manage-revisions).
    - Required path params: fileId, revisionId
    - Request body type: `Revision`
    - Response type: `Revision`

### teamdrives

  - `create` — Deprecated: Use `drives.create` instead.
    - Required query params: requestId
    - Request body type: `TeamDrive`
    - Response type: `TeamDrive`
  - `get` — Deprecated: Use `drives.get` instead.
    - Required path params: teamDriveId
    - Response type: `TeamDrive`
  - `list` — Deprecated: Use `drives.list` instead.
    - Response type: `TeamDriveList`
  - `update` — Deprecated: Use `drives.update` instead.
    - Required path params: teamDriveId
    - Request body type: `TeamDrive`
    - Response type: `TeamDrive`

## Common Schemas

### About

*Description: Information about the user, the user's Drive, and system capabilities.*

| Field | Type | Description |
|---|---|---|
| `appInstalled` | boolean | Whether the user has installed the requesting app. |
| `canCreateDrives` | boolean | Whether the user can create shared drives. |
| `canCreateTeamDrives` | boolean | Deprecated: Use `canCreateDrives` instead. |
| `driveThemes` | array of object | A list of themes that are supported for shared drives. |
| `exportFormats` | object | A map of source MIME type to possible targets for all supported exports. |
| `folderColorPalette` | array of string | The currently supported folder colors as RGB hex strings. |
| `importFormats` | object | A map of source MIME type to possible targets for all supported imports. |
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#about"`. |
| `maxImportSizes` | object | A map of maximum import sizes by MIME type, in bytes. |
| `maxUploadSize` | string (format: int64) | The maximum upload size in bytes. |
| `storageQuota` | object | The user's storage quota limits and usage. |
| `teamDriveThemes` | array of object | Deprecated: Use `driveThemes` instead. |
| `user` | `User` | The authenticated user. |

### AccessProposal

*Description: Manage outstanding access proposals on a file.*

| Field | Type | Description |
|---|---|---|
| `createTime` | string (format: google-datetime) | The creation time. |
| `fileId` | string | The file ID that the proposal for access is on. |
| `proposalId` | string | The ID of the access proposal. |
| `recipientEmailAddress` | string | The email address of the user that will receive permissions, if accepted. |
| `requestMessage` | string | The message that the requester added to the proposal. |
| `requesterEmailAddress` | string | The email address of the requesting user. |
| `rolesAndViews` | array of `AccessProposalRoleAndView` | A wrapper for the role and view of an access proposal. For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). |

### App

*Description: The `apps` resource provides a list of apps that a user has installed, with information about each app's supported MIME types, file extensions, and other details. Some resource methods (such as `apps.get`) require an `appId`.*

| Field | Type | Description |
|---|---|---|
| `authorized` | boolean | Whether the app is authorized to access data on the user's Drive. |
| `createInFolderTemplate` | string | The template URL to create a file with this app in a given folder. The template contains the {folderId} to be replaced by the folder ID house the new file. |
| `createUrl` | string | The URL to create a file with this app. |
| `hasDriveWideScope` | boolean | Whether the app has Drive-wide scope. An app with Drive-wide scope can access all files in the user's Drive. |
| `icons` | array of `AppIcons` | The various icons for the app. |
| `id` | string | The ID of the app. |
| `installed` | boolean | Whether the app is installed. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string "drive#app". |
| `longDescription` | string | A long description of the app. |
| `name` | string | The name of the app. |
| `objectType` | string | The type of object this app creates such as a Chart. If empty, the app name should be used instead. |
| `openUrlTemplate` | string | The template URL for opening files with this app. The template contains {ids} or {exportIds} to be replaced by the actual file IDs. For more information, see Open Files for the full documentation. |
| `primaryFileExtensions` | array of string | The list of primary file extensions. |
| `primaryMimeTypes` | array of string | The list of primary MIME types. |
| `productId` | string | The ID of the product listing for this app. |
| `productUrl` | string | A link to the product listing for this app. |
| `secondaryFileExtensions` | array of string | The list of secondary file extensions. |
| `secondaryMimeTypes` | array of string | The list of secondary MIME types. |
| `shortDescription` | string | A short description of the app. |
| `supportsCreate` | boolean | Whether this app supports creating objects. |
| `supportsImport` | boolean | Whether this app supports importing from Google Docs. |
| `supportsMultiOpen` | boolean | Whether this app supports opening more than one file. |
| `supportsOfflineCreate` | boolean | Whether this app supports creating files when offline. |
| `useByDefault` | boolean | Whether the app is selected as the default handler for the types it supports. |

### AppList

*Description: A list of third-party applications which the user has installed or given access to Google Drive.*

| Field | Type | Description |
|---|---|---|
| `defaultAppIds` | array of string | The list of app IDs that the user has specified to use by default. The list is in reverse-priority order (lowest to highest). |
| `items` | array of `App` | The list of apps. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string "drive#appList". |
| `selfLink` | string | A link back to this list. |

### Approval

*Description: Metadata for an approval. An approval is a review or approve process for a Drive item.*

| Field | Type | Description |
|---|---|---|
| `approvalId` | string | The approval ID. |
| `completeTime` | string (format: google-datetime) | Output only. The time the approval was completed. |
| `createTime` | string (format: google-datetime) | Output only. The time the approval was created. |
| `dueTime` | string (format: google-datetime) | The time that the approval is due. |
| `fileContentChangeBehavior` | string | Output only. The behavior of the approval when the file content changes. |
| `initiator` | `User` | The user that requested the approval. |
| `kind` | string | This is always drive#approval. |
| `modifyTime` | string (format: google-datetime) | Output only. The most recent time the approval was modified. |
| `reviewerResponses` | array of `ReviewerResponse` | The responses made on the approval by reviewers. |
| `status` | string | Output only. The status of the approval at the time this resource was requested. |
| `targetFileId` | string | Target file id of the approval. |

### ApprovalList

*Description: The response of an approvals list request.*

| Field | Type | Description |
|---|---|---|
| `items` | array of `Approval` | The list of approvals. If `nextPageToken` is populated, then this list may be incomplete and an additional page of results should be fetched. |
| `kind` | string | This is always drive#approvalList |
| `nextPageToken` | string | The page token for the next page of approvals. This is absent if the end of the approvals list has been reached. |

### ApproveApprovalRequest

*Description: Request for approving an approval as a reviewer.*

| Field | Type | Description |
|---|---|---|
| `message` | string | Optional. A message to accompany the reviewer response on the approval. This message is included in notifications for the action and in the approval activity log. |

### CancelApprovalRequest

*Description: Request for cancelling an approval as an initiator.*

| Field | Type | Description |
|---|---|---|
| `message` | string | Optional. A message to accompany the cancellation of the approval. This message is included in notifications for the action and in the approval activity log. |

### ChangeList

*Description: A list of changes for a user.*

| Field | Type | Description |
|---|---|---|
| `changes` | array of `Change` | The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. |
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#changeList"`. |
| `newStartPageToken` | string | The starting page token for future changes. This will be present only if the end of the current changes list has been reached. The page token doesn't expire. |
| `nextPageToken` | string | The page token for the next page of changes. This will be absent if the end of the changes list has been reached. The page token doesn't expire. |

### Channel

*Description: A notification channel used to watch for resource changes.*

| Field | Type | Description |
|---|---|---|
| `address` | string | The address where notifications are delivered for this channel. |
| `expiration` | string (format: int64) | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `id` | string | A UUID or similar unique string that identifies this channel. |
| `kind` | string | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `params` | object | Additional parameters controlling delivery channel behavior. Optional. |
| `payload` | boolean | A Boolean value to indicate whether payload is wanted. Optional. |
| `resourceId` | string | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `resourceUri` | string | A version-specific identifier for the watched resource. |
| `token` | string | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | string | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |

### Comment

*Description: A comment on a file. Some resource methods (such as `comments.update`) require a `commentId`. Use the `comments.list` method to retrieve the ID for a comment in a file.*

| Field | Type | Description |
|---|---|---|
| `anchor` | string | A region of the document represented as a JSON string. |
| `assigneeEmailAddress` | string | Output only. The email address of the user assigned to this comment. If no user is assigned, the field is unset. |
| `author` | `User` | Output only. The author of the comment. The author's email address and permission ID will not be populated. |
| `content` | string | The plain text content of the comment. This field is used for setting the content, while `htmlContent` should be displayed. |
| `createdTime` | string (format: date-time) | The time at which the comment was created (RFC 3339 date-time). |
| `deleted` | boolean | Output only. Whether the comment has been deleted. A deleted comment has no content. |
| `htmlContent` | string | Output only. The content of the comment with HTML formatting. |
| `id` | string | Output only. The ID of the comment. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#comment"`. |
| `mentionedEmailAddresses` | array of string | Output only. A list of email addresses for users mentioned in this comment. If no users are mentioned, the list is empty. |
| `modifiedTime` | string (format: date-time) | The last time the comment or any of its replies was modified (RFC 3339 date-time). |
| `quotedFileContent` | object | The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment. |
| `replies` | array of `Reply` | Output only. The full list of replies to the comment in chronological order. |
| `resolved` | boolean | Output only. Whether the comment has been resolved by one of its replies. |

### CommentApprovalRequest

*Description: Request for commenting on an approval.*

| Field | Type | Description |
|---|---|---|
| `message` | string | Required. A message to comment on the approval. This message is included in notifications for the action and in the approval activity log. |

### CommentList

*Description: A list of comments on a file.*

| Field | Type | Description |
|---|---|---|
| `comments` | array of `Comment` | The list of comments. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. |
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#commentList"`. |
| `nextPageToken` | string | The page token for the next page of comments. This will be absent if the end of the comments list has been reached. |

### DeclineApprovalRequest

*Description: Request for declining an approval as a reviewer.*

| Field | Type | Description |
|---|---|---|
| `message` | string | Optional. A message to accompany the reviewer response on the approval. This message is included in notifications for the action and in the approval activity log. |

### Drive

*Description: Representation of a shared drive. Some resource methods (such as `drives.update`) require a `driveId`. Use the `drives.list` method to retrieve the ID for a shared drive.*

| Field | Type | Description |
|---|---|---|
| `backgroundImageFile` | object | An image file and cropping parameters from which a background image for this shared drive is set. |
| `backgroundImageLink` | string | Output only. A short-lived link to this shared drive's background image. |
| `capabilities` | object | Output only. Capabilities the current user has on this shared drive. |
| `colorRgb` | string | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `createdTime` | string (format: date-time) | The time at which the shared drive was created (RFC 3339 date-time). |
| `hidden` | boolean | Whether the shared drive is hidden from default view. |
| `id` | string | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#drive"`. |
| `name` | string | The name of this shared drive. |
| `orgUnitId` | string | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `restrictions` | object | A set of restrictions that apply to this shared drive or items inside this shared drive. Note that restrictions can't be set when creating a shared drive. |
| `themeId` | string | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. |

### DriveList

*Description: A list of shared drives.*

| Field | Type | Description |
|---|---|---|
| `drives` | array of `Drive` | The list of shared drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. |
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#driveList"`. |
| `nextPageToken` | string | The page token for the next page of shared drives. This will be absent if the end of the list has been reached. |

### File

*Description: The metadata for a file. Some resource methods (such as `files.update`) require a `fileId`. Use the `files.list` method to retrieve the ID for a file.*

| Field | Type | Description |
|---|---|---|
| `appProperties` | object | A collection of arbitrary key-value pairs which are private to the requesting app.
Entries with null values are cleared in update and copy requests. |
| `capabilities` | object | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. |
| `clientEncryptionDetails` | `ClientEncryptionDetails` | Client Side Encryption related details. |
| `contentHints` | object | Additional information about the content of the file. These fields are never populated in responses. |
| `contentRestrictions` | array of `ContentRestriction` | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |
| `copyRequiresWriterPermission` | boolean | Whether the options to copy, print, or download this file should be disabled for readers and commenters. |
| `createdTime` | string (format: date-time) | The time at which the file was created (RFC 3339 date-time). |
| `description` | string | A short description of the file. |
| `downloadRestrictions` | `DownloadRestrictionsMetadata` | Download restrictions applied on the file. |
| `driveId` | string | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `explicitlyTrashed` | boolean | Output only. Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder. |
| `exportLinks` | object | Output only. Links for exporting Docs Editors files to specific formats. |
| `fileExtension` | string | Output only. The final component of `fullFileExtension`. This is only available for files with binary content in Google Drive. |
| `folderColorRgb` | string | The color for a folder or a shortcut to a folder as an RGB hex string. |
| `fullFileExtension` | string | Output only. The full file extension extracted from the `name` field. May contain multiple concatenated extensions, such as "tar.gz". |
| `hasAugmentedPermissions` | boolean | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `hasThumbnail` | boolean | Output only. Whether this file has a thumbnail. This doesn't indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `headRevisionId` | string | Output only. The ID of the file's head revision. This is currently only available for files with binary content in Google Drive. |
| `iconLink` | string | Output only. A static, unauthenticated link to the file's icon. |
| `id` | string | The ID of the file. |
| `imageMediaMetadata` | object | Output only. Additional metadata about image media, if available. |
| `inheritedPermissionsDisabled` | boolean | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `isAppAuthorized` | boolean | Output only. Whether the file was created or opened by the requesting app. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#file"`. |
| `labelInfo` | object | Label information on the file. |
| `lastModifyingUser` | `User` | Output only. The last user to modify the file. This field is only populated when the last modification was performed by a signed-in user. |
| `linkShareMetadata` | object | Contains details about the link URLs that clients are using to refer to this item. |
| `md5Checksum` | string | Output only. The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive. |
| `mimeType` | string | The MIME type of the file. Google Drive attempts to automatically detect an appropriate value from uploaded content, if no value is provided. |
| `modifiedByMe` | boolean | Output only. Whether the file has been modified by this user. |
| `modifiedByMeTime` | string (format: date-time) | The last time the file was modified by the user (RFC 3339 date-time). |
| `modifiedTime` | string (format: date-time) | he last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user. |
| `name` | string | The name of the file. This isn't necessarily unique within a folder. |
| `originalFilename` | string | The original filename of the uploaded content if available, or else the original value of the `name` field. This is only available for files with binary content in Google Drive. |
| `ownedByMe` | boolean | Output only. Whether the user owns the file. Not populated for items in shared drives. |
| `owners` | array of `User` | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `parents` | array of string | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. |
| `permissionIds` | array of string | Output only. List of permission IDs for users with access to this file. |
| `permissions` | array of `Permission` | Output only. The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives. |
| `properties` | object | A collection of arbitrary key-value pairs which are visible to all apps.
Entries with null values are cleared in update and copy requests. |
| `quotaBytesUsed` | string (format: int64) | Output only. The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with `keepForever` enabled. |
| `resourceKey` | string | Output only. A key needed to access the item via a shared link. |
| `sha1Checksum` | string | Output only. The SHA1 checksum associated with this file, if available. |
| `sha256Checksum` | string | Output only. The SHA256 checksum associated with this file, if available. |
| `shared` | boolean | Output only. Whether the file has been shared. Not populated for items in shared drives. |
| `sharedWithMeTime` | string (format: date-time) | The time at which the file was shared with the user, if applicable (RFC 3339 date-time). |
| `sharingUser` | `User` | Output only. The user who shared the file with the requesting user, if applicable. |
| `shortcutDetails` | object | Information about a shortcut file. |
| `size` | string (format: int64) | Output only. Size in bytes of blobs and Google Workspace editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `spaces` | array of string | Output only. The list of spaces which contain the file. The currently supported values are `drive`, `appDataFolder`, and `photos`. |
| `starred` | boolean | Whether the user has starred the file. |
| `teamDriveId` | string | Deprecated: Output only. Use `driveId` instead. |
| `thumbnailLink` | string | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. |
| `thumbnailVersion` | string (format: int64) | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `trashed` | boolean | Whether the file has been trashed, either explicitly or from a trashed parent folder. |
| `trashedTime` | string (format: date-time) | The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives. |
| `trashingUser` | `User` | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `version` | string (format: int64) | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user. |
| `videoMediaMetadata` | object | Output only. Additional metadata about video media. This may not be available immediately upon upload. |
| `viewedByMe` | boolean | Output only. Whether the file has been viewed by this user. |
| `viewedByMeTime` | string (format: date-time) | The last time the file was viewed by the user (RFC 3339 date-time). |
| `viewersCanCopyContent` | boolean | Deprecated: Use `copyRequiresWriterPermission` instead. |
| `webContentLink` | string | Output only. A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive. |
| `webViewLink` | string | Output only. A link for opening the file in a relevant Google editor or viewer in a browser. |
| `writersCanShare` | boolean | Whether users with only `writer` permission can modify the file's permissions. Not populated for items in shared drives. |

### FileList

*Description: A list of files.*

| Field | Type | Description |
|---|---|---|
| `files` | array of `File` | The list of files. If `nextPageToken` is populated, then this list may be incomplete and an additional page of results should be fetched. |
| `incompleteSearch` | boolean | Whether the search process was incomplete. If true, then some search results might be missing, since all documents were not searched. |
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#fileList"`. |
| `nextPageToken` | string | The page token for the next page of files. This will be absent if the end of the files list has been reached. |

### GenerateCseTokenResponse

*Description: JWT and associated metadata used to generate CSE files.*

| Field | Type | Description |
|---|---|---|
| `currentKaclsId` | string (format: int64) | The current Key ACL Service (KACLS) ID associated with the JWT. |
| `currentKaclsName` | string | Name of the KACLs that the returned KACLs ID points to. |
| `fileId` | string | The fileId for which the JWT was generated. |
| `jwt` | string | The signed JSON Web Token (JWT) for the file. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#generateCseTokenResponse"`. |

### GeneratedIds

*Description: A list of generated file IDs which can be provided in create requests.*

| Field | Type | Description |
|---|---|---|
| `ids` | array of string | The IDs generated for the requesting user in the specified space. |
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#generatedIds"`. |
| `space` | string | The type of file that can be created with these IDs. |

### LabelList

*Description: A list of labels applied to a file.*

| Field | Type | Description |
|---|---|---|
| `kind` | string | This is always `"drive#labelList"`. |
| `labels` | array of `Label` | The list of labels. |
| `nextPageToken` | string | The page token for the next page of labels. This field will be absent if the end of the list has been reached. |

### ListAccessProposalsResponse

*Description: The response to an access proposal list request.*

| Field | Type | Description |
|---|---|---|
| `accessProposals` | array of `AccessProposal` | The list of access proposals. This field is only populated in Drive API v3. |
| `nextPageToken` | string | The continuation token for the next page of results. This will be absent if the end of the results list has been reached. |

### ModifyLabelsRequest

*Description: A request to modify the set of labels on a file. This request may contain many modifications that will either all succeed or all fail atomically.*

| Field | Type | Description |
|---|---|---|
| `kind` | string | This is always `"drive#modifyLabelsRequest"`. |
| `labelModifications` | array of `LabelModification` | The list of modifications to apply to the labels on the file. |

### ModifyLabelsResponse

*Description: Response to a `ModifyLabels` request. This contains only those labels which were added or updated by the request.*

| Field | Type | Description |
|---|---|---|
| `kind` | string | This is always `"drive#modifyLabelsResponse"`. |
| `modifiedLabels` | array of `Label` | The list of labels which were added or updated by the request. |

### Operation

*Description: This resource represents a long-running operation that is the result of a network API call.*

| Field | Type | Description |
|---|---|---|
| `done` | boolean | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | `Status` | The error result of the operation in case of failure or cancellation. |
| `metadata` | object | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. |
| `name` | string | The server-assigned name, which is only unique within the same service that originally returns it. |
| `response` | object | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. |

### Permission

*Description: A permission for a file. A permission grants a user, group, domain, or the world access to a file or a folder hierarchy. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing).*

| Field | Type | Description |
|---|---|---|
| `allowFileDiscovery` | boolean | Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type `domain` or `anyone`. |
| `deleted` | boolean | Output only. Whether the account associated with this permission has been deleted. This field only pertains to permissions of type `user` or `group`. |
| `displayName` | string | Output only. The "pretty" name of the value of the permission. |
| `domain` | string | Output only. The domain to which this permission refers. |
| `emailAddress` | string | Output only. The email address of the user or group to which this permission refers. |
| `expirationTime` | string (format: date-time) | The time at which this permission will expire (RFC 3339 date-time). |
| `id` | string | Output only. The ID of this permission. |
| `inheritedPermissionsDisabled` | boolean | When `true`, only organizers, owners, and users with permissions added directly on the item can access it. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#permission"`. |
| `pendingOwner` | boolean | Whether the account associated with this permission is a pending owner. Only populated for permissions of type `user` for files that aren't in a shared drive. |
| `permissionDetails` | array of object | Output only. Details of whether the permissions on this item are inherited or are directly on this item. |
| `photoLink` | string | Output only. A link to the user's profile photo, if available. |
| `role` | string | The role granted by this permission. |
| `teamDrivePermissionDetails` | array of object | Output only. Deprecated: Output only. Use `permissionDetails` instead. |
| `type` | string | The type of the grantee. |
| `view` | string | Indicates the view for this permission. Only populated for permissions that belong to a view. |

### PermissionList

*Description: A list of permissions for a file.*

| Field | Type | Description |
|---|---|---|
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#permissionList"`. |
| `nextPageToken` | string | The page token for the next page of permissions. This field will be absent if the end of the permissions list has been reached. |
| `permissions` | array of `Permission` | The list of permissions. If `nextPageToken` is populated, then this list may be incomplete and an additional page of results should be fetched. |

### ReassignApprovalRequest

*Description: Request for reassigning an approval. Reviewers can be added or replaced, but not removed.*

| Field | Type | Description |
|---|---|---|
| `addReviewers` | array of `AddReviewer` | Optional. The list of reviewers to add. |
| `message` | string | Optional. A message to send to the new reviewers. This message is included in notifications for the action and in the approval activity log. |
| `replaceReviewers` | array of `ReplaceReviewer` | Optional. The list of reviewer replacements. |

### Reply

*Description: A reply to a comment on a file. Some resource methods (such as `replies.update`) require a `replyId`. Use the `replies.list` method to retrieve the ID for a reply.*

| Field | Type | Description |
|---|---|---|
| `action` | string | The action the reply performed to the parent comment. The supported values are: * `resolve` * `reopen` |
| `assigneeEmailAddress` | string | Output only. The email address of the user assigned to this comment. If no user is assigned, the field is unset. |
| `author` | `User` | Output only. The author of the reply. The author's email address and permission ID won't be populated. |
| `content` | string | The plain text content of the reply. This field is used for setting the content, while `htmlContent` should be displayed. |
| `createdTime` | string (format: date-time) | The time at which the reply was created (RFC 3339 date-time). |
| `deleted` | boolean | Output only. Whether the reply has been deleted. A deleted reply has no content. |
| `htmlContent` | string | Output only. The content of the reply with HTML formatting. |
| `id` | string | Output only. The ID of the reply. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#reply"`. |
| `mentionedEmailAddresses` | array of string | Output only. A list of email addresses for users mentioned in this comment. If no users are mentioned, the list is empty. |
| `modifiedTime` | string (format: date-time) | The last time the reply was modified (RFC 3339 date-time). |

### ReplyList

*Description: A list of replies to a comment on a file.*

| Field | Type | Description |
|---|---|---|
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#replyList"`. |
| `nextPageToken` | string | The page token for the next page of replies. This will be absent if the end of the replies list has been reached. |
| `replies` | array of `Reply` | The list of replies. If `nextPageToken` is populated, then this list may be incomplete and an additional page of results should be fetched. |

### ResolveAccessProposalRequest

*Description: Request message for resolving an AccessProposal on a file.*

| Field | Type | Description |
|---|---|---|
| `action` | string | Required. The action to take on the access proposal. |
| `role` | array of string | Optional. The roles that the approver has allowed, if any. For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). |
| `sendNotification` | boolean | Optional. Whether to send an email to the requester when the access proposal is denied or accepted. |
| `view` | string | Optional. Indicates the view for this access proposal. This should only be set when the proposal belongs to a view. Only `published` is supported. |

### Revision

*Description: The metadata for a revision to a file. Some resource methods (such as `revisions.update`) require a `revisionId`. Use the `revisions.list` method to retrieve the ID for a revision.*

| Field | Type | Description |
|---|---|---|
| `exportLinks` | object | Output only. Links for exporting Docs Editors files to specific formats. |
| `id` | string | Output only. The ID of the revision. |
| `keepForever` | boolean | Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. |
| `kind` | string | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#revision"`. |
| `lastModifyingUser` | `User` | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `md5Checksum` | string | Output only. The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive. |
| `mimeType` | string | Output only. The MIME type of the revision. |
| `modifiedTime` | string (format: date-time) | The last time the revision was modified (RFC 3339 date-time). |
| `originalFilename` | string | Output only. The original filename used to create this revision. This is only applicable to files with binary content in Drive. |
| `publishAuto` | boolean | Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files. |
| `published` | boolean | Whether this revision is published. This is only applicable to Docs Editors files. |
| `publishedLink` | string | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `publishedOutsideDomain` | boolean | Whether this revision is published outside the domain. This is only applicable to Docs Editors files. |
| `size` | string (format: int64) | Output only. The size of the revision's content in bytes. This is only applicable to files with binary content in Drive. |

### RevisionList

*Description: A list of revisions of a file.*

| Field | Type | Description |
|---|---|---|
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#revisionList"`. |
| `nextPageToken` | string | The page token for the next page of revisions. This will be absent if the end of the revisions list has been reached. |
| `revisions` | array of `Revision` | The list of revisions. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. |

### StartApprovalRequest

*Description: Allows creating an approval on a file.*

| Field | Type | Description |
|---|---|---|
| `dueTime` | string (format: google-datetime) | Optional. The time that the approval is due. |
| `fileContentChangeBehavior` | string | Optional. The behavior of the approval when the file content changes. |
| `lockFile` | boolean | Optional. Whether to lock the file when starting the approval. |
| `message` | string | Optional. A message to send to reviewers when notifying them of the approval request. |
| `reviewerEmails` | array of string | Required. The emails of the users who are set to review the approval. |

### StartPageToken

| Field | Type | Description |
|---|---|---|
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#startPageToken"`. |
| `startPageToken` | string | The starting page token for listing future changes. The page token doesn't expire. |

### TeamDrive

*Description: Deprecated: use the drive collection instead. Next ID: 33*

| Field | Type | Description |
|---|---|---|
| `backgroundImageFile` | object | The background image file for a Team Drive. |
| `backgroundImageLink` | string | A short-lived link to this Team Drive's background image. |
| `capabilities` | object | Capabilities the current user has on this Team Drive. |
| `colorRgb` | string | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `createdTime` | string (format: date-time) | The time at which the Team Drive was created (RFC 3339 date-time). |
| `id` | string | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#teamDrive"`. |
| `name` | string | The name of this Team Drive. |
| `orgUnitId` | string | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `restrictions` | object | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `themeId` | string | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. |

### TeamDriveList

*Description: A list of Team Drives.*

| Field | Type | Description |
|---|---|---|
| `kind` | string | Identifies what kind of resource this is. Value: the fixed string `"drive#teamDriveList"`. |
| `nextPageToken` | string | The page token for the next page of Team Drives. This will be absent if the end of the Team Drives list has been reached. |
| `teamDrives` | array of `TeamDrive` | The list of Team Drives. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws drive --help

# Inspect a method's required params, types, and defaults
gws schema drive.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

