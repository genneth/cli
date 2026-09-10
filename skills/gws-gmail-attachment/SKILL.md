---
name: gws-gmail-attachment
description: "Gmail: Download email attachments by name or download all attachments."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws gmail +attachment --help"
---

# gmail +attachment

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

Download email attachments by name or download all attachments

## Usage

```bash
gws gmail +attachment [OPTIONS] --message-id <MSG_ID>
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--message-id` | ✓ | — | Gmail message ID |
| `--name` | — | — | Name of the attachment to download |
| `--all` | — | — | Download all attachments |
| `--output` | — | — | Output file path (only valid with --name) |
| `--output-dir` | — | — | Output directory for downloaded files |
| `--format` | — | json | Output format; JSON reports saved file paths and metadata |
| `--dry-run` | — | — | Show the operations that would be executed without running them |

## Examples

```bash
gws gmail +attachment --message-id MSG_ID --name invoice.pdf --output ./invoice.pdf
gws gmail +attachment --message-id MSG_ID --all --output-dir ./downloads/
```

## Tips

- JSON output contains files with saved paths, sizes, and MIME types. Use --format text for human output.
- Output paths must stay under the current directory. Existing files are preserved with numbered filenames.
- Supports embedded and separately stored attachments, including inline images. Total download limit: 25 MB.

## See Also

- [gws-shared](../gws-shared/SKILL.md) — Global flags and auth
- [gws-gmail](../gws-gmail/SKILL.md) — All send, read, and manage email commands
