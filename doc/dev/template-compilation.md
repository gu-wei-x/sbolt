# Template compilation

```mermaid
flowchart TD
    A["Template: .rshtml / .rsjson / .rstxt"] --> B["Parse template syntax"]
    B --> C["Generate Rust code"]
    C --> D{"Optimization enabled and safe?"}
    D -- No --> E["Preserve content"]
    D -- HTML --> F["Compact HTML whitespace"]
    D -- Static JSON --> G["Validate and compact JSON"]
    E --> H["Write generated Rust"]
    F --> H
    G --> H
    H --> I["Compile into application"]
    I --> J["Runtime: render output"]
```

- All template types share the parser for `@` expressions and directives.
- `CompilerOptions::with_optimization(true)` enables static-write merging and
  whitespace compaction. HTML compaction assumes normal whitespace/default
  display rules and preserves protected or unknown contexts.
- JSON compaction applies only to fully static standalone templates. Dynamic
  JSON and section/layout fragments retain their whitespace. Invalid static JSON
  produces a compile error.
- Plain-text templates are not compacted. Rendering executes the generated Rust;
  it does not parse templates or run the optimizer again.

See [optimization details](./optimization.md) for behavior and limitations.
