---
name: gws-slides
description: "Google Slides: Read and write presentations."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws slides --help"
---

# slides (v1)

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

```bash
gws slides <resource> <method> [flags]
```

## API Resources

### presentations

  - `batchUpdate` — Applies one or more updates to the presentation. Each request is validated before being applied. If any request is not valid, then the entire request will fail and nothing will be applied. Some requests have replies to give you some information about how they are applied. Other requests do not need to return information; these each return an empty reply. The order of replies matches that of the requests.
    - Required path params: presentationId
    - Request body type: `BatchUpdatePresentationRequest`
    - Response type: `BatchUpdatePresentationResponse`
  - `create` — Creates a blank presentation using the title given in the request. If a `presentationId` is provided, it is used as the ID of the new presentation. Otherwise, a new ID is generated. Other fields in the request, including any provided content, are ignored. Returns the created presentation.
    - Request body type: `Presentation`
    - Response type: `Presentation`
  - `get` — Gets the latest version of the specified presentation.
    - Required path params: presentationId
    - Response type: `Presentation`

### presentations.pages

  - `get` — Gets the latest version of the specified page in the presentation.
    - Required path params: pageObjectId, presentationId
    - Response type: `Page`
  - `getThumbnail` — Generates a thumbnail of the latest version of the specified page in the presentation and returns a URL to the thumbnail image. This request counts as an [expensive read request](https://developers.google.com/workspace/slides/limits) for quota purposes.
    - Required path params: pageObjectId, presentationId
    - Response type: `Thumbnail`

## Common Schemas

### BatchUpdatePresentationRequest

*Description: Request message for PresentationsService.BatchUpdatePresentation.*

| Field | Type | Description |
|---|---|---|
| `requests` | array of `Request` | A list of updates to apply to the presentation. |
| `writeControl` | `WriteControl` | Provides control over how write requests are executed. |

### BatchUpdatePresentationResponse

*Description: Response message from a batch update.*

| Field | Type | Description |
|---|---|---|
| `presentationId` | string | The presentation the updates were applied to. |
| `replies` | array of `Response` | The reply of the updates. This maps 1:1 with the updates, although replies to some requests may be empty. |
| `writeControl` | `WriteControl` | The updated write control after applying the request. |

### Page

*Description: A page in a presentation.*

| Field | Type | Description |
|---|---|---|
| `layoutProperties` | `LayoutProperties` | Layout specific properties. Only set if page_type = LAYOUT. |
| `masterProperties` | `MasterProperties` | Master specific properties. Only set if page_type = MASTER. |
| `notesProperties` | `NotesProperties` | Notes specific properties. Only set if page_type = NOTES. |
| `objectId` | string | The object ID for this page. Object IDs used by Page and PageElement share the same namespace. |
| `pageElements` | array of `PageElement` | The page elements rendered on the page. |
| `pageProperties` | `PageProperties` | The properties of the page. |
| `pageType` | string | The type of the page. |
| `revisionId` | string | Output only. The revision ID of the presentation. Can be used in update requests to assert the presentation revision hasn't changed since the last read operation. |
| `slideProperties` | `SlideProperties` | Slide specific properties. Only set if page_type = SLIDE. |

### Presentation

*Description: A Google Slides presentation.*

| Field | Type | Description |
|---|---|---|
| `layouts` | array of `Page` | The layouts in the presentation. A layout is a template that determines how content is arranged and styled on the slides that inherit from that layout. |
| `locale` | string | The locale of the presentation, as an IETF BCP 47 language tag. |
| `masters` | array of `Page` | The slide masters in the presentation. A slide master contains all common page elements and the common properties for a set of layouts. |
| `notesMaster` | `Page` | The notes master in the presentation. It serves three purposes: - Placeholder shapes on a notes master contain the default text styles and shape properties of all placeholder shapes on notes pages. |
| `pageSize` | `Size` | The size of pages in the presentation. |
| `presentationId` | string | The ID of the presentation. |
| `revisionId` | string | Output only. The revision ID of the presentation. Can be used in update requests to assert the presentation revision hasn't changed since the last read operation. |
| `slides` | array of `Page` | The slides in the presentation. A slide inherits properties from a slide layout. |
| `title` | string | The title of the presentation. |

### Thumbnail

*Description: The thumbnail of a page.*

| Field | Type | Description |
|---|---|---|
| `contentUrl` | string | The content URL of the thumbnail image. The URL to the image has a default lifetime of 30 minutes. This URL is tagged with the account of the requester. |
| `height` | integer (format: int32) | The positive height in pixels of the thumbnail image. |
| `width` | integer (format: int32) | The positive width in pixels of the thumbnail image. |

## Discovering Commands

Before calling any API method, inspect it:

```bash
# Browse resources and methods
gws slides --help

# Inspect a method's required params, types, and defaults
gws schema slides.<resource>.<method>
```

Use `gws schema` output to build your `--params` and `--json` flags.

