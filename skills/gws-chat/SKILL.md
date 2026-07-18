---
name: gws-chat
description: "Google Chat: Manage Chat spaces and messages."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws chat --help"
---

# chat (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws chat <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+send`](../gws-chat-send/SKILL.md) | Send a message to a space |

## API Resources

### customEmojis

  - `create` — Creates a custom emoji. Custom emojis are only available for Google Workspace accounts, and the administrator must turn custom emojis on for the organization. For more information, see [Learn about custom emojis in Google Chat](https://support.google.com/chat/answer/12800149) and [Manage custom emoji permissions](https://support.google.com/a/answer/12850085).
    - Request body type: `CustomEmoji`
    - Response type: `CustomEmoji`
  - `delete` — Deletes a custom emoji. By default, users can only delete custom emoji they created. [Emoji managers](https://support.google.com/a/answer/12850085) assigned by the administrator can delete any custom emoji in the organization. See [Learn about custom emojis in Google Chat](https://support.google.com/chat/answer/12800149). Custom emojis are only available for Google Workspace accounts, and the administrator must turn custom emojis on for the organization.
    - Required path params: name
    - Response type: `Empty`
  - `get` — Returns details about a custom emoji. Custom emojis are only available for Google Workspace accounts, and the administrator must turn custom emojis on for the organization. For more information, see [Learn about custom emojis in Google Chat](https://support.google.com/chat/answer/12800149) and [Manage custom emoji permissions](https://support.google.com/a/answer/12850085).
    - Required path params: name
    - Response type: `CustomEmoji`
  - `list` — Lists custom emojis visible to the authenticated user. Custom emojis are only available for Google Workspace accounts, and the administrator must turn custom emojis on for the organization. For more information, see [Learn about custom emojis in Google Chat](https://support.google.com/chat/answer/12800149) and [Manage custom emoji permissions](https://support.google.com/a/answer/12850085).
    - Response type: `ListCustomEmojisResponse`

### media

  - `download` — Downloads media. Download is supported on the URI `/v1/media/{+name}?alt=media`.
    - Required path params: resourceName
    - Response type: `Media`
  - `upload` — Uploads an attachment. For an example, see [Upload media as a file attachment](https://developers.google.com/workspace/chat/upload-media-attachments).
    - Required path params: parent
    - Request body type: `UploadAttachmentRequest`
    - Response type: `UploadAttachmentResponse`

### spaces

  - `completeImport` — Completes the [import process](https://developers.google.com/workspace/chat/import-data) for the specified space and makes it visible to users.
    - Required path params: name
    - Request body type: `CompleteImportSpaceRequest`
    - Response type: `CompleteImportSpaceResponse`
  - `create` — Creates a space. Can be used to create a named space, or a group chat in `Import mode`. For an example, see [Create a space](https://developers.google.com/workspace/chat/create-spaces).
    - Request body type: `Space`
    - Response type: `Space`
  - `delete` — Deletes a named space. Always performs a cascading delete, which means that the space's child resources—like messages posted in the space and memberships in the space—are also deleted. For an example, see [Delete a space](https://developers.google.com/workspace/chat/delete-spaces).
    - Required path params: name
    - Response type: `Empty`
  - `findDirectMessage` — Returns the existing direct message with the specified user. If no direct message space is found, returns a `404 NOT_FOUND` error. For an example, see [Find a direct message](/chat/api/guides/v1/spaces/find-direct-message). With [app authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app), returns the direct message space between the specified user and the calling Chat app.
    - Response type: `Space`
  - `findGroupChats` — Returns all spaces with `spaceType == GROUP_CHAT`, whose human memberships contain exactly the calling user, and the users specified in `FindGroupChatsRequest.users`. Only members that have joined the conversation are supported. For an example, see [Find group chats](https://developers.google.com/workspace/chat/find-group-chats).
    - Response type: `FindGroupChatsResponse`
  - `get` — Returns details about a space. For an example, see [Get details about a space](https://developers.google.com/workspace/chat/get-spaces).
    - Required path params: name
    - Response type: `Space`
  - `list` — Lists spaces the caller is a member of. Group chats and DMs aren't listed until the first message is sent. For an example, see [List spaces](https://developers.google.com/workspace/chat/list-spaces).
    - Response type: `ListSpacesResponse`
  - `patch` — Updates a space. For an example, see [Update a space](https://developers.google.com/workspace/chat/update-spaces). If you're updating the `displayName` field and receive the error message `ALREADY_EXISTS`, try a different display name.. An existing space within the Google Workspace organization might already use this display name.
    - Required path params: name
    - Request body type: `Space`
    - Response type: `Space`
  - `search` — Returns a list of spaces in a Google Workspace organization. For an example, see [Search for and manage spaces](https://developers.google.com/workspace/chat/search-manage-admin). When `use_admin_access` is set to `false`, the results are limited to spaces where the calling user is a joined member. To search with administrator privileges, set `use_admin_access` to `true`. Setting `use_admin_access` to `false` is available under Developer Preview.
    - Response type: `SearchSpacesResponse`
  - `setup` — Creates a space and adds specified users to it. The calling user is automatically added to the space, and shouldn't be specified as a membership in the request. For an example, see [Set up a space with initial members](https://developers.google.com/workspace/chat/set-up-spaces). To specify the human members to add, add memberships with the appropriate `membership.member.name`. To add a human user, use `users/{user}`, where `{user}` can be the email address for the user.
    - Request body type: `SetUpSpaceRequest`
    - Response type: `Space`

### spaces.members

  - `create` — Creates a membership for the calling Chat app, a user, or a Google Group. Creating memberships for other Chat apps isn't supported. When creating a membership, if the specified member has their auto-accept policy turned off, then they're invited, and must accept the space invitation before joining. Otherwise, creating a membership adds the member directly to the specified space.
    - Required path params: parent
    - Request body type: `Membership`
    - Response type: `Membership`
  - `delete` — Deletes a membership. For an example, see [Remove a user or a Google Chat app from a space](https://developers.google.com/workspace/chat/delete-members).
    - Required path params: name
    - Response type: `Membership`
  - `get` — Returns details about a membership. For an example, see [Get details about a user's or Google Chat app's membership](https://developers.google.com/workspace/chat/get-members).
    - Required path params: name
    - Response type: `Membership`
  - `list` — Lists memberships in a space. For an example, see [List users and Google Chat apps in a space](https://developers.google.com/workspace/chat/list-members). Listing memberships with [app authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) lists memberships in spaces that the Chat app has access to, but excludes Chat app memberships, including its own.
    - Required path params: parent
    - Response type: `ListMembershipsResponse`
  - `patch` — Updates a membership. For an example, see [Update a user's membership in a space](https://developers.google.com/workspace/chat/update-members).
    - Required path params: name
    - Request body type: `Membership`
    - Response type: `Membership`

### spaces.messages

  - `create` — Creates a message in a Google Chat space. For an example, see [Send a message](https://developers.google.com/workspace/chat/create-messages).
    - Required path params: parent
    - Request body type: `Message`
    - Response type: `Message`
  - `delete` — Deletes a message. For an example, see [Delete a message](https://developers.google.com/workspace/chat/delete-messages).
    - Required path params: name
    - Response type: `Empty`
  - `get` — Returns details about a message. For an example, see [Get details about a message](https://developers.google.com/workspace/chat/get-messages).
    - Required path params: name
    - Response type: `Message`
  - `list` — Lists messages in a space that the caller is a member of, including messages from blocked members and spaces. System messages, like those announcing new space members, aren't included. If you list messages from a space with no messages, the response is an empty object. When using a REST/HTTP interface, the response contains an empty JSON object, `{}`. For an example, see [List messages](https://developers.google.com/workspace/chat/api/guides/v1/messages/list).
    - Required path params: parent
    - Response type: `ListMessagesResponse`
  - `patch` — Updates a message. There's a difference between the `patch` and `update` methods. The `patch` method uses a `patch` request while the `update` method uses a `put` request. We recommend using the `patch` method. For an example, see [Update a message](https://developers.google.com/workspace/chat/update-messages).
    - Required path params: name
    - Request body type: `Message`
    - Response type: `Message`
  - `update` — Updates a message. There's a difference between the `patch` and `update` methods. The `patch` method uses a `patch` request while the `update` method uses a `put` request. We recommend using the `patch` method. For an example, see [Update a message](https://developers.google.com/workspace/chat/update-messages).
    - Required path params: name
    - Request body type: `Message`
    - Response type: `Message`

### spaces.messages.attachments

  - `get` — Gets the metadata of a message attachment. The attachment data is fetched using the [media API](https://developers.google.com/workspace/chat/api/reference/rest/v1/media/download). For an example, see [Get metadata about a message attachment](https://developers.google.com/workspace/chat/get-media-attachments).
    - Required path params: name
    - Response type: `Attachment`

### spaces.messages.reactions

  - `create` — Creates a reaction and adds it to a message. For an example, see [Add a reaction to a message](https://developers.google.com/workspace/chat/create-reactions).
    - Required path params: parent
    - Request body type: `Reaction`
    - Response type: `Reaction`
  - `delete` — Deletes a reaction to a message. For an example, see [Delete a reaction](https://developers.google.com/workspace/chat/delete-reactions).
    - Required path params: name
    - Response type: `Empty`
  - `list` — Lists reactions to a message. For an example, see [List reactions for a message](https://developers.google.com/workspace/chat/list-reactions).
    - Required path params: parent
    - Response type: `ListReactionsResponse`

### spaces.spaceEvents

  - `get` — Returns an event from a Google Chat space. The [event payload](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.spaceEvents#SpaceEvent.FIELDS.oneof_payload) contains the most recent version of the resource that changed. For example, if you request an event about a new message but the message was later updated, the server returns the updated `Message` resource in the event payload.
    - Required path params: name
    - Response type: `SpaceEvent`
  - `list` — Lists events from a Google Chat space. For each event, the [payload](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.spaceEvents#SpaceEvent.FIELDS.oneof_payload) contains the most recent version of the Chat resource. For example, if you list events about new space members, the server returns `Membership` resources that contain the latest membership details.
    - Required path params: parent
    - Response type: `ListSpaceEventsResponse`

### users.availability

  - `get` — Returns availability information for a human user in Google Chat. For example, this can be used to check if a user is online or away, or to retrieve their custom status message. This method only retrieves the authenticated user's availability.
    - Required path params: name
    - Response type: `Availability`
  - `markAsActive` — Marks user as `ACTIVE` in Google Chat. Sets the user's availability state to `ACTIVE`. The `ACTIVE` state lasts until the specified expiration, at which point the user's state becomes `AWAY`. Note that if the user is actively using Chat, the `ACTIVE` state duration may extend beyond the provided expiration. This method only updates the authenticated user's availability.
    - Required path params: name
    - Request body type: `MarkAsActiveRequest`
    - Response type: `Availability`
  - `markAsAway` — Marks user as `AWAY` in Google Chat. Sets the user's state to away and is not affected by the user's activity. This method only updates the authenticated user's availability. Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.availability`
    - Required path params: name
    - Request body type: `MarkAsAwayRequest`
    - Response type: `Availability`
  - `markAsDoNotDisturb` — Marks user as `DO_NOT_DISTURB` in Google Chat. Sets a user's availability state to `DO_NOT_DISTURB` until a specified expiration time. When in `DO_NOT_DISTURB`, users typically won't receive notifications. This method only updates the authenticated user's availability.
    - Required path params: name
    - Request body type: `MarkAsDoNotDisturbRequest`
    - Response type: `Availability`
  - `patch` — Updates availability information for a human user. Only the `custom_status` field can be updated through this method. This method only updates the authenticated user's availability. Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following [authorization scopes](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.availability`
    - Required path params: name
    - Request body type: `Availability`
    - Response type: `Availability`

### users.sections

  - `create` — Creates a section in Google Chat. Sections help users group conversations and customize the list of spaces displayed in Chat navigation panel. Only sections of type `CUSTOM_SECTION` can be created. For details, see [Create and organize sections in Google Chat](https://support.google.com/chat/answer/16059854).
    - Required path params: parent
    - Request body type: `GoogleChatV1Section`
    - Response type: `GoogleChatV1Section`
  - `delete` — Deletes a section of type `CUSTOM_SECTION`. If the section contains items, such as spaces, the items are moved to Google Chat's default sections and are not deleted. For details, see [Create and organize sections in Google Chat](https://support.google.com/chat/answer/16059854).
    - Required path params: name
    - Response type: `Empty`
  - `list` — Lists sections available to the Chat user. Sections help users group their conversations and customize the list of spaces displayed in Chat navigation panel. For details, see [Create and organize sections in Google Chat](https://support.google.com/chat/answer/16059854).
    - Required path params: parent
    - Response type: `ListSectionsResponse`
  - `patch` — Updates a section. Only sections of type `CUSTOM_SECTION` can be updated. For details, see [Create and organize sections in Google Chat](https://support.google.com/chat/answer/16059854). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.sections`
    - Required path params: name
    - Request body type: `GoogleChatV1Section`
    - Response type: `GoogleChatV1Section`
  - `position` — Changes the sort order of a section. For details, see [Create and organize sections in Google Chat](https://support.google.com/chat/answer/16059854). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.sections`
    - Required path params: name
    - Request body type: `PositionSectionRequest`
    - Response type: `PositionSectionResponse`

### users.sections.items

  - `list` — Lists items in a section. Only spaces can be section items. For details, see [Create and organize sections in Google Chat](https://support.google.com/chat/answer/16059854).
    - Required path params: parent
    - Response type: `ListSectionItemsResponse`
  - `move` — Moves an item from one section to another. For example, if a section contains spaces, this method can be used to move a space to a different section. For details, see [Create and organize sections in Google Chat](https://support.google.com/chat/answer/16059854).
    - Required path params: name
    - Request body type: `MoveSectionItemRequest`
    - Response type: `MoveSectionItemResponse`

### users.spaces

  - `getSpaceReadState` — Returns details about a user's read state within a space, used to identify read and unread messages. For an example, see [Get details about a user's space read state](https://developers.google.com/workspace/chat/get-space-read-state).
    - Required path params: name
    - Response type: `SpaceReadState`
  - `updateSpaceReadState` — Updates a user's read state within a space, used to identify read and unread messages. For an example, see [Update a user's space read state](https://developers.google.com/workspace/chat/update-space-read-state). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.readstate`
    - Required path params: name
    - Request body type: `SpaceReadState`
    - Response type: `SpaceReadState`

### users.spaces.spaceNotificationSetting

  - `get` — Gets the space notification setting. For an example, see [Get the caller's space notification setting](https://developers.google.com/workspace/chat/get-space-notification-setting). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.spacesettings`
    - Required path params: name
    - Response type: `SpaceNotificationSetting`
  - `patch` — Updates the space notification setting. For an example, see [Update the caller's space notification setting](https://developers.google.com/workspace/chat/update-space-notification-setting). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.spacesettings`
    - Required path params: name
    - Request body type: `SpaceNotificationSetting`
    - Response type: `SpaceNotificationSetting`

### users.spaces.threads

  - `getThreadReadState` — Returns details about a user's read state within a thread, used to identify read and unread messages. For an example, see [Get details about a user's thread read state](https://developers.google.com/workspace/chat/get-thread-read-state).
    - Required path params: name
    - Response type: `ThreadReadState`

## Common Schemas

### Attachment

*Description: An attachment in Google Chat.*

| Field | Type | Description |
|---|---|---|
| `attachmentDataRef` | `AttachmentDataRef` | Optional. A reference to the attachment data. This field is used to create or update messages with attachments, or with the media API to download the attachment data. |
| `contentName` | string | Output only. The original file name for the content, not the full path. |
| `contentType` | string | Output only. The content type (MIME type) of the file. |
| `downloadUri` | string | Output only. The download URL which should be used to allow a human user to download the attachment. Chat apps shouldn't use this URL to download attachment content. |
| `driveDataRef` | `DriveDataRef` | Output only. A reference to the Google Drive attachment. This field is used with the Google Drive API. |
| `name` | string | Identifier. Resource name of the attachment. Format: `spaces/{space}/messages/{message}/attachments/{attachment}`. |
| `source` | string | Output only. The source of the attachment. |
| `thumbnailUri` | string | Output only. The thumbnail URL which should be used to preview the attachment to a human user. Chat apps shouldn't use this URL to download attachment content. |

### Availability

*Description: Represents a user's current availability information in Google Chat, including their state (for example, Active, Away, Do Not Disturb) and any custom status.*

| Field | Type | Description |
|---|---|---|
| `customStatus` | `CustomStatus` | Optional. The user's custom status. |
| `doNotDisturbMetadata` | `DoNotDisturbMetadata` | Output only. Metadata if the user state is set to DO_NOT_DISTURB. |
| `name` | string | Identifier. Resource name of the user's availability. Format: `users/{user}/availability` `{user}` is the id for the Person in the People API or Admin SDK directory API. |
| `state` | string | Output only. The user's current availability state. |

### CompleteImportSpaceRequest

*Description: Request message for completing the import process for a space.*

*(No fields)*

### CompleteImportSpaceResponse

*Description: Response message for completing the import process for a space.*

| Field | Type | Description |
|---|---|---|
| `space` | `Space` | The import mode space. |

### CustomEmoji

*Description: Represents a [custom emoji](https://support.google.com/chat/answer/12800149).*

| Field | Type | Description |
|---|---|---|
| `emojiName` | string | Optional. Immutable. User-provided name for the custom emoji, which is unique within the organization. Required when the custom emoji is created, output only otherwise. |
| `name` | string | Identifier. The resource name of the custom emoji, assigned by the server. Format: `customEmojis/{customEmoji}` |
| `payload` | `CustomEmojiPayload` | Optional. Input only. Payload data. Required when the custom emoji is created. |
| `temporaryImageUri` | string | Output only. A temporary image URL for the custom emoji, valid for at least 10 minutes. Note that this is not populated in the response when the custom emoji is created. |
| `uid` | string | Output only. Unique key for the custom emoji resource. |

### Empty

*Description: A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }*

*(No fields)*

### FindGroupChatsResponse

*Description: A response containing group chat spaces with exactly the calling user and the requested users.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | A token that you can send as `pageToken` to retrieve the next page of results. If empty, there are no subsequent pages. |
| `spaces` | array of `Space` | List of spaces in the requested (or first) page. |

### GoogleChatV1Section

*Description: Represents a [section](https://support.google.com/chat/answer/16059854) in Google Chat. Sections help users organize their spaces. There are two types of sections: 1. **System Sections:** These are predefined sections managed by Google Chat.*

| Field | Type | Description |
|---|---|---|
| `displayName` | string | Optional. The section's display name. Only populated for sections of type `CUSTOM_SECTION`. Supports up to 80 characters. Required when creating a `CUSTOM_SECTION`. |
| `name` | string | Identifier. Resource name of the section. |
| `sortOrder` | integer (format: int32) | Output only. The order of the section in relation to other sections. Sections with a lower `sort_order` value appear before sections with a higher value. |
| `type` | string | Required. The type of the section. |

### ListCustomEmojisResponse

*Description: A response to list custom emojis.*

| Field | Type | Description |
|---|---|---|
| `customEmojis` | array of `CustomEmoji` | Unordered list. List of custom emojis. |
| `nextPageToken` | string | A token that you can send as `pageToken` to retrieve the next page of results. If empty, there are no subsequent pages. |

### ListMembershipsResponse

*Description: Response to list memberships of the space.*

| Field | Type | Description |
|---|---|---|
| `memberships` | array of `Membership` | Unordered list. List of memberships in the requested (or first) page. |
| `nextPageToken` | string | A token that you can send as `pageToken` to retrieve the next page of results. If empty, there are no subsequent pages. |

### ListMessagesResponse

*Description: Response message for listing messages.*

| Field | Type | Description |
|---|---|---|
| `messages` | array of `Message` | List of messages. |
| `nextPageToken` | string | You can send a token as `pageToken` to retrieve the next page of results. If empty, there are no subsequent pages. |

### ListReactionsResponse

*Description: Response to a list reactions request.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Continuation token to retrieve the next page of results. It's empty for the last page of results. |
| `reactions` | array of `Reaction` | List of reactions in the requested (or first) page. |

### ListSectionItemsResponse

*Description: Response message for listing section items.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `sectionItems` | array of `SectionItem` | The section items from the specified section. |

### ListSectionsResponse

*Description: Response message for listing sections.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `sections` | array of `GoogleChatV1Section` | The sections from the specified user. |

### ListSpaceEventsResponse

*Description: Response message for listing space events.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Continuation token used to fetch more events. If this field is omitted, there are no subsequent pages. |
| `spaceEvents` | array of `SpaceEvent` | Results are returned in chronological order (oldest event first). Note: The `permissionSettings` field is not returned in the Space object for list requests. |

### ListSpacesResponse

*Description: The response for a list spaces request.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | You can send a token as `pageToken` to retrieve the next page of results. If empty, there are no subsequent pages. |
| `spaces` | array of `Space` | List of spaces in the requested (or first) page. Note: The `permissionSettings` field is not returned in the Space object for list requests. |

### MarkAsActiveRequest

*Description: Request message for the `MarkAsActive` method.*

| Field | Type | Description |
|---|---|---|
| `expireTime` | string (format: google-datetime) | The absolute timestamp when the ACTIVE state expires. |
| `ttl` | string (format: google-duration) | The duration from the current time until the ACTIVE state expires. Using a short TTL can effectively reset the user's state to be based on activity after this brief duration. |

### MarkAsAwayRequest

*Description: Request message for the `MarkAsAway` method.*

*(No fields)*

### MarkAsDoNotDisturbRequest

*Description: Request message for the `MarkAsDoNotDisturb` method.*

| Field | Type | Description |
|---|---|---|
| `expireTime` | string (format: google-datetime) | The absolute timestamp when the DND state expires. |
| `ttl` | string (format: google-duration) | The duration from the current time until the DND state expires. |

### Media

*Description: Media resource.*

| Field | Type | Description |
|---|---|---|
| `resourceName` | string | Name of the media resource. |

### Membership

*Description: Represents a membership relation in Google Chat, such as whether a user or Chat app is invited to, part of, or absent from a space.*

| Field | Type | Description |
|---|---|---|
| `affiliation` | string | Output only. A user's relationship to the Workspace organization that owns the space. In spaces owned by consumer accounts, the affiliation of all members is `EXTERNAL`. |
| `createTime` | string (format: google-datetime) | Optional. Immutable. The creation time of the membership, such as when a member joined or was invited to join a space. |
| `deleteTime` | string (format: google-datetime) | Optional. Immutable. The deletion time of the membership, such as when a member left or was removed from a space. |
| `groupMember` | `Group` | Optional. The Google Group the membership corresponds to. |
| `member` | `User` | Optional. The Google Chat user or app the membership corresponds to. |
| `name` | string | Identifier. Resource name of the membership, assigned by the server. Format: `spaces/{space}/members/{member}` |
| `role` | string | Optional. User's role within a Chat space, which determines their permitted actions in the space. This field can only be used as input in `UpdateMembership`. |
| `state` | string | Output only. State of the membership. |

### Message

*Description: A message in a Google Chat space.*

| Field | Type | Description |
|---|---|---|
| `accessoryWidgets` | array of `AccessoryWidget` | Optional. One or more interactive widgets that appear at the bottom of a message. You can add accessory widgets to messages that contain text, cards, or both text and cards. |
| `actionResponse` | `ActionResponse` | Input only. Parameters that a Chat app can use to configure how its response is posted. |
| `annotations` | array of `Annotation` | Output only. |
| `argumentText` | string | Output only. Plain-text body of the message with all Chat app mentions stripped out. |
| `attachedGifs` | array of `AttachedGif` | Output only. GIF images that are attached to the message. |
| `attachment` | array of `Attachment` | Optional. User-uploaded attachment. |
| `cards` | array of `Card` | Deprecated: Use `cards_v2` instead. Rich, formatted, and interactive cards that you can use to display UI elements such as: formatted texts, buttons, and clickable images. |
| `cardsV2` | array of `CardWithId` | Optional. An array of [cards](https://developers.google.com/workspace/chat/api/reference/rest/v1/cards). |
| `clientAssignedMessageId` | string | Optional. A custom ID for the message. You can use field to identify a message, or to get, delete, or update a message. |
| `createTime` | string (format: google-datetime) | Optional. Immutable. For spaces created in Chat, the time at which the message was created. This field is output only, except when used in import mode spaces. |
| `deleteTime` | string (format: google-datetime) | Output only. The time at which the message was deleted in Google Chat. If the message is never deleted, this field is empty. |
| `deletionMetadata` | `DeletionMetadata` | Output only. Information about a deleted message. A message is deleted when `delete_time` is set. |
| `emojiReactionSummaries` | array of `EmojiReactionSummary` | Output only. The list of emoji reaction summaries on the message. |
| `fallbackText` | string | Optional. A plain-text description of the message's cards, used when the actual cards can't be displayed—for example, mobile notifications. |
| `formattedText` | string | Output only. Contains the message `text` with markups added to communicate formatting. |
| `lastUpdateTime` | string (format: google-datetime) | Output only. The time at which the message was last edited by a user. If the message has never been edited, this field is empty. |
| `matchedUrl` | `MatchedUrl` | Output only. A URL in the Chat message `text` field that matches a link preview pattern. For more information, see [Preview links](https://developers.google.com/workspace/chat/preview-links). |
| `name` | string | Identifier. Resource name of the message. |
| `privateMessageViewer` | `User` | Optional. Immutable. Input for creating a message, otherwise output only. The user that can view the message. When set, the message is private and only visible to the specified user and the Chat app. |
| `quotedMessageMetadata` | `QuotedMessageMetadata` | Optional. Information about a message that another message quotes. When you create a message, you can quote messages within the same thread, or quote a root message to create a new root message. |
| `sender` | `User` | Output only. The user who created the message. |
| `silent` | boolean | Output only. Whether this is a silent message. Silent messages are messages where Chat suppresses push notifications for recipients. |
| `slashCommand` | `SlashCommand` | Output only. Slash command information, if applicable. |
| `space` | `Space` | Output only. If your Chat app [authenticates as a user](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user), the output only populates the [space](https://developers.google. |
| `text` | string | Optional. Plain-text body of the message. The first link to an image, video, or web page generates a [preview chip](https://developers.google.com/workspace/chat/preview-links). |
| `thread` | `Thread` | The thread the message belongs to. For example usage, see [Start or reply to a message thread](https://developers.google.com/workspace/chat/create-messages#create-message-thread). |
| `threadReply` | boolean | Output only. When `true`, the message is a response in a reply thread. |

### MoveSectionItemRequest

*Description: Request message for moving a section item across sections.*

| Field | Type | Description |
|---|---|---|
| `targetSection` | string | Required. The resource name of the section to move the section item to. Format: `users/{user}/sections/{section}` |

### MoveSectionItemResponse

*Description: Response message for moving a section item.*

| Field | Type | Description |
|---|---|---|
| `sectionItem` | `SectionItem` | The updated section item. |

### PositionSectionRequest

*Description: Request message for positioning a section.*

| Field | Type | Description |
|---|---|---|
| `relativePosition` | string | Optional. The relative position of the section in the list of sections. |
| `sortOrder` | integer (format: int32) | Optional. The absolute position of the section in the list of sections. The position must be greater than 0. |

### PositionSectionResponse

*Description: Response message for positioning a section.*

| Field | Type | Description |
|---|---|---|
| `section` | `GoogleChatV1Section` | The updated section. |

### Reaction

*Description: A reaction to a message.*

| Field | Type | Description |
|---|---|---|
| `emoji` | `Emoji` | Required. The emoji used in the reaction. |
| `name` | string | Identifier. The resource name of the reaction. Format: `spaces/{space}/messages/{message}/reactions/{reaction}` |
| `user` | `User` | Output only. The user who created the reaction. |

### SearchSpacesResponse

*Description: Response with a list of spaces corresponding to the search spaces request.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | A token that can be used to retrieve the next page. If this field is empty, there are no subsequent pages. |
| `spaces` | array of `Space` | Deprecated: Please use the new `results` field instead. A page of the requested spaces. |
| `totalSize` | integer (format: int32) | The total number of spaces that match the query, across all pages. If the result is over 10,000 spaces, this value is an estimate. |

### SetUpSpaceRequest

*Description: Request to create a space and add specified users to it.*

| Field | Type | Description |
|---|---|---|
| `memberships` | array of `Membership` | Optional. The Google Chat users or groups to invite to join the space. Omit the calling user, as they are added automatically. |
| `requestId` | string | Optional. A unique identifier for this request. A random UUID is recommended. Specifying an existing request ID returns the space created with that ID instead of creating a new space. |
| `space` | `Space` | Required. The `Space.spaceType` field is required. To create a space, set `Space.spaceType` to `SPACE` and set `Space.displayName`. |

### Space

*Description: A space in Google Chat. Spaces are conversations between two or more users or 1:1 messages between a user and a Chat app.*

| Field | Type | Description |
|---|---|---|
| `accessSettings` | `AccessSettings` | Optional. Specifies the [access setting](https://support.google.com/chat/answer/11971020) of the space. Only populated when the `space_type` is `SPACE`. |
| `adminInstalled` | boolean | Output only. For direct message (DM) spaces with a Chat app, whether the space was created by a Google Workspace administrator. |
| `createTime` | string (format: google-datetime) | Optional. Immutable. For spaces created in Chat, the time the space was created. This field is output only, except when used in import mode spaces. |
| `customer` | string | Optional. Immutable. The customer id of the domain of the space. |
| `displayName` | string | Optional. The space's display name. Required when [creating a space](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces/create) with a `spaceType` of `SPACE`. |
| `externalUserAllowed` | boolean | Optional. Immutable. Whether this space permits any Google Chat user as a member. Input when creating a space in a Google Workspace organization. |
| `importMode` | boolean | Optional. Whether this space is created in `Import Mode` as part of a data migration into Google Workspace. While spaces are being imported, they aren't visible to users until the import is complete. |
| `importModeExpireTime` | string (format: google-datetime) | Output only. The time when the space will be automatically deleted by the system if it remains in import mode. |
| `lastActiveTime` | string (format: google-datetime) | Output only. Timestamp of the last message in the space. |
| `membershipCount` | `MembershipCount` | Output only. The count of joined memberships grouped by member type. Populated when the `space_type` is `SPACE`, `DIRECT_MESSAGE` or `GROUP_CHAT`. |
| `name` | string | Identifier. Resource name of the space. Format: `spaces/{space}` Where `{space}` represents the system-assigned ID for the space. |
| `permissionSettings` | `PermissionSettings` | Optional. Space permission settings for existing spaces. Input for updating exact space permission settings, where existing permission settings are replaced. Output lists current permission settings. |
| `predefinedPermissionSettings` | string | Optional. Input only. Predefined space permission settings, input only when creating a space. If the field is not set, a collaboration space is created. |
| `singleUserBotDm` | boolean | Optional. Whether the space is a DM between a Chat app and a single human. |
| `spaceDetails` | `SpaceDetails` | Optional. Details about the space including description and rules. |
| `spaceHistoryState` | string | Optional. The message history state for messages and threads in this space. |
| `spaceThreadingState` | string | Output only. The threading state in the Chat space. |
| `spaceType` | string | Optional. The type of space. Required when creating a space or updating the space type of a space. Output only for other usage. |
| `spaceUri` | string | Output only. The URI for a user to access the space. |
| `threaded` | boolean | Output only. Deprecated: Use `spaceThreadingState` instead. Whether messages are threaded in this space. |
| `type` | string | Output only. Deprecated: Use `space_type` instead. The type of a space. |

### SpaceEvent

*Description: An event that represents a change or activity in a Google Chat space. To learn more, see [Work with events from Google Chat](https://developers.google.com/workspace/chat/events-overview).*

| Field | Type | Description |
|---|---|---|
| `eventTime` | string (format: google-datetime) | Time when the event occurred. |
| `eventType` | string | Type of space event. Each event type has a batch version, which represents multiple instances of the event type that occur in a short period of time. |
| `membershipBatchCreatedEventData` | `MembershipBatchCreatedEventData` | Event payload for multiple new memberships. Event type: `google.workspace.chat.membership.v1.batchCreated` |
| `membershipBatchDeletedEventData` | `MembershipBatchDeletedEventData` | Event payload for multiple deleted memberships. Event type: `google.workspace.chat.membership.v1.batchDeleted` |
| `membershipBatchUpdatedEventData` | `MembershipBatchUpdatedEventData` | Event payload for multiple updated memberships. Event type: `google.workspace.chat.membership.v1.batchUpdated` |
| `membershipCreatedEventData` | `MembershipCreatedEventData` | Event payload for a new membership. Event type: `google.workspace.chat.membership.v1.created` |
| `membershipDeletedEventData` | `MembershipDeletedEventData` | Event payload for a deleted membership. Event type: `google.workspace.chat.membership.v1.deleted` |
| `membershipUpdatedEventData` | `MembershipUpdatedEventData` | Event payload for an updated membership. Event type: `google.workspace.chat.membership.v1.updated` |
| `messageBatchCreatedEventData` | `MessageBatchCreatedEventData` | Event payload for multiple new messages. Event type: `google.workspace.chat.message.v1.batchCreated` |
| `messageBatchDeletedEventData` | `MessageBatchDeletedEventData` | Event payload for multiple deleted messages. Event type: `google.workspace.chat.message.v1.batchDeleted` |
| `messageBatchUpdatedEventData` | `MessageBatchUpdatedEventData` | Event payload for multiple updated messages. Event type: `google.workspace.chat.message.v1.batchUpdated` |
| `messageCreatedEventData` | `MessageCreatedEventData` | Event payload for a new message. Event type: `google.workspace.chat.message.v1.created` |
| `messageDeletedEventData` | `MessageDeletedEventData` | Event payload for a deleted message. Event type: `google.workspace.chat.message.v1.deleted` |
| `messageUpdatedEventData` | `MessageUpdatedEventData` | Event payload for an updated message. Event type: `google.workspace.chat.message.v1.updated` |
| `name` | string | Resource name of the space event. Format: `spaces/{space}/spaceEvents/{spaceEvent}` |
| `reactionBatchCreatedEventData` | `ReactionBatchCreatedEventData` | Event payload for multiple new reactions. Event type: `google.workspace.chat.reaction.v1.batchCreated` |
| `reactionBatchDeletedEventData` | `ReactionBatchDeletedEventData` | Event payload for multiple deleted reactions. Event type: `google.workspace.chat.reaction.v1.batchDeleted` |
| `reactionCreatedEventData` | `ReactionCreatedEventData` | Event payload for a new reaction. Event type: `google.workspace.chat.reaction.v1.created` |
| `reactionDeletedEventData` | `ReactionDeletedEventData` | Event payload for a deleted reaction. Event type: `google.workspace.chat.reaction.v1.deleted` |
| `spaceBatchUpdatedEventData` | `SpaceBatchUpdatedEventData` | Event payload for multiple updates to a space. Event type: `google.workspace.chat.space.v1.batchUpdated` |
| `spaceUpdatedEventData` | `SpaceUpdatedEventData` | Event payload for a space update. Event type: `google.workspace.chat.space.v1.updated` |

### SpaceNotificationSetting

*Description: The notification setting of a user in a space.*

| Field | Type | Description |
|---|---|---|
| `muteSetting` | string | The space notification mute setting. |
| `name` | string | Identifier. The resource name of the space notification setting. Format: `users/{user}/spaces/{space}/spaceNotificationSetting`. |
| `notificationSetting` | string | The notification setting. |

### SpaceReadState

*Description: A user's read state within a space, used to identify read and unread messages.*

| Field | Type | Description |
|---|---|---|
| `lastReadTime` | string (format: google-datetime) | Optional. The time when the user's space read state was updated. |
| `name` | string | Resource name of the space read state. Format: `users/{user}/spaces/{space}/spaceReadState` |

### ThreadReadState

*Description: A user's read state within a thread, used to identify read and unread messages.*

| Field | Type | Description |
|---|---|---|
| `lastReadTime` | string (format: google-datetime) | The time when the user's thread read state was updated. Usually this corresponds with the timestamp of the last read message in a thread. |
| `name` | string | Resource name of the thread read state. Format: `users/{user}/spaces/{space}/threads/{thread}/threadReadState` |

### UploadAttachmentRequest

*Description: Request to upload an attachment.*

| Field | Type | Description |
|---|---|---|
| `filename` | string | Required. The filename of the attachment, including the file extension. |

### UploadAttachmentResponse

*Description: Response of uploading an attachment.*

| Field | Type | Description |
|---|---|---|
| `attachmentDataRef` | `AttachmentDataRef` | Reference to the uploaded attachment. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws chat --help

# Inspect a method's required params, types, and defaults
gws schema chat.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

