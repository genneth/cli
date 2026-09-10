---
name: gws-workflow-weekly-digest
description: "Google Workflow: Weekly summary: this week's meetings + unread email count."
metadata:
  version: 0.22.5
  openclaw:
    category: "productivity"
    requires:
      bins:
        - gws
    cliHelp: "gws workflow +weekly-digest --help"
---

# workflow +weekly-digest

> **PREREQUISITE:** Read `../gws-shared/SKILL.md` for auth, global flags, and security rules. If missing, run `gws generate-skills` to create it.

Weekly summary: this week's meetings + unread email count

## Usage

```bash
gws workflow +weekly-digest [OPTIONS]
```

## Flags

| Flag | Required | Default | Description |
|------|----------|---------|-------------|
| `--format` | — | — | Output format: json (default), table, yaml, csv |

## Examples

```bash
gws workflow +weekly-digest
gws workflow +weekly-digest --format table
```

## Tips

- Read-only — never modifies data.
- Combines this week's primary calendar with Gmail's unread-email estimate.
- Calendar events are paginated up to 100 pages; failed requests fail the command.
- unreadEmailsIsEstimate=true distinguishes the estimate from an exact mailbox count.

## See Also

- [gws-shared](../gws-shared/SKILL.md) — Global flags and auth
- [gws-workflow](../gws-workflow/SKILL.md) — All cross-service productivity workflows commands
