# Minerva Project

This repository uses Minerva for task and context management.

Prompt tags:

- If the prompt starts with `[exploration]`, inspect Minerva files and the referenced task files before changing code.
- If the prompt starts with `[static]`, skip extra Minerva investigation, but still complete the declaration.
- If the prompt has no tag, ignore Minerva investigation and focus only on the prompt.

Before starting work:

1. Read `.minerva/instructions.md` for project-specific rules.
2. Read the current task's `task.md`, `instructions.md`, and `declaration.md`.
3. Prefer Minerva CLI or MCP operations over manual edits to task metadata.
4. Update `declaration.md` after meaningful progress, decisions, or blockers.
5. Validate task state before marking work complete.

Preferred operations:

- Create and update tasks through Minerva tools.
- Change status and dependencies through Minerva tools.
- Keep detailed project guidance in `.minerva/instructions.md`.
