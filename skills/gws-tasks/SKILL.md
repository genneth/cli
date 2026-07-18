---
name: gws-tasks
description: "Google Tasks: Manage task lists and tasks."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws tasks --help"
---

# tasks (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws tasks <resource> <method> [flags]
```

## API Resources

### tasklists

  - `delete` — Deletes the authenticated user's specified task list. If the list contains assigned tasks, both the assigned tasks and the original tasks in the assignment surface (Docs, Chat Spaces) are deleted.
    - Required path params: tasklist
  - `get` — Returns the authenticated user's specified task list.
    - Required path params: tasklist
    - Response type: `TaskList`
  - `insert` — Creates a new task list and adds it to the authenticated user's task lists. A user can have up to 2000 lists at a time.
    - Request body type: `TaskList`
    - Response type: `TaskList`
  - `list` — Returns all the authenticated user's task lists. A user can have up to 2000 lists at a time.
    - Response type: `TaskLists`
  - `patch` — Updates the authenticated user's specified task list. This method supports patch semantics.
    - Required path params: tasklist
    - Request body type: `TaskList`
    - Response type: `TaskList`
  - `update` — Updates the authenticated user's specified task list.
    - Required path params: tasklist
    - Request body type: `TaskList`
    - Response type: `TaskList`

### tasks

  - `clear` — Clears all completed tasks from the specified task list. The affected tasks will be marked as 'hidden' and no longer be returned by default when retrieving all tasks for a task list.
    - Required path params: tasklist
  - `delete` — Deletes the specified task from the task list. If the task is assigned, both the assigned task and the original task (in Docs, Chat Spaces) are deleted. To delete the assigned task only, navigate to the assignment surface and unassign the task from there.
    - Required path params: task, tasklist
  - `get` — Returns the specified task.
    - Required path params: task, tasklist
    - Response type: `Task`
  - `insert` — Creates a new task on the specified task list. Tasks assigned from Docs or Chat Spaces cannot be inserted from Tasks Public API; they can only be created by assigning them from Docs or Chat Spaces. A user can have up to 20,000 non-hidden tasks per list and up to 100,000 tasks in total at a time.
    - Required path params: tasklist
    - Request body type: `Task`
    - Response type: `Task`
  - `list` — Returns all tasks in the specified task list. Doesn't return assigned tasks by default (from Docs, Chat Spaces). A user can have up to 20,000 non-hidden tasks per list and up to 100,000 tasks in total at a time.
    - Required path params: tasklist
    - Response type: `Tasks`
  - `move` — Moves the specified task to another position in the destination task list. If the destination list is not specified, the task is moved within its current list. This can include putting it as a child task under a new parent and/or move it to a different position among its sibling tasks. A user can have up to 2,000 subtasks per task.
    - Required path params: task, tasklist
    - Response type: `Task`
  - `patch` — Updates the specified task. This method supports patch semantics.
    - Required path params: task, tasklist
    - Request body type: `Task`
    - Response type: `Task`
  - `update` — Updates the specified task.
    - Required path params: task, tasklist
    - Request body type: `Task`
    - Response type: `Task`

## Common Schemas

### Task

| Field | Type | Description |
|---|---|---|
| `assignmentInfo` | `AssignmentInfo` | Output only. Context information for assigned tasks. A task can be assigned to a user, currently possible from surfaces like Docs and Chat Spaces. |
| `completed` | string | Completion date of the task (as a RFC 3339 timestamp). This field is omitted if the task has not been completed. |
| `deleted` | boolean | Flag indicating whether the task has been deleted. For assigned tasks this field is read-only. |
| `due` | string | Scheduled date for the task (as an RFC 3339 timestamp). Optional. This represents the day that the task should be done, or that the task is visible on the calendar grid. |
| `etag` | string | ETag of the resource. |
| `hidden` | boolean | Flag indicating whether the task is hidden. This is the case if the task had been marked completed when the task list was last cleared. The default is False. This field is read-only. |
| `id` | string | Task identifier. |
| `kind` | string | Output only. Type of the resource. This is always "tasks#task". |
| `links` | array of object | Output only. Collection of links. This collection is read-only. |
| `notes` | string | Notes describing the task. Tasks assigned from Google Docs cannot have notes. Optional. Maximum length allowed: 8192 characters. |
| `parent` | string | Output only. Parent task identifier. This field is omitted if it is a top-level task. Use the "move" method to move the task under a different parent or to the top level. |
| `position` | string | Output only. String indicating the position of the task among its sibling tasks under the same parent task or at the top level. |
| `selfLink` | string | Output only. URL pointing to this task. Used to retrieve, update, or delete this task. |
| `status` | string | Status of the task. This is either "needsAction" or "completed". |
| `title` | string | Title of the task. Maximum length allowed: 1024 characters. |
| `updated` | string | Output only. Last modification time of the task (as a RFC 3339 timestamp). |
| `webViewLink` | string | Output only. An absolute link to the task in the Google Tasks Web UI. |

### TaskList

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the resource. |
| `id` | string | Task list identifier. |
| `kind` | string | Output only. Type of the resource. This is always "tasks#taskList". |
| `selfLink` | string | Output only. URL pointing to this task list. Used to retrieve, update, or delete this task list. |
| `title` | string | Title of the task list. Maximum length allowed: 1024 characters. |
| `updated` | string | Output only. Last modification time of the task list (as a RFC 3339 timestamp). |

### TaskLists

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the resource. |
| `items` | array of `TaskList` | Collection of task lists. |
| `kind` | string | Type of the resource. This is always "tasks#taskLists". |
| `nextPageToken` | string | Token that can be used to request the next page of this result. |

### Tasks

| Field | Type | Description |
|---|---|---|
| `etag` | string | ETag of the resource. |
| `items` | array of `Task` | Collection of tasks. |
| `kind` | string | Type of the resource. This is always "tasks#tasks". |
| `nextPageToken` | string | Token used to access the next page of this result. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws tasks --help

# Inspect a method's required params, types, and defaults
gws schema tasks.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

