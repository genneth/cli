---
name: gws-admin-reports
description: "Google Workspace Admin SDK: Audit logs and usage reports."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws admin-reports --help"
---

# admin-reports (reports_v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws admin-reports <resource> <method> [flags]
```

## API Resources

### activities

  - `list` — Retrieves a list of activities for a specific customer's account and application such as the Admin console application or the Google Drive application. For more information, see the guides for administrator and Google Drive activity reports. For more information about the activity report's parameters, see the activity parameters reference guides.
    - Required path params: applicationName, userKey
    - Response type: `Activities`
  - `watch` — Start receiving notifications for account activities. For more information, see Receiving Push Notifications.
    - Required path params: applicationName, userKey
    - Request body type: `Channel`
    - Response type: `Channel`

### channels

  - `stop` — Stop watching resources through this channel.
    - Request body type: `Channel`

### customerUsageReports

  - `get` — Retrieves a report which is a collection of properties and statistics for a specific customer's account. For more information, see the Customers Usage Report guide. For more information about the customer report's parameters, see the Customers Usage parameters reference guides.
    - Required path params: date
    - Response type: `UsageReports`

### entityUsageReports

  - `get` — Retrieves a report which is a collection of properties and statistics for entities used by users within the account. For more information, see the Entities Usage Report guide. For more information about the entities report's parameters, see the Entities Usage parameters reference guides.
    - Required path params: date, entityKey, entityType
    - Response type: `UsageReports`

### userUsageReport

  - `get` — Retrieves a report which is a collection of properties and statistics for a set of users with the account. For more information, see the User Usage Report guide. For more information about the user report's parameters, see the Users Usage parameters reference guides.
    - Required path params: date, userKey
    - Response type: `UsageReports`

## Common Schemas

### Activities

*Description: JSON template for a collection of activities.*

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the resource. |
| `items` | array of `Activity` | Each activity record in the response. |
| `kind` | string | The type of API resource. For an activity report, the value is `reports#activities`. |
| `nextPageToken` | string | Token for retrieving the follow-on next page of the report. The `nextPageToken` value is used in the request's `pageToken` query string. |

### Channel

*Description: A notification channel used to watch for resource changes.*

| Field | Type | Description |
|---|---|---|
| `address` | string | The address where notifications are delivered for this channel. |
| `expiration` | string (format: int64) | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `id` | string | A UUID or similar unique string that identifies this channel. |
| `kind` | string | Identifies this as a notification channel used to watch for changes to a resource, which is "`api#channel`". |
| `params` | object | Additional parameters controlling delivery channel behavior. Optional. |
| `payload` | boolean | A Boolean value to indicate whether payload is wanted. A payload is data that is sent in the body of an HTTP POST, PUT, or PATCH message and contains important information about the request. Optional. |
| `resourceId` | string | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `resourceUri` | string | A version-specific identifier for the watched resource. |
| `token` | string | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | string | The type of delivery mechanism used for this channel. The value should be set to `"web_hook"`. |

### UsageReports

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the resource. |
| `kind` | string | The type of API resource. For a usage report, the value is `admin#reports#usageReports`. |
| `nextPageToken` | string | Token to specify next page. A report with multiple pages has a `nextPageToken` property in the response. |
| `usageReports` | array of `UsageReport` | Various application parameter records. |
| `warnings` | array of object | Warnings, if any. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws admin-reports --help

# Inspect a method's required params, types, and defaults
gws schema admin-reports.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

