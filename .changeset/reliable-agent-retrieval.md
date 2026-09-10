---
"@googleworkspace/cli": minor
---

Add Gmail search with complete message content and whole-thread reading. Decode HTML-only and non-UTF-8 bodies, retrieve external body parts, expose Gmail IDs and attachment metadata, and preserve embedded attachments when forwarding. Attachment downloads now return JSON manifests by default (use `--format text` for human output), preserve existing files, and confine output paths to the working directory.

Paginate Calendar agendas, Gmail triage, and workflow reports; propagate retrieval failures instead of returning misleading empty results. Preserve source identifiers and document result bounds, continuation tokens, and estimated counts in generated skills.
