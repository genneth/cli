---
name: gws-script
description: "Manage Google Apps Script projects."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws script --help"
---

# script (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws script <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+push`](../gws-script-push/SKILL.md) | Upload local files to an Apps Script project |

## API Resources

### processes

  - `list` — List information about processes made by or on behalf of a user, such as process type and current status.
    - Response type: `ListUserProcessesResponse`
  - `listScriptProcesses` — List information about a script's executed processes, such as process type and current status.
    - Response type: `ListScriptProcessesResponse`

### projects

  - `create` — Creates a new, empty script project with no script files and a base manifest file.
    - Request body type: `CreateProjectRequest`
    - Response type: `Project`
  - `get` — Gets a script project's metadata.
    - Required path params: scriptId
    - Response type: `Project`
  - `getContent` — Gets the content of the script project, including the code source and metadata for each script file.
    - Required path params: scriptId
    - Response type: `Content`
  - `getMetrics` — Get metrics data for scripts, such as number of executions and active users.
    - Required path params: scriptId
    - Response type: `Metrics`
  - `updateContent` — Updates the content of the specified script project. This content is stored as the HEAD version, and is used when the script is executed as a trigger, in the script editor, in add-on preview mode, or as a web app or Apps Script API in development mode. This clears all the existing files in the project.
    - Required path params: scriptId
    - Request body type: `Content`
    - Response type: `Content`

### projects.deployments

  - `create` — Creates a deployment of an Apps Script project.
    - Required path params: scriptId
    - Request body type: `DeploymentConfig`
    - Response type: `Deployment`
  - `delete` — Deletes a deployment of an Apps Script project.
    - Required path params: deploymentId, scriptId
    - Response type: `Empty`
  - `get` — Gets a deployment of an Apps Script project.
    - Required path params: deploymentId, scriptId
    - Response type: `Deployment`
  - `list` — Lists the deployments of an Apps Script project.
    - Required path params: scriptId
    - Response type: `ListDeploymentsResponse`
  - `update` — Updates a deployment of an Apps Script project.
    - Required path params: deploymentId, scriptId
    - Request body type: `UpdateDeploymentRequest`
    - Response type: `Deployment`

### projects.versions

  - `create` — Creates a new immutable version using the current code, with a unique version number.
    - Required path params: scriptId
    - Request body type: `Version`
    - Response type: `Version`
  - `get` — Gets a version of a script project.
    - Required path params: scriptId, versionNumber
    - Response type: `Version`
  - `list` — List the versions of a script project.
    - Required path params: scriptId
    - Response type: `ListVersionsResponse`

### scripts

  - `run` — 
    - Required path params: scriptId
    - Request body type: `ExecutionRequest`
    - Response type: `Operation`

## Common Schemas

### Content

*Description: The Content resource.*

| Field | Type | Description |
|---|---|---|
| `files` | array of `File` | The list of script project files. One of the files is a script manifest; it must be named "appsscript", must have type of JSON, and include the manifest configurations for the project. |
| `scriptId` | string | The script project's Drive ID. |

### CreateProjectRequest

*Description: Request to create a script project.*

| Field | Type | Description |
|---|---|---|
| `parentId` | string | The Drive ID of a parent file that the created script project is bound to. This is usually the ID of a Google Doc, Google Sheet, Google Form, or Google Slides file. |
| `title` | string | The title for the project. |

### Deployment

*Description: Representation of a single script deployment.*

| Field | Type | Description |
|---|---|---|
| `deploymentConfig` | `DeploymentConfig` | The deployment configuration. |
| `deploymentId` | string | The deployment ID for this deployment. |
| `entryPoints` | array of `EntryPoint` | The deployment's entry points. |
| `updateTime` | string (format: google-datetime) | Last modified date time stamp. |

### DeploymentConfig

*Description: Metadata the defines how a deployment is configured.*

| Field | Type | Description |
|---|---|---|
| `description` | string | The description for this deployment. |
| `manifestFileName` | string | The manifest file name for this deployment. |
| `scriptId` | string | The script project's Drive ID. |
| `versionNumber` | integer (format: int32) | The version number on which this deployment is based. |

### Empty

*Description: A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }*

*(No fields)*

### ExecutionRequest

