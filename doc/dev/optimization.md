# Optimization

Optimize three separate metrics: binary size, rendered output size, and rendering
cost. Improving one can worsen another.

For runtime value specialization, resolved-template handles, capacity hints, and
typed models, see [template compilation](./template-compilation.md).

## Measuring runtime rendering costs

Run these commands from the `rust` workspace:

```shell
cargo test -p lib_bench
cargo bench -p lib_bench --bench render_allocations
cargo bench -p lib_bench --bench template_benches -- "^(teams|big_table)/"
```

The allocation executable uses `stats_alloc` with the system allocator.
Inputs, contexts, model `Arc`s, and resolver initialization
are prepared before measurement; each workload is warmed once. It reports
allocation calls, reallocation calls, and allocated bytes separately, and
asserts one allocation and zero reallocations for estimated-capacity fresh output.
Counts include output destruction but exclude fixture setup. They are not
peak live memory.
Estimated-capacity cases set `context.set_capacity_hint(...)` before measurement
and use separate contexts so the estimate does not affect the baseline cases.

Criterion timing runs separately without the allocation-counting allocator.
The new Big table fixture has 100 rows and 100 integer cells per row. Teams has
three owned team names and integer scores. The `named/fresh` and `resolved/fresh`
comparisons keep setup outside timed iterations and consume results with `black_box`.
Each render returns a fresh string.
Tests compare all variants' output, including every cell.
All data-driven benchmark templates use `@model` declarations with `Arc<Model>`
stored under context key `"model"`. Static templates and layouts need no model.
The page benchmarks use a [standalone page](../../rust/benchmark/views/page.rshtml)
with `PageModel` and a
[layout-backed page](../../rust/benchmark/views/page_with_layout.rshtml).
The `*/named/fresh_with_setup` cases cover Big table and both pages, including
context creation inside each timed iteration.

Keep the named/fresh results separate from resolved/fresh, estimated-capacity,
and concrete-model-view results. Model rendering is reported
separately for named, resolved, and concrete entry points. All model entry points
retrieve the model from its declared context key; none accepts a borrowed model.
The Teams entry points all reuse the same
[Teams template](../../rust/benchmark/views/teams.rshtml), including the concrete
model-view benchmarks.
`resolution_only` measures warmed path
normalization and registry lookup, not rendering or lazy registry construction.
`formatting_only` and `specialized_formatting_only` both write the integers
0 through 9,999 into a fresh 40,000-byte buffer, without markup, context access,
or resolution. The specialized case uses the exact `WriteToRef` dispatch
emitted by the generator; the other uses `Writer::write_formatted`.
These isolated timings help attribution but are not an additive profile of a
complete render.

In a short local Windows run before migrating the benchmark fixtures to `@model`
(20 samples, 1 s warmup, 2 s measurement), changing
only value specialization reduced Big table named/fresh from about 158 us to
102 us. Both versions already avoided per-cell heap allocations. Before capacity
changes, resolving once removed 3 allocation calls and 3 reallocation calls per
render. Estimated-capacity fresh output measured one allocation and zero
reallocations.
Static hints alone reduced growth but did not eliminate it or guarantee lower
allocated bytes. Machine load affects these timings; rerun on a quiet machine
before drawing small-workload or typed-versus-dynamic timing conclusions.

## 1. Code generation: reduce embedded static strings

The [content generator](../../rust/core/lib/src/codegen/compiler/types/content.rs)
merges adjacent static HTML/JSON output before embedding it in generated Rust code.

### Implemented first pass

The pass defaults to `cfg!(not(debug_assertions))` in sbolt: normally disabled
in debug builds and enabled in release builds. Override it explicitly with
`CompilerOptions::with_optimization(true)` or `.with_optimization(false)`.
The pass:

- Combines consecutive static writes, including fragments separated by template
  comments.
- Compacts redundant HTML tag-syntax whitespace, without changing attribute
  values, quote delimiters, entities, or comment contents.
- Removes whitespace-only runs between understood tags when either adjacent tag
  is a standard block-level element. For example, `<div>\n  </div>` becomes
  `<div></div>`, retaining the required closing tag.
