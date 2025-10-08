use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render_view_with_layout", |b| {
        b.iter(|| {
            lib_bench::render_template_with_layout();
        });
    });
    c.bench_function("render_view_without_layout", |b| {
        b.iter(|| {
            lib_bench::render_template_without_layout();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
