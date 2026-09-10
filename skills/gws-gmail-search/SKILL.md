---
name: gws-gmail-search
description: "Gmail: Search Gmail and retrieve complete readable messages."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws gmail +search --help"
---

# gmail +search

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

Search Gmail and retrieve complete readable messages

## Usage

```bash
gws gmail +search [OPTIONS]
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--params` | — | {} | Gmail messages.list parameters as JSON, e.g. {"q":"from:alice"} |
| `--max-messages` | — | 20 | Maximum messages to retrieve across pages (1–1000) |
| `--page-limit` | — | 10 | Maximum search pages to fetch (1–100) |
| `--format` | — | json | Output decoded messages as JSON or readable text |
| `--dry-run` | — | — | Validate search options without reading mail |

## Examples

```bash
gws gmail +search --params '{"q":"from:alice has:attachment"}' --max-messages 10
gws gmail +search --params '{"q":"newer_than:7d"}' --format json
```

## Tips

- Returns messages with full decoded bodies, Gmail IDs, thread IDs, and attachment metadata.
- complete=false and nextPageToken indicate more search results; resume with pageToken in --params.
- A failed fetch fails the command; omitted messages are never reported as a successful empty result.
- For a whole conversation use gws gmail +read --thread-id THREAD_ID.

## See Also

- [gws-shared](../gws-shared/SKILL.md) — Global flags and auth
- [gws-gmail](../gws-gmail/SKILL.md) — All send, read, and manage email commands
