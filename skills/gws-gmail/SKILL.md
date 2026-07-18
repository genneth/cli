---
name: gws-gmail
description: "Gmail: Send, read, and manage email."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws gmail --help"
---

# gmail (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws gmail <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+send`](../gws-gmail-send/SKILL.md) | Send an email |
| [`+triage`](../gws-gmail-triage/SKILL.md) | Show unread inbox summary (sender, subject, date) |
| [`+reply`](../gws-gmail-reply/SKILL.md) | Reply to a message (handles threading automatically) |
| [`+reply-all`](../gws-gmail-reply-all/SKILL.md) | Reply-all to a message (handles threading automatically) |
| [`+forward`](../gws-gmail-forward/SKILL.md) | Forward a message to new recipients |
| [`+read`](../gws-gmail-read/SKILL.md) | Read a message and extract its body or headers |
| [`+watch`](../gws-gmail-watch/SKILL.md) | Watch for new emails and stream them as NDJSON |

## API Resources

### users

  - `getProfile` — Gets the current user's Gmail profile.
    - Required path params: userId
    - Response type: `Profile`
  - `stop` — Turn off push notification delivery for the given user mailbox. For more information, see [Configure push notifications in Gmail API](https://developers.google.com/workspace/gmail/api/guides/push).
    - Required path params: userId
  - `watch` — Set up or update a push notification watch on the given user mailbox. For more information, see [Configure push notifications in Gmail API](https://developers.google.com/workspace/gmail/api/guides/push).
    - Required path params: userId
    - Request body type: `WatchRequest`
    - Response type: `WatchResponse`

### users.drafts

  - `create` — Creates a draft with the `DRAFT` label. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).
    - Required path params: userId
    - Request body type: `Draft`
    - Response type: `Draft`
  - `delete` — Immediately and permanently deletes the specified draft. Does not simply trash it. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).
    - Required path params: id, userId
  - `get` — Gets the specified draft. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).
    - Required path params: id, userId
    - Response type: `Draft`
  - `list` — Lists the drafts in the user's mailbox. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).
    - Required path params: userId
    - Response type: `ListDraftsResponse`
  - `send` — Sends the specified, existing draft to the recipients in the `To`, `Cc`, and `Bcc` headers. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).
    - Required path params: userId
    - Request body type: `Draft`
    - Response type: `Message`
  - `update` — Replaces a draft's content. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).
    - Required path params: id, userId
    - Request body type: `Draft`
    - Response type: `Draft`

### users.history

  - `list` — Lists the history of all changes to the given mailbox. History results are returned in chronological order (increasing `historyId`). For more information, see [Synchronize clients with Gmail](https://developers.google.com/workspace/gmail/api/guides/sync).
    - Required path params: userId
    - Response type: `ListHistoryResponse`

### users.labels

  - `create` — Creates a label. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).
    - Required path params: userId
    - Request body type: `Label`
    - Response type: `Label`
  - `delete` — Immediately and permanently deletes the specified label and removes it from any messages and threads that it's applied to. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).
    - Required path params: id, userId
  - `get` — Gets the specified label. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).
    - Required path params: id, userId
    - Response type: `Label`
  - `list` — Lists all labels in the user's mailbox. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).
    - Required path params: userId
    - Response type: `ListLabelsResponse`
  - `patch` — Patch the specified label. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).
    - Required path params: id, userId
    - Request body type: `Label`
    - Response type: `Label`
  - `update` — Updates the specified label. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).
    - Required path params: id, userId
    - Request body type: `Label`
    - Response type: `Label`

