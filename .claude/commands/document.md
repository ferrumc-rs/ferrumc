Document the specified topic into the `docs/` folder.

## Instructions

1. **Identify the topic** from the user's prompt. If unclear, ask for clarification.
2. **Research thoroughly** — read all relevant source files, configs, and code before writing. Do not document from memory or assumptions.
3. **Choose the right location** in `docs/`:
   - Use existing subdirectories if the topic fits (e.g., `docs/ci/` for CI/CD topics).
   - Create a new subdirectory if documenting a distinct system (e.g., `docs/networking/`, `docs/ecs/`, `docs/storage/`).
   - Use `README.md` as the main file in each subdirectory.
   - Add additional files in the subdirectory for sub-topics if needed (e.g., `docs/networking/packet-format.md`).
4. **Writing style**:
   - Be concise and direct. No fluff or filler.
   - Use tables for structured data (configs, flags, comparisons).
   - Include concrete examples (commands, code snippets, file paths).
   - Document the "why" behind non-obvious decisions, not just the "what".
   - Keep it accurate to the current state of the code — don't document aspirational or planned features.
5. **After writing**, update `CLAUDE.md` if the documented topic affects build commands, architecture descriptions, or development workflows already mentioned there.

## Subdirectory Conventions

```
docs/
  ci/          — CI/CD pipelines, build profiles, caching, releases
  architecture/ — System design, crate relationships, data flow
  networking/  — Protocol, packets, connection lifecycle
  storage/     — World storage, LMDB, chunk format
  ecs/         — Bevy ECS patterns, components, systems, messages
  commands/    — Command system, adding new commands
  registry/    — Block/item registry, build-time codegen
```

Create subdirectories as needed. Not all of these need to exist from day one.

$ARGUMENTS
