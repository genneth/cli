---
name: gws-events
description: "Subscribe to Google Workspace events."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws events --help"
---

# events (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws events <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+subscribe`](../gws-events-subscribe/SKILL.md) | Subscribe to Workspace events and stream them as NDJSON |
| [`+renew`](../gws-events-renew/SKILL.md) | Renew/reactivate Workspace Events subscriptions |

## API Resources

### message

  - `stream` — SendStreamingMessage is a streaming call that will return a stream of task update events until the Task is in an interrupted or terminal state.
    - Request body type: `SendMessageRequest`
    - Response type: `StreamResponse`

### operations

  - `get` — Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.
    - Required path params: name
    - Response type: `Operation`

### subscriptions

  - `create` — Creates a Google Workspace subscription. To learn how to use this method, see [Create a Google Workspace subscription](https://developers.google.com/workspace/events/guides/create-subscription). For a subscription on a [Chat target resource](https://developers.google.com/workspace/events/guides/events-chat), you can create a subscription as: - A Chat app by specifying an authorization scope that begins with `chat.app` and getting one-time administrator approval.
    - Request body type: `Subscription`
    - Response type: `Operation`
  - `delete` — Deletes a Google Workspace subscription. To learn how to use this method, see [Delete a Google Workspace subscription](https://developers.google.com/workspace/events/guides/delete-subscription).
    - Required path params: name
    - Response type: `Operation`
  - `get` — Gets details about a Google Workspace subscription. To learn how to use this method, see [Get details about a Google Workspace subscription](https://developers.google.com/workspace/events/guides/get-subscription).
    - Required path params: name
    - Response type: `Subscription`
  - `list` — Lists Google Workspace subscriptions. To learn how to use this method, see [List Google Workspace subscriptions](https://developers.google.com/workspace/events/guides/list-subscriptions).
    - Response type: `ListSubscriptionsResponse`
  - `patch` — Updates or renews a Google Workspace subscription. To learn how to use this method, see [Update or renew a Google Workspace subscription](https://developers.google.com/workspace/events/guides/update-subscription). For a subscription on a [Chat target resource](https://developers.google.com/workspace/events/guides/events-chat), you can update a subscription as: - A Chat app by specifying an authorization scope that begins with `chat.app` and getting one-time administrator approval.
    - Required path params: name
    - Request body type: `Subscription`
    - Response type: `Operation`
  - `reactivate` — Reactivates a suspended Google Workspace subscription. This method resets your subscription's `State` field to `ACTIVE`. Before you use this method, you must fix the error that suspended the subscription. This method will ignore or reject any subscription that isn't currently in a suspended state. To learn how to use this method, see [Reactivate a Google Workspace subscription](https://developers.google.com/workspace/events/guides/reactivate-subscription).
    - Required path params: name
    - Request body type: `ReactivateSubscriptionRequest`
    - Response type: `Operation`

### tasks

  - `cancel` — Cancel a task from the agent. If supported one should expect no more task updates for the task.
    - Required path params: name
    - Request body type: `CancelTaskRequest`
    - Response type: `Task`
  - `get` — Get the current state of a task from the agent.
    - Required path params: name
    - Response type: `Task`
  - `subscribe` — TaskSubscription is a streaming call that will return a stream of task update events. This attaches the stream to an existing in process task. If the task is complete the stream will return the completed task (like GetTask) and close the stream.
    - Required path params: name
    - Response type: `StreamResponse`

### tasks.pushNotificationConfigs

  - `create` — Set a push notification config for a task.
    - Required path params: parent
    - Request body type: `TaskPushNotificationConfig`
    - Response type: `TaskPushNotificationConfig`
  - `delete` — Delete a push notification config for a task.
    - Required path params: name
    - Response type: `Empty`
  - `get` — Get a push notification config for a task.
    - Required path params: name
    - Response type: `TaskPushNotificationConfig`
  - `list` — Get a list of push notifications configured for a task.
    - Required path params: parent
    - Response type: `ListTaskPushNotificationConfigResponse`

## Common Schemas

### CancelTaskRequest

| Field | Type | Description |
|---|---|---|
| `tenant` | string | Optional tenant, provided as a path parameter. Experimental, might still change for 1.0 release. |

### Empty

*Description: A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }*

*(No fields)*

### ListSubscriptionsResponse

*Description: The response message for SubscriptionsService.ListSubscriptions.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `subscriptions` | array of `Subscription` | List of subscriptions. |

### ListTaskPushNotificationConfigResponse

| Field | Type | Description |
|---|---|---|
| `configs` | array of `TaskPushNotificationConfig` | The list of push notification configurations. |
| `nextPageToken` | string | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |

### Operation

*Description: This resource represents a long-running operation that is the result of a network API call.*

| Field | Type | Description |
|---|---|---|
| `done` | boolean | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | `Status` | The error result of the operation in case of failure or cancellation. |
| `metadata` | object | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. |
| `name` | string | The server-assigned name, which is only unique within the same service that originally returns it. |
| `response` | object | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. |

### ReactivateSubscriptionRequest

*Description: The request message for SubscriptionsService.ReactivateSubscription.*

*(No fields)*

### SendMessageRequest

*Description: /////////// Request Messages ///////////*

| Field | Type | Description |
|---|---|---|
| `configuration` | `SendMessageConfiguration` | Configuration for the send request. |
| `message` | `Message` | Required. The message to send to the agent. |
| `metadata` | object | Optional metadata for the request. |
| `tenant` | string | Optional tenant, provided as a path parameter. Experimental, might still change for 1.0 release. |

### StreamResponse

*Description: The stream response for a message.*

| Field | Type | Description |
|---|---|---|
| `artifactUpdate` | `TaskArtifactUpdateEvent` | — |
| `message` | `Message` | — |
| `statusUpdate` | `TaskStatusUpdateEvent` | — |
| `task` | `Task` | — |

### Subscription

*Description: A subscription to receive events about a Google Workspace resource. To learn more about subscriptions, see the [Google Workspace Events API overview](https://developers.google.com/workspace/events).*

| Field | Type | Description |
|---|---|---|
| `authority` | string | Output only. The user who authorized the creation of the subscription. |
| `createTime` | string (format: google-datetime) | Output only. The time when the subscription is created. |
| `driveOptions` | `DriveOptions` | Optional. Features that are supported only for subscriptions on Drive resources. |
| `etag` | string | Optional. This checksum is computed by the server based on the value of other fields, and might be sent on update requests to ensure the client has an up-to-date value before proceeding. |
| `eventTypes` | array of string | Required. Unordered list. Input for creating a subscription. Otherwise, output only. One or more types of events to receive about the target resource. |
| `expireTime` | string (format: google-datetime) | Non-empty default. The timestamp in UTC when the subscription expires. Always displayed on output, regardless of what was used on input. |
| `name` | string | Identifier. Resource name of the subscription. Format: `subscriptions/{subscription}` |
| `notificationEndpoint` | `NotificationEndpoint` | Required. Immutable. The endpoint where the subscription delivers events, such as a Pub/Sub topic. |
| `payloadOptions` | `PayloadOptions` | Optional. Options about what data to include in the event payload. Only supported for Google Chat and Google Drive events. |
| `reconciling` | boolean | Output only. If `true`, the subscription is in the process of being updated. |
| `serviceAccountAuthority` | string | Output only. The service account that was used to authorize the creation of the subscription. This service account must be owned by the same Google Cloud project where you created this subscription. |
| `state` | string | Output only. The state of the subscription. Determines whether the subscription can receive events and deliver them to the notification endpoint. |
| `suspensionReason` | string | Output only. The error that suspended the subscription. To reactivate the subscription, resolve the error and call the `ReactivateSubscription` method. |
| `targetResource` | string | Required. Immutable. The Google Workspace resource that's monitored for events, formatted as the [full resource name](https://google.aip.dev/122#full-resource-names). |
| `ttl` | string (format: google-duration) | Input only. The time-to-live (TTL) or duration for the subscription. If unspecified or set to `0`, uses the maximum possible duration. |
| `uid` | string | Output only. System-assigned unique identifier for the subscription. |
| `updateTime` | string (format: google-datetime) | Output only. The last time that the subscription is updated. |
| `userAuthority` | string | Output only. The user who authorized the creation of the subscription. The user must be able to view the `target_resource`. |

### Task

*Description: Task is the core unit of action for A2A. It has a current status and when results are created for the task they are stored in the artifact. If there are multiple turns for a task, these are stored in history.*

| Field | Type | Description |
|---|---|---|
| `artifacts` | array of `Artifact` | A set of output artifacts for a Task. |
| `contextId` | string | Unique identifier (e.g. UUID) for the contextual collection of interactions (tasks and messages). Created by the A2A server. |
| `history` | array of `Message` | protolint:disable REPEATED_FIELD_NAMES_PLURALIZED The history of interactions from a task. |
| `id` | string | Unique identifier (e.g. UUID) for the task, generated by the server for a new task. |
| `metadata` | object | protolint:enable REPEATED_FIELD_NAMES_PLURALIZED A key/value object to store custom metadata about a task. |
| `status` | `TaskStatus` | The current status of a Task, including state and a message. |

### TaskPushNotificationConfig

| Field | Type | Description |
|---|---|---|
| `name` | string | The resource name of the config. Format: tasks/{task_id}/pushNotificationConfigs/{config_id} |
| `pushNotificationConfig` | `PushNotificationConfig` | The push notification configuration details. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws events --help

# Inspect a method's required params, types, and defaults
gws schema events.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