- Collapses whitespace-only runs between inline tags to one space, without
  introducing a separator where none existed. For example,
  `<span>Hello</span>\n  <span>world</span>` becomes
  `<span>Hello</span> <span>world</span>`.
- Preserves non-whitespace text and leading/trailing whitespace at static-run
  boundaries, where a dynamic continuation might follow. At the end of a
  template in a known HTML context, removes trailing HTML whitespace after a
  known block boundary. Inline, protected, and unknown-context endings retain
  their whitespace.
- Preserves complete static RCDATA and raw-text elements byte-for-byte while
  compacting surrounding whitespace. Shared handling covers `title`, `textarea`,
  `style`, `xmp`, `iframe`, `noembed`, and `noframes`; their contents, entities,
  and tag spelling remain unchanged. Incomplete or unsupported element syntax
  still ends compaction. Scripts remain barriers because their escaped and
  double-escaped parsing states require different handling.
- Preserves incomplete tags and the remainder of a static run once it reaches
  unsupported syntax, comments, or protected containers such as `<pre>`,
  `<listing>`, `<dialog>`, `<script>`, SVG, and MathML. An element
  with a `hidden` attribute also ends compaction, preserving the preceding
  whitespace and remaining static output so invisible elements cannot join
  surrounding words. The ordinary `<!DOCTYPE html>`
  declaration is recognized case-insensitively and preserved.
- Continues compacting static HTML after expressions in a known HTML text
  context, entirely at compile time. Leading and trailing whitespace at each
  static-run boundary is still preserved. The compiler scans the template's
  static markup before allowing this continuation. Unsupported or protected
  regions, including a closing `</pre>` whose opener may be dynamic, prevent
  compaction after expressions in that template. There are no runtime context
  flags or alternate literals.
- Treats arbitrary Rust code, expressions accessing `writer`, section creation,
  child views, and unresolved `@render` contexts as barriers. Protected or
  incomplete static HTML also prevents compaction across subsequent expressions.
  Later static output remains unmodified, though adjacent static writes can
  still be merged.
- Resolves HTML layouts at compile time using the same paths, fallback order,
  and output kind as runtime resolution. A static layout with exactly one default
  `@render` in normal HTML text context allows compaction of its child and its own
  suffix, including safe terminal whitespace removal. Every ancestor layout must
  satisfy the same check. Protected, dynamic, missing, or cyclic layouts retain
  the conservative behavior; missing layouts still produce runtime errors.
  Section buffers and other layout insertion contexts remain unknown.
- Compacts fully static standalone JSON templates by removing only the four JSON
  whitespace characters (space, tab, CR, and LF) outside strings. It validates
  the complete JSON value, including object/array structure, numbers, literals,
  and string escapes. Invalid static JSON produces a compile error when this
  optimization is enabled. Diagnostics refer to the original template locations,
  including after removed template comments and escaped `@@` characters.
- Preserves JSON string contents and escape spelling, number spelling/precision,
  object-member order, and duplicate keys. It accepts any JSON root value and
  supports up to 128 nested containers, reporting an error beyond that limit.
- Leaves all dynamic JSON templates unminified, including static output before
  the first dynamic operation. Sections and layout-backed JSON fragments also
  remain unminified: compaction must not guess whether a fragment is inside a
  string or next to a partial token. Static writes can still be merged.
- Leaves optimization-disabled generation and plain-text output unchanged.

For example, static JSON `{ "text" : " a  b ", "items" : [ 1, true ] }`
becomes `{"text":" a  b ","items":[1,true]}`. This does not add JSON escaping
to dynamic interpolation or validate the JSON produced by runtime Rust code;
those behaviors are unchanged.

The scanner assumes a standalone HTML template starts in normal HTML text
context. If rendered HTML is embedded into another language or a raw-text
container by application code, disable this optimization for that compilation.
Code generation cannot infer arbitrary callers' insertion contexts.

Dynamic raw HTML must preserve the surrounding parsing context unless protected
boundaries are visible in the static template. Complete raw fragments such as
`<pre> a  b </pre>` preserve their own contents and allow compaction afterwards.
An opening `<pre>` returned by one expression and a closing `</pre>` returned by
another cannot be detected at compile time; disable optimization for such
templates. This contract also applies to custom values that bypass HTML escaping.
Default-section content inserted into a statically analyzed HTML layout must
likewise preserve the surrounding parsing context. If application code supplies
an unclosed raw-text or protected element as a default section, disable optimization.

