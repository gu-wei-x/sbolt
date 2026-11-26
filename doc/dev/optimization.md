# Optimization

Focus on two areas: binary size and rendering cost.
Measure them separately; improving one can worsen the other.

| Area | Focus | Measurement |
| --- | --- | --- |
| Binary size | Embedded static strings and generated code | Release executable bytes and data-section size |
| Rendering cost | Rendering time, heap allocations, and buffer growth | Criterion timings and allocation statistics |

Rendered output size is a related metric, but smaller output does not necessarily
mean a smaller executable or faster rendering.

## 1. Binary size

### Current compile-time optimization

The [content generator](../../rust/core/lib/src/codegen/compiler/types/content.rs)
merges adjacent static writes and compacts whitespace where context is known.
The default is `cfg!(not(debug_assertions))` in sbolt: normally disabled in debug
builds and enabled in release builds. Override it with
`CompilerOptions::with_optimization(true)` or `.with_optimization(false)`.

| Content | Behavior |
| --- | --- |
| Adjacent static writes | Merge when no runtime operation separates them, including across template comments |
| Known HTML tag syntax | Compact redundant whitespace without rewriting attribute values or quotes |
| Whitespace between understood HTML tags | Remove at standard block boundaries; collapse to one space between inline tags |
| Unknown or protected HTML context | Preserve content rather than guessing |
| Fully static standalone JSON | Validate the complete value and remove JSON whitespace outside strings |
| Dynamic JSON, sections, and layout-backed JSON | Preserve whitespace; adjacent static writes can still merge |
| Plain text or optimization disabled | Leave output unchanged |

For example:

```html
<div>
  </div>
```

becomes `<div></div>`, while whitespace between inline elements remains a
separator. Static JSON `{ "items" : [ 1, true ] }` becomes `{"items":[1,true]}`.

JSON compaction preserves string contents, escape spelling, number precision,
member order, and duplicate keys. Invalid static JSON is a compile error when
optimization is enabled. Any JSON root value is supported, with a nesting limit
of 128 containers. Diagnostics retain original template locations.

### Safety boundaries

HTML optimization assumes normal whitespace and default element-display rules;
it does not evaluate CSS. Disable it when whitespace-only DOM nodes matter,
when styles such as `white-space: pre-wrap` make spacing significant, or when
output is embedded in an unknown language or raw-text context.

The optimizer preserves:

- Non-whitespace text and leading/trailing whitespace at static-run boundaries.
- Attribute values, quote delimiters, entities, and HTML comments.
- Paired closing tags for non-void elements, such as `<div></div>`.
- Incomplete or unsupported syntax instead of repairing it.

Compaction stops at protected containers such as `<pre>`, `<listing>`,
`<dialog>`, `<script>`, `<style>`, SVG, and MathML. A `hidden` attribute also
ends compaction, preserving preceding whitespace and the remaining static run.
The ordinary `<!DOCTYPE html>` declaration is recognized and preserved.

Dynamic expressions, arbitrary Rust code, section creation, and `@render` are
context barriers. Later static output remains unmodified, though adjacent
static writes can still merge. Sections and templates with layouts start in
an unknown insertion context.

For example, the static fragments around `@value` in:

```html
<div title="@value">hello</div>
```

are not independent HTML documents. The second fragment starts inside an
attribute. Do not optimize fragments as standalone markup or assume normal text
context after a dynamic operation.

### Measure actual size

Use `CompileResult::static_output_stats()` to inspect generated static content:

```rust
let result = sbolt::codegen::Compiler::new(options).compile();
let stats = result.static_output_stats();
eprintln!(
    "static bytes: {} -> {}; writes: {} -> {}; distinct bytes: {}; unknown-context writes: {}",
    stats.input_bytes(),
    stats.output_bytes(),
    stats.input_writes(),
    stats.output_writes(),
    stats.distinct_output_bytes(),
    stats.unknown_context_writes(),
);
```