### users.messages

  - `batchDelete` — Deletes many messages by message ID. Provides no guarantees that messages were not already deleted or even existed at all.
    - Required path params: userId
    - Request body type: `BatchDeleteMessagesRequest`
  - `batchModify` — Modifies the labels and the Classification Label values on the specified messages.
    - Required path params: userId
    - Request body type: `BatchModifyMessagesRequest`
  - `delete` — Immediately and permanently deletes the specified message. This operation cannot be undone. Prefer `messages.trash` instead.
    - Required path params: id, userId
  - `get` — Gets the specified message.
    - Required path params: id, userId
    - Response type: `Message`
  - `import` — Imports a message into only this user's mailbox, with standard email delivery scanning and classification similar to receiving via SMTP. This method doesn't perform SPF checks, so it might not work for some spam messages, such as those attempting to perform domain spoofing. This method does not send a message. Note that the maximum size of the message is 150 MB.
    - Required path params: userId
    - Request body type: `Message`
    - Response type: `Message`
  - `insert` — Directly inserts a message into only this user's mailbox similar to `IMAP APPEND`, bypassing most scanning and classification. Does not send a message. For more information, see [Create and send email messages](https://developers.google.com/workspace/gmail/api/guides/sending).
    - Required path params: userId
    - Request body type: `Message`
    - Response type: `Message`
  - `list` — Lists the messages in the user's mailbox. For more information, see [List Gmail messages](https://developers.google.com/workspace/gmail/api/guides/list-messages).
    - Required path params: userId
    - Response type: `ListMessagesResponse`
  - `modify` — Modifies the labels and the Classification Label values on the specified message.
    - Required path params: id, userId
    - Request body type: `ModifyMessageRequest`
    - Response type: `Message`
  - `send` — Sends the specified message to the recipients in the `To`, `Cc`, and `Bcc` headers. For more information, see [Create and send email messages](https://developers.google.com/workspace/gmail/api/guides/sending).
    - Required path params: userId
    - Request body type: `Message`
    - Response type: `Message`
  - `trash` — Moves the specified message to the trash.
    - Required path params: id, userId
    - Response type: `Message`
  - `untrash` — Removes the specified message from the trash.
    - Required path params: id, userId
    - Response type: `Message`

### users.messages.attachments

  - `get` — Gets the specified message attachment.
    - Required path params: id, messageId, userId
    - Response type: `MessagePartBody`

### users.settings

  - `getAutoForwarding` — Gets the auto-forwarding setting for the specified account. For more information, see [Manage email forwarding](https://developers.google.com/workspace/gmail/api/guides/forwarding_settings).
    - Required path params: userId
    - Response type: `AutoForwarding`
  - `getImap` — Gets IMAP settings. For more information, see [Configure POP and IMAP settings with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/pop_imap_settings).
    - Required path params: userId
    - Response type: `ImapSettings`
  - `getLanguage` — Gets language settings. For more information, see [Manage language settings](https://developers.google.com/workspace/gmail/api/guides/language-settings).
    - Required path params: userId
    - Response type: `LanguageSettings`
  - `getPop` — Gets POP settings. For more information, see [Configure POP and IMAP settings with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/pop_imap_settings).
    - Required path params: userId
    - Response type: `PopSettings`
  - `getVacation` — Gets vacation responder settings. For more information, see [Manage vacation settings with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/vacation_settings).
    - Required path params: userId
    - Response type: `VacationSettings`
  - `updateAutoForwarding` — Updates the auto-forwarding setting for the specified account. A verified forwarding address must be specified when auto-forwarding is enabled. For more information, see [Manage email forwarding](https://developers.google.com/workspace/gmail/api/guides/forwarding_settings). This method is only available to service account clients that have been delegated domain-wide authority.
    - Required path params: userId
    - Request body type: `AutoForwarding`
    - Response type: `AutoForwarding`
  - `updateImap` — Updates IMAP settings. For more information, see [Configure POP and IMAP settings with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/pop_imap_settings).
    - Required path params: userId
    - Request body type: `ImapSettings`
    - Response type: `ImapSettings`
  - `updateLanguage` — Updates language settings. For more information, see [Manage language settings](https://developers.google.com/workspace/gmail/api/guides/language-settings). If successful, the return object contains the `displayLanguage` that was saved for the user, which may differ from the value passed into the request. This is because the requested `displayLanguage` may not be directly supported by Gmail but have a close variant that is, and so the variant may be chosen and saved instead.
    - Required path params: userId
    - Request body type: `LanguageSettings`
    - Response type: `LanguageSettings`
  - `updatePop` — Updates POP settings. For more information, see [Configure POP and IMAP settings with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/pop_imap_settings).
    - Required path params: userId
    - Request body type: `PopSettings`
    - Response type: `PopSettings`
  - `updateVacation` — Updates vacation responder settings. For more information, see [Manage vacation settings with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/vacation_settings).
    - Required path params: userId
    - Request body type: `VacationSettings`
    - Response type: `VacationSettings`

### users.settings.cse.identities

  - `create` — Creates and configures a client-side encryption identity that's authorized to send mail from the user account. Google publishes the S/MIME certificate to a shared domain-wide directory so that people within a Google Workspace organization can encrypt and send mail to the identity.
    - Required path params: userId
    - Request body type: `CseIdentity`
    - Response type: `CseIdentity`
  - `delete` — Deletes a client-side encryption identity. The authenticated user can no longer use the identity to send encrypted messages. You cannot restore the identity after you delete it. Instead, use the CreateCseIdentity method to create another identity with the same configuration.
    - Required path params: cseEmailAddress, userId
  - `get` — Retrieves a client-side encryption identity configuration. For administrators managing identities and keypairs for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.settings.basic` scope.
    - Required path params: cseEmailAddress, userId
    - Response type: `CseIdentity`
  - `list` — Lists the client-side encrypted identities for an authenticated user.
    - Required path params: userId
    - Response type: `ListCseIdentitiesResponse`
  - `patch` — Associates a different key pair with an existing client-side encryption identity. The updated key pair must validate against Google's [S/MIME certificate profiles](https://support.google.com/a/answer/7300887).
    - Required path params: emailAddress, userId
    - Request body type: `CseIdentity`
    - Response type: `CseIdentity`

### users.settings.cse.keypairs

  - `create` — Creates and uploads a client-side encryption S/MIME public key certificate chain and private key metadata for the authenticated user.
    - Required path params: userId
    - Request body type: `CseKeyPair`
    - Response type: `CseKeyPair`
  - `disable` — Turns off a client-side encryption key pair. The authenticated user can no longer use the key pair to decrypt incoming CSE message texts or sign outgoing CSE mail. To regain access, use the EnableCseKeyPair to turn on the key pair. After 30 days, you can permanently delete the key pair by using the ObliterateCseKeyPair method.
    - Required path params: keyPairId, userId
    - Request body type: `DisableCseKeyPairRequest`
    - Response type: `CseKeyPair`
  - `enable` — Turns on a client-side encryption key pair that was turned off. The key pair becomes active again for any associated client-side encryption identities.
    - Required path params: keyPairId, userId
    - Request body type: `EnableCseKeyPairRequest`
    - Response type: `CseKeyPair`
  - `get` — Retrieves an existing client-side encryption key pair. For administrators managing identities and keypairs for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.settings.basic` scope.
    - Required path params: keyPairId, userId
    - Response type: `CseKeyPair`
  - `list` — Lists client-side encryption key pairs for an authenticated user.
    - Required path params: userId
    - Response type: `ListCseKeyPairsResponse`
  - `obliterate` — Deletes a client-side encryption key pair permanently and immediately. You can only permanently delete key pairs that have been turned off for more than 30 days. To turn off a key pair, use the DisableCseKeyPair method. Gmail can't restore or decrypt any messages that were encrypted by an obliterated key. Authenticated users and Google Workspace administrators lose access to reading the encrypted messages.
    - Required path params: keyPairId, userId
    - Request body type: `ObliterateCseKeyPairRequest`

### users.settings.delegates

  - `create` — Adds a delegate with its verification status set directly to `accepted`, without sending any verification email. The delegate user must be a member of the same Google Workspace organization as the delegator user. For more information, see [Manage delegates](https://developers.google.com/workspace/gmail/api/guides/delegate_settings). Gmail imposes limitations on the number of delegates and delegators each user in a Google Workspace organization can have.
    - Required path params: userId
    - Request body type: `Delegate`
    - Response type: `Delegate`
  - `delete` — Removes the specified delegate (which can be of any verification status), and revokes any verification that may have been required for using it. For more information, see [Manage delegates](https://developers.google.com/workspace/gmail/api/guides/delegate_settings). A delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority.
    - Required path params: delegateEmail, userId
  - `get` — Gets the specified delegate. For more information, see [Manage delegates](https://developers.google.com/workspace/gmail/api/guides/delegate_settings). A delegate user must be referred to by their primary email address, and not an email alias. This method is only available to service account clients that have been delegated domain-wide authority.
    - Required path params: delegateEmail, userId
    - Response type: `Delegate`
  - `list` — Lists the delegates for the specified account. For more information, see [Manage delegates](https://developers.google.com/workspace/gmail/api/guides/delegate_settings). This method is only available to service account clients that have been delegated domain-wide authority.
    - Required path params: userId
    - Response type: `ListDelegatesResponse`

### users.settings.filters

  - `create` — Creates a filter. Note: you can only create a maximum of 1,000 filters. For more information, see [Manage Gmail filters](https://developers.google.com/workspace/gmail/api/guides/filter_settings).
    - Required path params: userId
    - Request body type: `Filter`
    - Response type: `Filter`
  - `delete` — Immediately and permanently deletes the specified filter. For more information, see [Manage Gmail filters](https://developers.google.com/workspace/gmail/api/guides/filter_settings).
    - Required path params: id, userId
  - `get` — Gets a filter. For more information, see [Manage Gmail filters](https://developers.google.com/workspace/gmail/api/guides/filter_settings).
    - Required path params: id, userId
    - Response type: `Filter`
  - `list` — Lists the message filters of a Gmail user. For more information, see [Manage Gmail filters](https://developers.google.com/workspace/gmail/api/guides/filter_settings).
    - Required path params: userId
    - Response type: `ListFiltersResponse`

### users.settings.forwardingAddresses

  - `create` — Creates a forwarding address. If ownership verification is required, a message will be sent to the recipient and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`. For more information, see [Manage email forwarding](https://developers.google.com/workspace/gmail/api/guides/forwarding_settings). This method is only available to service account clients that have been delegated domain-wide authority.
    - Required path params: userId
    - Request body type: `ForwardingAddress`
    - Response type: `ForwardingAddress`
  - `delete` — Deletes the specified forwarding address and revokes any verification that may have been required. For more information, see [Manage email forwarding](https://developers.google.com/workspace/gmail/api/guides/forwarding_settings). This method is only available to service account clients that have been delegated domain-wide authority.
    - Required path params: forwardingEmail, userId
  - `get` — Gets the specified forwarding address. For more information, see [Manage email forwarding](https://developers.google.com/workspace/gmail/api/guides/forwarding_settings).
    - Required path params: forwardingEmail, userId
    - Response type: `ForwardingAddress`
  - `list` — Lists the forwarding addresses for the specified account. For more information, see [Manage email forwarding](https://developers.google.com/workspace/gmail/api/guides/forwarding_settings).
    - Required path params: userId
    - Response type: `ListForwardingAddressesResponse`

### users.settings.sendAs

  - `create` — Creates a custom "from" send-as alias. If an SMTP MSA is specified, Gmail will attempt to connect to the SMTP service to validate the configuration before creating the alias. If ownership verification is required for the alias, a message will be sent to the email address and the resource's verification status will be set to `pending`; otherwise, the resource will be created with verification status set to `accepted`.
    - Required path params: userId
    - Request body type: `SendAs`
    - Response type: `SendAs`
  - `delete` — Deletes the specified send-as alias. Revokes any verification that may have been required for using it. For more information, see [Manage aliases and signatures with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/alias_and_signature_settings). This method is only available to service account clients that have been delegated domain-wide authority.
    - Required path params: sendAsEmail, userId
  - `get` — Gets the specified send-as alias. Fails with an HTTP 404 error if the specified address is not a member of the collection. For more information, see [Manage aliases and signatures with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/alias_and_signature_settings).
    - Required path params: sendAsEmail, userId
    - Response type: `SendAs`
  - `list` — Lists the send-as aliases for the specified account. The result includes the primary send-as address associated with the account as well as any custom "from" aliases. For more information, see [Manage aliases and signatures with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/alias_and_signature_settings).
    - Required path params: userId
    - Response type: `ListSendAsResponse`
  - `patch` — Patch the specified send-as alias. For more information, see [Manage aliases and signatures with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/alias_and_signature_settings).
    - Required path params: sendAsEmail, userId
    - Request body type: `SendAs`
    - Response type: `SendAs`
  - `update` — Updates a send-as alias. If a signature is provided, Gmail will sanitize the HTML before saving it with the alias. For more information, see [Manage aliases and signatures with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/alias_and_signature_settings). Addresses other than the primary address for the account can only be updated by service account clients that have been delegated domain-wide authority.
    - Required path params: sendAsEmail, userId
    - Request body type: `SendAs`
    - Response type: `SendAs`
  - `verify` — Sends a verification email to the specified send-as alias address. The verification status must be `pending`. For more information, see [Manage aliases and signatures with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/alias_and_signature_settings). This method is only available to service account clients that have been delegated domain-wide authority.
    - Required path params: sendAsEmail, userId

### users.settings.sendAs.smimeInfo

  - `delete` — Deletes the specified S/MIME config for the specified send-as alias. For more information, see [Manage S/MIME certificates with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/smime_certs).
    - Required path params: id, sendAsEmail, userId
  - `get` — Gets the specified S/MIME config for the specified send-as alias. For more information, see [Manage S/MIME certificates with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/smime_certs).
    - Required path params: id, sendAsEmail, userId
    - Response type: `SmimeInfo`
  - `insert` — Insert (upload) the given S/MIME config for the specified send-as alias. Note that `pkcs12` format is required for the key. For more information, see [Manage S/MIME certificates with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/smime_certs).
    - Required path params: sendAsEmail, userId
    - Request body type: `SmimeInfo`
    - Response type: `SmimeInfo`
  - `list` — Lists S/MIME configs for the specified send-as alias. For more information, see [Manage S/MIME certificates with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/smime_certs).
    - Required path params: sendAsEmail, userId
    - Response type: `ListSmimeInfoResponse`
  - `setDefault` — Sets the default S/MIME config for the specified send-as alias. For more information, see [Manage S/MIME certificates with the Gmail API](https://developers.google.com/workspace/gmail/api/guides/smime_certs).
    - Required path params: id, sendAsEmail, userId

### users.threads

  - `delete` — Immediately and permanently deletes the specified thread. Any messages that belong to the thread are also deleted. This operation cannot be undone. Prefer `threads.trash` instead. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).
    - Required path params: id, userId
  - `get` — Gets the specified thread. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).
    - Required path params: id, userId
    - Response type: `Thread`
  - `list` — Lists the threads in the user's mailbox. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).
    - Required path params: userId
    - Response type: `ListThreadsResponse`
  - `modify` — Modifies the labels applied to the thread. This applies to all messages in the thread. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).
    - Required path params: id, userId
    - Request body type: `ModifyThreadRequest`
    - Response type: `Thread`
  - `trash` — Moves the specified thread to the trash. Any messages that belong to the thread are also moved to the trash. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).
    - Required path params: id, userId
    - Response type: `Thread`
  - `untrash` — Removes the specified thread from the trash. Any messages that belong to the thread are also removed from the trash. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).
    - Required path params: id, userId
    - Response type: `Thread`

## Common Schemas

### AutoForwarding

*Description: Auto-forwarding settings for an account.*

| Field | Type | Description |
|---|---|---|
| `disposition` | string | The state that a message should be left in after it has been forwarded. |
| `emailAddress` | string | Email address to which all incoming messages are forwarded. This email address must be a verified member of the forwarding addresses. |
| `enabled` | boolean | Whether all incoming mail is automatically forwarded to another address. |

### BatchDeleteMessagesRequest

| Field | Type | Description |
|---|---|---|
| `ids` | array of string | The IDs of the messages to delete. |

### BatchModifyMessagesRequest

| Field | Type | Description |
|---|---|---|
| `addClassificationLabels` | array of `ClassificationLabelValue` | A list of Classification Label values to add. |
| `addLabelIds` | array of string | A list of label IDs to add to messages. |
| `ids` | array of string | The IDs of the messages to modify. There is a limit of 1000 ids per request. |
| `removeClassificationLabelIds` | array of string | A list of Classification Label values to remove from messages. |
| `removeLabelIds` | array of string | A list of label IDs to remove from messages. |

### CseIdentity

*Description: The client-side encryption (CSE) configuration for the email address of an authenticated user. Gmail uses CSE configurations to save drafts of client-side encrypted email messages, and to sign and send encrypted email messages.*

| Field | Type | Description |
|---|---|---|
| `emailAddress` | string | The email address for the sending identity. The email address must be the primary email address of the authenticated user. |
| `primaryKeyPairId` | string | If a key pair is associated, the ID of the key pair, CseKeyPair. |
| `signAndEncryptKeyPairs` | `SignAndEncryptKeyPairs` | The configuration of a CSE identity that uses different key pairs for signing and encryption. |

### CseKeyPair

*Description: A client-side encryption S/MIME key pair, which is comprised of a public key, its certificate chain, and metadata for its paired private key. Gmail uses the key pair to complete the following tasks: - Sign outgoing client-side encrypted messages.*

| Field | Type | Description |
|---|---|---|
| `disableTime` | string (format: google-datetime) | Output only. If a key pair is set to `DISABLED`, the time that the key pair's state changed from `ENABLED` to `DISABLED`. This field is present only when the key pair is in state `DISABLED`. |
| `enablementState` | string | Output only. The current state of the key pair. |
| `keyPairId` | string | Output only. The immutable ID for the client-side encryption S/MIME key pair. |
| `pem` | string | Output only. The public key and its certificate chain, in [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail) format. |
| `pkcs7` | string | Input only. The public key and its certificate chain. The chain must be in [PKCS#7](https://en.wikipedia.org/wiki/PKCS_7) format and use PEM encoding and ASCII armor. |
| `privateKeyMetadata` | array of `CsePrivateKeyMetadata` | Metadata for instances of this key pair's private key. |
| `subjectEmailAddresses` | array of string | Output only. The email address identities that are specified on the leaf certificate. |

### Delegate

*Description: Settings for a delegate. Delegates can read, send, and delete messages, as well as view and add contacts, for the delegator's account. See "Set up mail delegation" for more information about delegates.*

| Field | Type | Description |
|---|---|---|
| `delegateEmail` | string | The email address of the delegate. |
| `verificationStatus` | string | Indicates whether this address has been verified and can act as a delegate for the account. Read-only. |

### DisableCseKeyPairRequest

*Description: Requests to turn off a client-side encryption key pair.*

*(No fields)*

### Draft

*Description: A draft email in the user's mailbox.*

| Field | Type | Description |
|---|---|---|
| `id` | string | The immutable ID of the draft. |
| `message` | `Message` | The message content of the draft. |

### EnableCseKeyPairRequest

*Description: Requests to turn on a client-side encryption key pair.*

*(No fields)*

### Filter

*Description: Resource definition for Gmail filters. Filters apply to specific messages instead of an entire email thread.*

| Field | Type | Description |
|---|---|---|
| `action` | `FilterAction` | Action that the filter performs. |
| `criteria` | `FilterCriteria` | Matching criteria for the filter. |
| `id` | string | The server assigned ID of the filter. |

### ForwardingAddress

*Description: Settings for a forwarding address.*

| Field | Type | Description |
|---|---|---|
| `forwardingEmail` | string | An email address to which messages can be forwarded. |
| `verificationStatus` | string | Indicates whether this address has been verified and is usable for forwarding. Read-only. |

### ImapSettings

*Description: IMAP settings for an account.*

| Field | Type | Description |
|---|---|---|
| `autoExpunge` | boolean | If this value is true, Gmail will immediately expunge a message when it is marked as deleted in IMAP. |
| `enabled` | boolean | Whether IMAP is enabled for the account. |
| `expungeBehavior` | string | The action that will be executed on a message when it is marked as deleted and expunged from the last visible IMAP folder. |
| `maxFolderSize` | integer (format: int32) | An optional limit on the number of messages that an IMAP folder may contain. Legal values are 0, 1000, 2000, 5000 or 10000. A value of zero is interpreted to mean that there is no limit. |

### Label

*Description: Labels are used to categorize messages and threads within the user's mailbox. The maximum number of labels supported for a user's mailbox is 10,000.*

| Field | Type | Description |
|---|---|---|
| `color` | `LabelColor` | The color to assign to the label. Color is only available for labels that have their `type` set to `user`. |
| `id` | string | The immutable ID of the label. |
| `labelListVisibility` | string | The visibility of the label in the label list in the Gmail web interface. |
| `messageListVisibility` | string | The visibility of messages with this label in the message list in the Gmail web interface. |
| `messagesTotal` | integer (format: int32) | The total number of messages with the label. |
| `messagesUnread` | integer (format: int32) | The number of unread messages with the label. |
| `name` | string | The display name of the label. |
| `threadsTotal` | integer (format: int32) | The total number of threads with the label. |
| `threadsUnread` | integer (format: int32) | The number of unread threads with the label. |
| `type` | string | The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. |

### LanguageSettings

*Description: Language settings for an account. These settings correspond to the "Language settings" feature in the web interface.*

| Field | Type | Description |
|---|---|---|
| `displayLanguage` | string | The language to display Gmail in, formatted as an RFC 3066 Language Tag (for example `en-GB`, `fr` or `ja` for British English, French, or Japanese respectively). |

### ListCseIdentitiesResponse

| Field | Type | Description |
|---|---|---|
| `cseIdentities` | array of `CseIdentity` | One page of the list of CSE identities configured for the user. |
| `nextPageToken` | string | Pagination token to be passed to a subsequent ListCseIdentities call in order to retrieve the next page of identities. |

### ListCseKeyPairsResponse

| Field | Type | Description |
|---|---|---|
| `cseKeyPairs` | array of `CseKeyPair` | One page of the list of CSE key pairs installed for the user. |
| `nextPageToken` | string | Pagination token to be passed to a subsequent ListCseKeyPairs call in order to retrieve the next page of key pairs. If this value is not returned, then no further pages remain. |

### ListDelegatesResponse

*Description: Response for the ListDelegates method.*

| Field | Type | Description |
|---|---|---|
| `delegates` | array of `Delegate` | List of the user's delegates (with any verification status). If an account doesn't have delegates, this field doesn't appear. |

### ListDraftsResponse

| Field | Type | Description |
|---|---|---|
| `drafts` | array of `Draft` | List of drafts. Note that the `Message` property in each `Draft` resource only contains an `id` and a `threadId`. |
| `nextPageToken` | string | Token to retrieve the next page of results in the list. |
| `resultSizeEstimate` | integer (format: uint32) | Estimated total number of results. |

### ListFiltersResponse

*Description: Response for the ListFilters method.*

| Field | Type | Description |
|---|---|---|
| `filter` | array of `Filter` | List of a user's filters. |

### ListForwardingAddressesResponse

*Description: Response for the ListForwardingAddresses method.*

| Field | Type | Description |
|---|---|---|
| `forwardingAddresses` | array of `ForwardingAddress` | List of addresses that may be used for forwarding. |

### ListHistoryResponse

| Field | Type | Description |
|---|---|---|
| `history` | array of `History` | List of history records. Any `messages` contained in the response will typically only have `id` and `threadId` fields populated. |
| `historyId` | string (format: uint64) | The ID of the mailbox's current history record. |
| `nextPageToken` | string | Page token to retrieve the next page of results in the list. |

### ListLabelsResponse

| Field | Type | Description |
|---|---|---|
| `labels` | array of `Label` | List of labels. Note that each label resource only contains an `id`, `name`, `messageListVisibility`, `labelListVisibility`, and `type`. |

### ListMessagesResponse

| Field | Type | Description |
|---|---|---|
| `messages` | array of `Message` | List of messages. Note that each message resource contains only an `id` and a `threadId`. Additional message details can be fetched using the messages.get method. |
| `nextPageToken` | string | Token to retrieve the next page of results in the list. |
| `resultSizeEstimate` | integer (format: uint32) | Estimated total number of results. |

### ListSendAsResponse

*Description: Response for the ListSendAs method.*

| Field | Type | Description |
|---|---|---|
| `sendAs` | array of `SendAs` | List of send-as aliases. |

### ListSmimeInfoResponse

| Field | Type | Description |
|---|---|---|
| `smimeInfo` | array of `SmimeInfo` | List of SmimeInfo. |

### ListThreadsResponse

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Page token to retrieve the next page of results in the list. |
| `resultSizeEstimate` | integer (format: uint32) | Estimated total number of results. |
| `threads` | array of `Thread` | List of threads. Note that each thread resource does not contain a list of `messages`. |

### Message

*Description: An email message.*

| Field | Type | Description |
|---|---|---|
| `classificationLabelValues` | array of `ClassificationLabelValue` | Classification Label values on the message. Available Classification Label schemas can be queried using the Google Drive Labels API. Each classification label ID must be unique. |
| `historyId` | string (format: uint64) | The ID of the last history record that modified this message. |
| `id` | string | The immutable ID of the message. |
| `internalDate` | string (format: int64) | The internal message creation timestamp (epoch ms), which determines ordering in the inbox. |
| `labelIds` | array of string | List of IDs of labels applied to this message. |
| `payload` | `MessagePart` | The parsed email structure in the message parts. |
| `raw` | string (format: byte) | The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. |
| `sizeEstimate` | integer (format: int32) | Estimated size in bytes of the message. |
| `snippet` | string | A short part of the message text. |
| `threadId` | string | The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. |

### MessagePartBody

*Description: The body of a single MIME message part.*

| Field | Type | Description |
|---|---|---|
| `attachmentId` | string | When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. |
| `data` | string (format: byte) | The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. |
| `size` | integer (format: int32) | Number of bytes for the message part data (encoding notwithstanding). |

### ModifyMessageRequest

| Field | Type | Description |
|---|---|---|
| `addClassificationLabels` | array of `ClassificationLabelValue` | A list of classification label values to add. |
| `addLabelIds` | array of string | A list of IDs of labels to add to this message. You can add up to 100 labels with each update. |
| `removeClassificationLabelIds` | array of string | A list of Classification Label values to remove from this message. |
| `removeLabelIds` | array of string | A list IDs of labels to remove from this message. You can remove up to 100 labels with each update. |

### ModifyThreadRequest

| Field | Type | Description |
|---|---|---|
| `addLabelIds` | array of string | A list of IDs of labels to add to this thread. You can add up to 100 labels with each update. |
| `removeLabelIds` | array of string | A list of IDs of labels to remove from this thread. You can remove up to 100 labels with each update. |

### ObliterateCseKeyPairRequest

*Description: Request to obliterate a CSE key pair.*

*(No fields)*

### PopSettings

*Description: POP settings for an account.*

| Field | Type | Description |
|---|---|---|
| `accessWindow` | string | The range of messages which are accessible via POP. |
| `disposition` | string | The action that will be executed on a message after it has been fetched via POP. |

### Profile

*Description: Profile for a Gmail user.*

| Field | Type | Description |
|---|---|---|
| `emailAddress` | string | The user's email address. |
| `historyId` | string (format: uint64) | The ID of the mailbox's current history record. |
| `messagesTotal` | integer (format: int32) | The total number of messages in the mailbox. |
| `threadsTotal` | integer (format: int32) | The total number of threads in the mailbox. |

### SendAs

*Description: Settings associated with a send-as alias, which can be either the primary login address associated with the account or a custom "from" address. Send-as aliases correspond to the "Send Mail As" feature in the web interface. The send-as alias must be a valid email address.*

| Field | Type | Description |
|---|---|---|
| `displayName` | string | A name that appears in the "From:" header for mail sent using this alias. |
| `isDefault` | boolean | Whether this address is selected as the default "From:" address in situations such as composing a new message or sending a vacation auto-reply. |
| `isPrimary` | boolean | Whether this address is the primary address used to login to the account. Every Gmail account has exactly one primary address, and it cannot be deleted from the collection of send-as aliases. |
| `replyToAddress` | string | An optional email address that is included in a "Reply-To:" header for mail sent using this alias. If this is empty, Gmail will not generate a "Reply-To:" header. |
| `sendAsEmail` | string | The email address that appears in the "From:" header for mail sent using this alias. This is read-only for all operations except create. |
| `signature` | string | An optional HTML signature that is included in messages composed with this alias in the Gmail web UI. This signature is added to new emails only. |
| `smtpMsa` | `SmtpMsa` | An optional SMTP service that will be used as an outbound relay for mail sent using this alias. |
| `treatAsAlias` | boolean | Whether Gmail should treat this address as an alias for the user's primary email address. This setting only applies to custom "from" aliases. |
| `verificationStatus` | string | Indicates whether this address has been verified for use as a send-as alias. Read-only. This setting only applies to custom "from" aliases. |

### SmimeInfo

*Description: An S/MIME email config.*

| Field | Type | Description |
|---|---|---|
| `encryptedKeyPassword` | string | Encrypted key password, when key is encrypted. |
| `expiration` | string (format: int64) | When the certificate expires (in milliseconds since epoch). |
| `id` | string | The immutable ID for the SmimeInfo. |
| `isDefault` | boolean | Whether this SmimeInfo is the default one for this user's send-as address. |
| `issuerCn` | string | The S/MIME certificate issuer's common name. |
| `pem` | string | PEM formatted X509 concatenated certificate string (standard base64 encoding). Format used for returning key, which includes public key as well as certificate chain (not private key). |
| `pkcs12` | string (format: byte) | PKCS#12 format containing a single private/public key pair and certificate chain. |

### Thread

*Description: A collection of messages representing a conversation.*

| Field | Type | Description |
|---|---|---|
| `historyId` | string (format: uint64) | The ID of the last history record that modified this thread. |
| `id` | string | The unique ID of the thread. |
| `messages` | array of `Message` | The list of messages in the thread. |
| `snippet` | string | A short part of the message text. |

### VacationSettings

*Description: Vacation auto-reply settings for an account. These settings correspond to the "Vacation responder" feature in the web interface.*

| Field | Type | Description |
|---|---|---|
| `enableAutoReply` | boolean | Flag that controls whether Gmail automatically replies to messages. |
| `endTime` | string (format: int64) | An optional end time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives before the end time. |
| `responseBodyHtml` | string | Response body in HTML format. Gmail will sanitize the HTML before storing it. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used. |
| `responseBodyPlainText` | string | Response body in plain text format. If both `response_body_plain_text` and `response_body_html` are specified, `response_body_html` will be used. |
| `responseSubject` | string | Optional text to prepend to the subject line in vacation responses. In order to enable auto-replies, either the response subject or the response body must be nonempty. |
| `restrictToContacts` | boolean | Flag that determines whether responses are sent to recipients who are not in the user's list of contacts. |
| `restrictToDomain` | boolean | Flag that determines whether responses are sent to recipients who are outside of the user's domain. This feature is only available for Google Workspace users. |
| `startTime` | string (format: int64) | An optional start time for sending auto-replies (epoch ms). When this is specified, Gmail will automatically reply only to messages that it receives after the start time. |

### WatchRequest

*Description: Set up or update a new push notification watch on this user's mailbox.*

| Field | Type | Description |
|---|---|---|
| `labelFilterAction` | string | Filtering behavior of `labelIds list` specified. This field is deprecated because it caused incorrect behavior in some cases; use `label_filter_behavior` instead. |
| `labelFilterBehavior` | string | Filtering behavior of `labelIds list` specified. This field replaces `label_filter_action`; if set, `label_filter_action` is ignored. |
| `labelIds` | array of string | List of label_ids to restrict notifications about. By default, if unspecified, all changes are pushed out. If specified then dictates which labels are required for a push notification to be generated. |
| `topicName` | string | A fully qualified Google Cloud Pub/Sub API topic name to publish the events to. |

### WatchResponse

*Description: Push notification watch response.*

| Field | Type | Description |
|---|---|---|
| `expiration` | string (format: int64) | When Gmail will stop sending notifications for mailbox updates (epoch millis). Call `watch` again before this time to renew the watch. |
| `historyId` | string (format: uint64) | The ID of the mailbox's current history record. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws gmail --help

# Inspect a method's required params, types, and defaults
gws schema gmail.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