Whitespace reduction uses the normal HTML whitespace and default element-display
contract. It does not evaluate CSS. Disable optimization if application styles
make ordinary tag-boundary whitespace significant, for example with inherited
`white-space: pre-wrap` or block elements styled as inline. Whitespace-only DOM
text nodes can change, so code that depends on those nodes or whitespace-sensitive
selectors must also opt out.

Compared with the old DOM-round-trip optimizer, non-whitespace text and fragment
edges are no longer trimmed, comments are no longer removed, and attribute
quoting is no longer rewritten. Invalid markup is preserved rather than repaired.
Runtime minification, compression, string pooling, and control-flow analysis are
not part of this pass.

Compilation exposes measurements through `CompileResult::static_output_stats()`:

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

These are generated static-output measurements, not counts of writes executed
by loops or arbitrary user-written Rust. Distinct literal bytes are aggregated
across compiled templates; they are not a prediction of linker storage.

### Define what "smaller" means

Measure these separately:

- Total UTF-8 bytes in generated output literals.
- Distinct output-literal bytes, to estimate the benefit of exact deduplication.
- Generated write-call count.
- Data-section size and total size of a representative release executable.

Shorter generated Rust source does not necessarily mean a smaller executable.
For example, writing a newline as `\n` changes the source representation, not the
one byte stored in the string. Likewise, replacing a literal with a `const`
does not inherently save bytes.

Compile the same application with the same target, profile, features, linker,
and debug-information settings for before/after comparisons. Measure an actual
executable that uses the templates, not just the library archive.

### Why fragment context matters

The [HTML optimizer](../../rust/core/lib/src/codegen/compiler/optimizer/html.rs)
previously parsed and reserialized each fragment independently. The
[code-generation context](../../rust/core/lib/src/codegen/compiler/context.rs)
now tracks whether HTML context is known and shares static-output measurements
with section contexts. Layout and default-render safety decisions are passed
directly into code generation rather than stored in that mutable context.

Consider this template:

```html
<div title="@value">hello</div>
```

Its output is split around the dynamic expression:

```text
Static:  <div title="
Dynamic: value
Static:  ">hello</div>
```

The second static fragment starts inside a quoted attribute, not at the start of
an HTML document. Parsing it as standalone HTML cannot reliably determine what
is safe to change.

Simply concatenating the static fragments while omitting the expression is not
safe either: the dynamic value can contain markup or quote characters under the
current raw-output behavior.

### Implemented pipeline

Perform optimization before converting output blocks into Rust token streams:

```text
Parsed template blocks, retaining source locations
    -> identify ordered output regions and dynamic barriers
    -> combine adjacent static output within each region
    -> optimize regions whose HTML context is known
    -> emit Rust literals and write calls
```

The pass uses the existing
[block representation](../../rust/core/lib/src/codegen/types/block.rs), not a new
general-purpose intermediate language. It keeps original source spans for
diagnostics rather than replacing parser input with minified text.

The [content generator](../../rust/core/lib/src/codegen/compiler/types/content.rs)
and [section generator](../../rust/core/lib/src/codegen/compiler/types/section.rs)
use the same static-output machinery. Section buffers are separate output
streams; their eventual insertion context may be unknown.

#### Merge adjacent static output

Conceptually, these writes can become one:

```rust
writer.write("<div>");
writer.write("hello");
writer.write("</div>");
```

```rust
writer.write("<div>hello</div>");
```

Only merge consecutive writes to the same destination when nothing between them
has runtime effects. Do not move output across expressions, Rust statements,
branches, loops, section creation, or layout rendering.

This primarily reduces write calls and generated code. It does not necessarily
reduce literal bytes, and joining literals can reduce opportunities for sharing
repeated substrings. Benchmark both executable size and render time.

#### Carry context, but stop making assumptions at dynamic barriers

For continuous, known static output, retain enough parser state to distinguish:

- Text from tag syntax.
- Tag names, attribute names, and quoted or unquoted attribute values.
- Comments and declarations.
- Raw-text and whitespace-sensitive content, including relevant ancestor context.
- Incomplete tokens split across static blocks.

