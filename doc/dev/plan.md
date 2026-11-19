# Development Plan and Feature History

Tracks planned work and completed features across parsing, code generation, and
the public interface. Completed entries include implementation details, commit
links, and dates.

## TODOs

| Area | Feature | Notes |
| --- | --- | --- |
| Parser | Template-defined functions | Allow functions to be defined in templates. |
| Parser | Direct borrow/dereference syntax | Proposed borrow syntax: `@&EXP`. Reconsider `@*EXP` because `@*` starts template comments. |
| Parser | Partial/custom-view rendering syntax | Proposed syntax: `@Custom::View("name", context!(/*properties*/))`. A view resolver already exists. |
| Code generation | Static CSS/JavaScript optimization | Preserve plain-text output. |
| Public interface | Configuration/context-based view variants | Extend view selection beyond template kind. |

## Completed

Entries are sorted newest first by their most recent commit date. Within each
entry, commits are also sorted newest first. Dates are Git committer dates and
follow the same order as the commits.

| Area | Feature | Behavior | Commits | Date |
| --- | --- | --- | --- | --- |
| Code generation | Static-write merging | Merge adjacent static HTML/JSON writes. | [abb16af](https://github.com/gu-wei-x/sbolt/commit/abb16afa694e47c166e9860df52fd9f75d058cb2) - JSON optimization integration<br>[b38ef88](https://github.com/gu-wei-x/sbolt/commit/b38ef8844b93a9a498033bf2c68fe2ea356696a7) - context-aware static runs | 2026-11-18<br>2026-11-17 |
| Code generation | Static JSON validation and compaction | Apply to fully static standalone JSON when optimization is enabled. | [abb16af](https://github.com/gu-wei-x/sbolt/commit/abb16afa694e47c166e9860df52fd9f75d058cb2) - JSON validation, compaction, and source diagnostics | 2026-11-18 |
| Code generation | Kind-aware templates and layouts | Support same-name HTML, JSON, and text templates with matching-kind layouts. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - kind-indexed registry and layout lookup | 2026-11-18 |
| Code generation | Collision-safe template modules | Avoid generated module-name collisions with sibling directories. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - unique module names and explicit file paths | 2026-11-18 |
| Public interface | Optional render kind | Default to HTML and return output plus the resolved kind. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - generated render API | 2026-11-18 |
| Public interface | Accept preference resolution | Use `sbolt::resolve_kind`, including quality weights and wildcards. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - shared resolver and quality validation | 2026-11-18 |
| Public interface | Example-owned framework integration | Set matching `Content-Type` and `Vary: Accept` response headers. | [f89e2c3](https://github.com/gu-wei-x/sbolt/commit/f89e2c3b9b0a9e0921cceac8c4156fec88098ca8) - Actix, Axum, and Rocket extractors/responders | 2026-11-18 |
| Parser | Sections and section rendering | Define and render named or default content. | [e472e31](https://github.com/gu-wei-x/sbolt/commit/e472e31a01793b37b43e8e01c3420775abe80fc5) - reusable sections and required-argument fixes<br>[2f76dfc](https://github.com/gu-wei-x/sbolt/commit/2f76dfc9129848b8bc20052c740a71b45031e13a) - layout/section integration | 2026-11-17<br>2025-10-08 |
| Code generation | HTML whitespace compaction | Compact conservatively while preserving protected contexts. | [b38ef88](https://github.com/gu-wei-x/sbolt/commit/b38ef8844b93a9a498033bf2c68fe2ea356696a7) - context-aware whitespace scanner | 2026-11-17 |
| Parser | Source diagnostics | Include line/column information. | [8d19bdd](https://github.com/gu-wei-x/sbolt/commit/8d19bdd571e1baace97a7e4302a359af92131b04) - error-position fixes<br>[237cd5f](https://github.com/gu-wei-x/sbolt/commit/237cd5fe24507987f21dba76a800e3359d05b2b4) - source locations | 2025-10-10<br>2025-10-02 |
| Parser | Layout and use directives | Declare layouts and Rust imports. | [5687f99](https://github.com/gu-wei-x/sbolt/commit/5687f990cd86dca6009da1c3a1350a2104abbcd7) - directive handling and code generation | 2025-09-26 |
| Public interface | Multiple source directories | Compile templates from multiple source directories. | [aed4665](https://github.com/gu-wei-x/sbolt/commit/aed46651cb9d9bafabf3d0d36030cf12cc1afbd5) - source-directory list and per-directory generation | 2025-09-16 |
