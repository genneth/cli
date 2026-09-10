---
name: gws-gmail-read
description: "Gmail: Read a complete message or conversation with decoded bodies and attachment metadata."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws gmail +read --help"
---

# gmail +read

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

Read a complete message or conversation with decoded bodies and attachment metadata

## Usage

```bash
gws gmail +read (--id <ID> | --thread-id <ID>) [OPTIONS]
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--id` | — | — | The Gmail message ID to read |
| `--thread-id` | — | — | Read every message in this Gmail thread |
| `--headers` | — | — | Include headers (From, To, Subject, Date) in the output |
| `--format` | — | text | Output format (text, json) |
| `--html` | — | — | Return HTML body instead of plain text |
| `--dry-run` | — | — | Show the request that would be sent without executing it |

## Examples

```bash
gws gmail +read --id 18f1a2b3c4d
gws gmail +read --id 18f1a2b3c4d --headers
gws gmail +read --id 18f1a2b3c4d --format json | jq '.body_text'
gws gmail +read --thread-id THREAD_ID --format json
```

## Tips

- Converts HTML-only messages to plain text automatically.
- Handles multipart/alternative, charset decoding, and externally stored body parts.
- Choose exactly one of --id or --thread-id. Thread JSON contains a messages array.
- JSON id is the Gmail API ID; rfc_message_id and the legacy message_id are RFC mail headers.
- JSON includes body_text, body_html, label_ids, and attachments; attachment bytes are downloaded with +attachment.
- Missing or undecodable content fails visibly; snippets are never substituted for full bodies.

## See Also

- [gws-shared](../gws-shared/SKILL.md) — Global flags and auth
- [gws-gmail](../gws-gmail/SKILL.md) — All send, read, and manage email commands
