---
name: gws-docs
description: "Read and write Google Docs."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws docs --help"
---

# docs (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws docs <resource> <method> [flags]
```

## Helper Commands

| Command | Description |
|---------|-------------|
| [`+write`](../gws-docs-write/SKILL.md) | Append text to a document |

## API Resources

### documents

  - `batchUpdate` — Applies one or more updates to the document. Each request is validated before being applied. If any request is not valid, then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. Other requests do not need to return information; these each return an empty reply. The order of replies matches that of the requests.
    - Required path params: documentId
    - Request body type: `BatchUpdateDocumentRequest`
    - Response type: `BatchUpdateDocumentResponse`
  - `create` — Creates a blank document using the title given in the request. Other fields in the request, including any provided content, are ignored. Returns the created document.
    - Request body type: `Document`
    - Response type: `Document`
  - `get` — Gets the latest version of the specified document.
    - Required path params: documentId
    - Response type: `Document`

## Common Schemas

### BatchUpdateDocumentRequest

*Description: Request message for BatchUpdateDocument.*

| Field | Type | Description |
|---|---|---|
| `requests` | array of `Request` | A list of updates to apply to the document. |
| `writeControl` | `WriteControl` | Provides control over how write requests are executed. |

### BatchUpdateDocumentResponse

*Description: Response message from a BatchUpdateDocument request.*

| Field | Type | Description |
|---|---|---|
| `documentId` | string | The ID of the document to which the updates were applied to. |
| `replies` | array of `Response` | The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty. |
| `writeControl` | `WriteControl` | The updated write control after applying the request. |

### Document

*Description: A Google Docs document.*

| Field | Type | Description |
|---|---|---|
| `body` | `Body` | Output only. The main body of the document. |
| `documentId` | string | Output only. The ID of the document. |
| `documentStyle` | `DocumentStyle` | Output only. The style of the document. |
| `footers` | object | Output only. The footers in the document, keyed by footer ID. |
| `footnotes` | object | Output only. The footnotes in the document, keyed by footnote ID. |
| `headers` | object | Output only. The headers in the document, keyed by header ID. |
| `inlineObjects` | object | Output only. The inline objects in the document, keyed by object ID. |
| `lists` | object | Output only. The lists in the document, keyed by list ID. |
| `namedRanges` | object | Output only. The named ranges in the document, keyed by name. |
| `namedStyles` | `NamedStyles` | Output only. The named styles of the document. |
| `positionedObjects` | object | Output only. The positioned objects in the document, keyed by object ID. |
| `revisionId` | string | Output only. The revision ID of the document. |
| `suggestedDocumentStyleChanges` | object | Output only. The suggested changes to the style of the document, keyed by suggestion ID. |
| `suggestedNamedStylesChanges` | object | Output only. The suggested changes to the named styles of the document, keyed by suggestion ID. |
| `suggestionsViewMode` | string | Output only. The suggestions view mode applied to the document. Note: When editing a document, changes must be based on a document with SUGGESTIONS_INLINE. |
| `tabs` | array of `Tab` | Tabs that are part of a document. Tabs can contain child tabs, a tab nested within another tab. Child tabs are represented by the Tab.childTabs field. |
| `title` | string | The title of the document. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws docs --help

# Inspect a method's required params, types, and defaults
gws schema docs.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