These are compile-time counts, not writes executed by loops or predictions of
linker storage. Also measure the data section and total size of an actual release
executable that uses the templates, not just the library archive.

Keep the target, features, linker, and debug-information settings unchanged.
Fewer generated writes or shorter Rust source do not guarantee a smaller binary.
Rust and the linker may already share identical literals.

## 2. Rendering cost

### Measuring rendering cost

Run from the [core workspace](../../rust/core):

```shell
# Rendering time
cargo bench -p lib_benchmark --bench template_benches

# Heap allocations for the large data table
cargo bench -p lib_benchmark --bench memory_benches
```

Both compare resolving once with looking up the template on every render.
Setup is excluded.

For the quick execution check used by [CI](../../.github/workflows/ci.yml):

```shell
cargo bench -p lib_benchmark --bench memory_benches --bench template_benches -- --test
```

`--test` skips Criterion timing statistics; allocation measurements still run.
Neither target enforces performance thresholds. See [benchmarks](./benchmark.md)
for workloads, metrics, and limitations.

### Implemented behavior

- **Direct output:** HTML and text expressions use specialized `WriteTo`
  implementations where available, with streaming `Display` formatting as a
  fallback. This avoids intermediate interpolation strings. HTML expressions
  use an escaping adapter; JSON expressions stream serialized values.
- **Resolved templates:** resolve the entry template once and reuse its handle
  to avoid repeated name lookup. Layouts and nested views still resolve their
  own dependencies.
- **Capacity estimates:** generated static-literal counts seed output-buffer
  capacity. A context hint can supply a larger estimate for dynamic content.
- **Fresh output:** each render returns a new `String`. Sections and layouts
  retain intermediate buffers needed for composition.

For a known workload, supply an estimated output size in bytes:

```rust
context.set_capacity_hint(140_000);
let output = template.render(&mut context)?;
```

Rendering uses the larger of the static estimate and the context hint, not their
sum. The hint persists across renders; setting it to zero restores static-only
estimates. Loop counts and runtime value lengths are not known at compile time,
so hints are neither output-size bounds nor guarantees against reallocation.
Overestimating can waste memory.

See [template compilation](./template-compilation.md) for dispatch, escaping,
typed models, resolved handles, and capacity-hint details.

### Future options

These are not part of the current optimization pass:

| Option | Potential benefit | Trade-off |
| --- | --- | --- |
| Final-output HTML minification | Sees the complete rendered document | Extra parsing and typically another allocation |
| Streaming HTML minification | Can avoid a second full buffer | Complex handling of dynamic boundaries and insertion context |
| Literal pooling | May reduce repeated embedded data | Extra metadata; the linker may already deduplicate it |
| Embedded-string compression | May reduce storage for large static content | Decompression code, CPU cost, and possible allocations |

Only add these after measurements justify them. Runtime HTML minification must
preserve whitespace-sensitive content, scripts, styles, and CSS-dependent
spacing, with an opt-out. It must not apply HTML rules to JSON or plain text.

For web applications, HTTP Brotli or gzip can reduce transferred bytes without
changing HTML semantics. That reduces wire size, not the rendered string or
executable size.

## Validating optimizations

Compare before and after changes using the same machine, toolchain, profile,
and fixtures.

1. Establish timing, allocation, output-size, and executable-size baselines.
2. Change one optimization at a time and rerun representative workloads.
3. Check output correctness separately; benchmarks do not replace tests.
4. Accept changes only when the intended metric improves without unacceptable
   regressions in the others.

For output-preserving changes, assert exact byte equality. For intentional
whitespace compaction, assert expected output and check browser behavior where
DOM structure, text spacing, or CSS matters.

Regression cases should cover quoted attributes, split tags, fragment-edge
spaces, inline elements, protected containers, custom elements, dynamic
expressions, branches, loops, sections, layouts, and JSON/text behavior.
Preserve source diagnostics and document intentional output differences.