A lexical state alone is not a complete HTML parser. Transformations that depend
on tree structure need HTML-aware parsing, including special handling for
foreign content such as SVG.

For the first implementation, treat arbitrary Rust code, raw interpolation, and
rendered sections as barriers:

1. Emit preceding static content without transformations that require knowing
   the missing continuation.
2. Mark the subsequent HTML context as unknown.
3. Preserve later content wherever a transformation requires that context.

Do not reset to "normal HTML text" after a barrier without proof. A dynamic value
could open a tag, close a quote, or start a whitespace-sensitive element. Arbitrary
Rust code can also write directly to the writer.

A future distinction between escaped text and explicitly raw HTML could allow
more optimization. It would require a separately designed, context-appropriate
escaping contract; it must not be assumed or silently introduced here.

Branches and loops need the same caution. If later work analyzes them, optimize
their outputs separately and only propagate a known exit state when every path
supports it. Initially, keeping them as barriers is simpler and safer.

### Choose transformations by their safety requirements

| Transformation | Conservative policy |
| --- | --- |
| Redundant whitespace between attributes | Reduce only when parsing proves it is tag syntax, outside values, with required separators retained |
| Whitespace-only runs between understood tags | Remove at standard block boundaries; collapse to one space between inline tags under normal HTML whitespace rules |
| Non-whitespace text and fragment edges | Preserve; dynamic continuations can need separators |
| Whitespace inside attribute values | Preserve; it can be application data |
| Empty non-void elements | Keep paired tags, such as `<div></div>`, rather than emitting `<div/>` |
| Attribute quote changes | Preserve value semantics and existing entities; escape any newly conflicting delimiter |
| HTML comments | Preserve by default; removal requires an explicit policy for application/tool directives |
| Script/style content | Preserve unless a dedicated language-aware optimizer is explicitly enabled |
| Partial or unknown-context fragments | Preserve bytes when safety cannot be established |

These constraints also guide future extensions. The scanner compacts supported
tag syntax and inter-tag whitespace, and preserves the rest once context is
uncertain.

For example, do not remove the space between these elements:

```html
<span>Hello</span> <span>world</span>
```

Removing the space changes the displayed text.

Likewise, preserve the trailing space in this fragment:

```text
<div att1="a">test 
```

If dynamic output later supplies `world</div>`, removing that space would join
the words. Even reducing multiple spaces to one requires a whitespace policy:
CSS such as `white-space: pre-wrap` can preserve those spaces.

### Deduplication and compression come later

First inspect whether identical literals actually occupy duplicate storage in
the release executable. Rust and the linker may already share them.

If measurement shows substantial duplication, consider sharing exact literals
before attempting substring pooling. Substring pooling needs offsets, extra
metadata, and UTF-8-safe boundaries. Neither technique guarantees a smaller
executable after code and metadata overhead.

Avoid embedded-string compression initially. It reduces stored bytes but adds
decompression code, CPU cost, and possibly allocations. It is most plausible for
large, rarely used static templates, not as the default rendering path.

### Incremental delivery and acceptance checks

1. Add literal-byte and write-count measurements alongside executable-size
   baselines.
2. Implement adjacent static-write merging without changing output bytes.
3. Make unknown-context fragments and dynamic barriers explicit. Preserve source
   diagnostics and expose skipped-optimization reasons in optional diagnostics.
4. Add narrowly scoped HTML transformations with focused correctness tests.
5. Consider more advanced context analysis only after measuring the remaining
   opportunities.

Keep the optimization-disabled path unchanged. Before tightening existing
optimization behavior, document the affected output differences.

Regression cases should include:

- Quoted attributes and tag tokens split across output blocks.
- Leading and trailing spaces around dynamic expressions and inline elements.
- `<pre>`, `<textarea>`, script/style content, and CSS-sensitive whitespace.
- Empty ordinary elements, void elements, and custom-element closing tags.
- Attribute values containing quotes and existing character references.
- Dynamic raw markup, branch/loop output, and sections inserted through layouts.
- Unchanged JSON/text templates.

