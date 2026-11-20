# Development Plan and Feature History

Roadmap and implementation history for template parsing, code generation,
the public interface, examples, and documentation. Planned features appear under TODOs; completed features
include behavior summaries, implementation commits, and commit dates.

## Areas

Feature tables use the aliases below.

| Area | Alias | Scope |
| --- | --- | --- |
| Parser | `Parse` | Template syntax, directives, sections, and source diagnostics. |
| Code generation | `CodeGen` | Generated Rust, template modules, layouts, and output optimization. |
| Public interface | `API` | Rendering APIs, configuration, kind resolution, and framework integration. |
| Example | `Example` | Runnable CLI and web demonstrations, shared models/templates, standalone compilation, and framework adapters. |
| Docs | `Docs` | Getting-started guides, syntax and API references, layout guidance, developer documentation, and feature history. |

## TODOs

| Area | Feature | Notes |
| --- | --- | --- |
| `CodeGen` | HTML escaping and JSON serialization | **Highest priority; next planned work.** Separate HTML text/attribute escaping from JSON string/value serialization, with explicit trusted/raw controls. Define defaults, insertion-context limits, and a migration policy. Test injection-sensitive HTML, JSON quotes/backslashes/control characters, and serialized value types; preserve streaming writes where practical. [Current interpolation is raw](../user/syntax.md#output-safety). |
| `Parse` | Template-defined functions | Allow functions to be defined in templates. |
| `Parse` | Direct borrow/dereference syntax | Proposed borrow syntax: `@&EXP`. Reconsider `@*EXP` because `@*` starts template comments. |
| `CodeGen` | Static CSS/JavaScript optimization | Preserve plain-text output. |
| `API` | Configurable view variants | Select variants using configuration or context, beyond template kind. |

## Completed

Entries are sorted newest first by their latest implementation commit.
Within each entry, commits and their corresponding dates are also newest first.
All dates are Git committer dates.

| Area | Feature | Behavior | Commits | Dates |
| --- | --- | --- | --- | --- |
| `Parse` | Inline custom views | `@view(name, context[, kind])` renders a child inline with an explicit context, an inherited or overridden kind, the child's layout, and propagated errors. See [custom views](../user/syntax.md#custom-views). | [c209bfd](https://github.com/gu-wei-x/sbolt/commit/c209bfd75031d8c3fe676ffc79e346018657b7f8) - inline custom views and optional kind override | 2026-11-19 |
| `CodeGen` | Static-write merging | Merge adjacent static HTML/JSON writes. | [abb16af](https://github.com/gu-wei-x/sbolt/commit/abb16afa694e47c166e9860df52fd9f75d058cb2) - JSON optimization integration<br>[b38ef88](https://github.com/gu-wei-x/sbolt/commit/b38ef8844b93a9a498033bf2c68fe2ea356696a7) - context-aware static runs | 2026-11-18<br>2026-11-17 |
| `CodeGen` | Static JSON validation and compaction | Apply to fully static standalone JSON when optimization is enabled. | [abb16af](https://github.com/gu-wei-x/sbolt/commit/abb16afa694e47c166e9860df52fd9f75d058cb2) - JSON validation, compaction, and source diagnostics | 2026-11-18 |
| `CodeGen` | Kind-aware templates and layouts | Support same-name HTML, JSON, and text templates with matching-kind layouts. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - kind-indexed registry and layout lookup | 2026-11-18 |
| `CodeGen` | Collision-safe template modules | Avoid generated module-name collisions with sibling directories. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - unique module names and explicit file paths | 2026-11-18 |
| `API` | Optional render kind | Default to HTML and return output plus the resolved kind. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - generated render API | 2026-11-18 |
| `API` | Accept preference resolution | Use `sbolt::resolve_kind`, including quality weights and wildcards. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - shared resolver and quality validation | 2026-11-18 |
| `API` | Example-owned framework integration | Set matching `Content-Type` and `Vary: Accept` response headers. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - Actix, Axum, and Rocket extractors/responders | 2026-11-18 |
| `Parse` | Sections and section rendering | Define and render named or default content. | [e472e31](https://github.com/gu-wei-x/sbolt/commit/e472e31a01793b37b43e8e01c3420775abe80fc5) - reusable sections and required-argument fixes<br>[2f76dfc](https://github.com/gu-wei-x/sbolt/commit/2f76dfc9129848b8bc20052c740a71b45031e13a) - layout/section integration | 2026-11-17<br>2025-10-08 |
| `CodeGen` | HTML whitespace compaction | Compact conservatively while preserving protected contexts. | [b38ef88](https://github.com/gu-wei-x/sbolt/commit/b38ef8844b93a9a498033bf2c68fe2ea356696a7) - context-aware whitespace scanner | 2026-11-17 |
| `Parse` | Source diagnostics | Include line/column information. | [8d19bdd](https://github.com/gu-wei-x/sbolt/commit/8d19bdd571e1baace97a7e4302a359af92131b04) - error-position fixes<br>[237cd5f](https://github.com/gu-wei-x/sbolt/commit/237cd5fe24507987f21dba76a800e3359d05b2b4) - source locations | 2025-10-10<br>2025-10-02 |
| `Parse` | Layout and use directives | Declare layouts and Rust imports. | [5687f99](https://github.com/gu-wei-x/sbolt/commit/5687f990cd86dca6009da1c3a1350a2104abbcd7) - directive handling and code generation | 2025-09-26 |
| `API` | Multiple source directories | Compile templates from multiple source directories. | [aed4665](https://github.com/gu-wei-x/sbolt/commit/aed46651cb9d9bafabf3d0d36030cf12cc1afbd5) - source-directory list and per-directory generation | 2025-09-16 |