*Description: A request to run the function in a script. The script is identified by the specified `script_id`. Executing a function on a script returns results based on the implementation of the script.*

| Field | Type | Description |
|---|---|---|
| `devMode` | boolean | If `true` and the user is an owner of the script, the script runs at the most recently saved version rather than the version deployed for use with the Apps Script API. Optional; default is `false`. |
| `function` | string | The name of the function to execute in the given script. The name does not include parentheses or parameters. It can reference a function in an included library such as `Library.libFunction1`. |
| `parameters` | array of any | The parameters to be passed to the function being executed. The object type for each parameter should match the expected type in Apps Script. |
| `sessionState` | string | *Deprecated*. For use with Android add-ons only. |

### ListDeploymentsResponse

*Description: Response with the list of deployments for the specified Apps Script project.*

| Field | Type | Description |
|---|---|---|
| `deployments` | array of `Deployment` | The list of deployments. |
| `nextPageToken` | string | The token that can be used in the next call to get the next page of results. |

### ListScriptProcessesResponse

*Description: Response with the list of Process resources.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token for the next page of results. If empty, there are no more pages remaining. |
| `processes` | array of `GoogleAppsScriptTypeProcess` | List of processes matching request parameters. |

### ListUserProcessesResponse

*Description: Response with the list of Process resources.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | Token for the next page of results. If empty, there are no more pages remaining. |
| `processes` | array of `GoogleAppsScriptTypeProcess` | List of processes matching request parameters. |

### ListVersionsResponse

*Description: Response with the list of the versions for the specified script project.*

| Field | Type | Description |
|---|---|---|
| `nextPageToken` | string | The token use to fetch the next page of records. if not exist in the response, that means no more versions to list. |
| `versions` | array of `Version` | The list of versions. |

### Metrics

*Description: Resource containing usage stats for a given script, based on the supplied filter and mask present in the request.*

| Field | Type | Description |
|---|---|---|
| `activeUsers` | array of `MetricsValue` | Number of active users. |
| `failedExecutions` | array of `MetricsValue` | Number of failed executions. |
| `totalExecutions` | array of `MetricsValue` | Number of total executions. |

### Operation

*Description: A representation of an execution of an Apps Script function started with run. The execution response does not arrive until the function finishes executing. The maximum execution runtime is listed in the [Apps Script quotas guide](/apps-script/guides/services/quotas#current_limitations).*

| Field | Type | Description |
|---|---|---|
| `done` | boolean | This field indicates whether the script execution has completed. A completed execution has a populated `response` field containing the ExecutionResponse from function that was executed. |
| `error` | `Status` | If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, this field contains a Status object. |
| `response` | object | If the script function returns successfully, this field contains an ExecutionResponse object with the function's return value. |

### Project

*Description: The script project resource.*

| Field | Type | Description |
|---|---|---|
| `createTime` | string (format: google-datetime) | When the script was created. |
| `creator` | `GoogleAppsScriptTypeUser` | User who originally created the script. |
| `lastModifyUser` | `GoogleAppsScriptTypeUser` | User who last modified the script. |
| `parentId` | string | The parent's Drive ID that the script will be attached to. This is usually the ID of a Google Document or Google Sheet. This field is optional, and if not set, a stand-alone script will be created. |
| `scriptId` | string | The script project's Drive ID. |
| `title` | string | The title for the project. |
| `updateTime` | string (format: google-datetime) | When the script was last updated. |

### UpdateDeploymentRequest

*Description: Request with deployment information to update an existing deployment.*

| Field | Type | Description |
|---|---|---|
| `deploymentConfig` | `DeploymentConfig` | The deployment configuration. |

### Version

*Description: A resource representing a script project version. A version is a "snapshot" of a script project and is similar to a read-only branched release. When creating deployments, the version to use must be specified.*

| Field | Type | Description |
|---|---|---|
| `createTime` | string (format: google-datetime) | When the version was created. |
| `description` | string | The description for this version. |
| `scriptId` | string | The script project's Drive ID. |
| `versionNumber` | integer (format: int32) | The incremental ID that is created by Apps Script when a version is created. This is system assigned number and is immutable once created. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws script --help

# Inspect a method's required params, types, and defaults
gws schema script.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