For output-preserving passes, assert exact byte equality. For intentional
minification, assert exact expected output and use an HTML parser/browser to
check affected structure and text behavior. Browser checks should include
relevant whitespace CSS; DOM structure alone is not sufficient.

Accept an optimization only when the intended metric improves on representative
templates without unacceptable build-time or runtime regressions. Do not infer
binary-size wins solely from fewer source characters or write calls.

### Initial tag-syntax-only measurements

Before the inter-tag whitespace extension, local Windows release builds compared
the same probe executable with code generation disabled and enabled, keeping the
target and profile unchanged:

| Input | Static literal bytes | Static write calls | Executable bytes | `.rdata` virtual bytes |
| --- | --- | --- | --- | --- |
| Synthetic tag-whitespace fixture | 2,749 -> 589 | 2 -> 1 | 160,768 -> 158,208 | 42,610 -> 40,434 |
| Existing benchmark templates | 768 -> 768 | 22 -> 22 | 183,808 -> 183,808 | 45,132 -> 45,132 |

The synthetic fixture deliberately contained twenty ordinary `div` tags with
redundant spaces around attributes and a template comment splitting static
output. It demonstrates a possible reduction, not a typical application result.
At that stage, the existing benchmark templates had no savings; their dynamic
regions and formatting were preserved. Rendering the existing large benchmark
template produced 139,047 bytes with either setting. These historical figures
do not measure the subsequent inter-tag whitespace reduction.

Focused tests check exact generated literals and output boundaries. Browser
checks of representative tag compactions also confirmed identical DOM
serialization and text, including quoted values, entities, unquoted values
followed by slashes, inline spacing, and unchanged text with
`white-space: pre-wrap`. Inter-tag whitespace reduction instead follows the
normal-whitespace contract described above.

## 2. Runtime: reduce rendered output

The current [HTML writer](../../rust/core/lib/src/runtime/html_writer.rs) simply
appends content.

Two possible approaches:

| Approach | Benefit | Cost |
| --- | --- | --- |
| Minify completed output | Sees the complete document; simplest integration | Extra parsing and typically another allocation |
| Stateful streaming minifier | Can avoid a full second buffer | Much harder to handle boundaries and arbitrary dynamic markup |

Start with optional final-output minification using an HTML-aware implementation.
Add streaming only if measurements justify the complexity.

Constraints:

- Preserve whitespace-sensitive content, including `<pre>` and `<textarea>`.
- Handle scripts and styles using their own parsing rules, not generic HTML
  whitespace rules.
- CSS can make whitespace significant elsewhere, so provide conservative behavior
  and an opt-out.
- Do not modify JSON or text output with HTML rules.

For web applications, HTTP Brotli or gzip compression often saves more transfer
bytes without changing HTML semantics. This reduces wire size, not the
uncompressed rendered string.

## 3. String operations: reduce allocations and copies

This is the best first optimization target.

### Eliminate temporary interpolation strings

Current [inline code generation](../../rust/core/lib/src/codegen/compiler/types/code.rs)
emits:

```rust
writer.write(&value.to_string());
```

Add a formatting operation to the
[writer interface](../../rust/core/lib/src/types/writer.rs), allowing values to
format directly into the destination buffer. For example, a proposed API is:

```rust
writer.write_display(&value)?;
```

Preserve existing error reporting and HTML-escaping behavior. Formatting is not
escaping. This API is a proposal, not an existing method.

### Reserve output capacity

Generate an initial capacity estimate from unconditional static content:

```rust
String::with_capacity(static_size_estimate)
```

Avoid summing every branch or assuming loop iteration counts: that can
substantially overallocate.

### Reduce intermediate buffers

- Keep sections buffered where layouts require deferred placement.
- Avoid repeatedly constructing complete intermediate strings when composition
  permits direct writing.

## Suggested implementation order

1. Establish benchmarks for render time, allocation count and bytes, output bytes,
   and release binary size.
2. Implement direct formatting and capacity reservation.
3. Merge static writes and make compile-time optimization context-aware.
4. Add optional runtime minification.
5. Consider string pooling or compression only if binary-size measurements justify
   it.

Extend the existing
[benchmarks](../../rust/benchmark/benches/template_benches.rs) with
interpolation-heavy loops, many small fragments, and nested layouts, not just
large static templates.
