# Benchmarks

Run these commands from the [core workspace](../../rust/core).

## Rendering time

```shell
cargo bench -p lib_benchmark --bench template_benches
```

Criterion measures the large table, team list, model page, and layout page.
Each compares:

- **Resolved:** resolve once, then render.
- **Lookup:** look up the template on every render.

Setup is outside timed iterations. These benchmarks do not verify rendered output.

## Heap allocations

```shell
cargo bench -p lib_benchmark --bench memory_benches
```

Measures only the 100-by-100 large table, comparing resolved and lookup rendering.
After one warmup render, `stats_alloc` measures one render and reports:

- `allocations`: new heap allocation calls.
- `reallocations`: calls to resize existing allocations.
- `allocated_bytes`: bytes allocated, not peak memory usage.

Setup is excluded. Rendering errors fail the run; allocation counts are reported,
not enforced as thresholds.

## CI checks

[CI](../../.github/workflows/ci.yml) runs both targets in the Linux release job:

```shell
cargo bench -p lib_benchmark --bench memory_benches --bench template_benches -- --test
```

`--` forwards `--test` to the executables. Criterion runs smoke tests without
timing statistics; the memory executable ignores the argument and runs normally.

For performance comparisons, use the same machine, toolchain, and fixtures on a
quiet system. See [optimization](./optimization.md) for more context.
