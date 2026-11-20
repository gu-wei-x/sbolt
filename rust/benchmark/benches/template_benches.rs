use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let mut teams = lib_bench::teams_context();
    let mut table = lib_bench::table_context();
    let teams_handle = lib_bench::resolve_teams();
    let table_handle = lib_bench::resolve_table();
    c.bench_function("teams/named/fresh", |b| {
        b.iter(|| black_box(lib_bench::render_teams(black_box(&mut teams))));
    });
    c.bench_function("big_table/named/fresh", |b| {
        b.iter(|| black_box(lib_bench::render_table(black_box(&mut table))));
    });
    c.bench_function("teams/resolved/fresh", |b| {
        b.iter(|| black_box(teams_handle.render(black_box(&mut teams)).unwrap()));
    });
    c.bench_function("big_table/resolved/fresh", |b| {
        b.iter(|| black_box(table_handle.render(black_box(&mut table)).unwrap()));
    });
    let mut estimated_teams = lib_bench::teams_context();
    estimated_teams.set_capacity_hint(128);
    let mut estimated_table = lib_bench::table_context();
    estimated_table.set_capacity_hint(140_000);
    c.bench_function("teams/resolved/fresh_estimated", |b| {
        b.iter(|| {
            black_box(
                teams_handle
                    .render(black_box(&mut estimated_teams))
                    .unwrap(),
            )
        });
    });
    c.bench_function("big_table/resolved/fresh_estimated", |b| {
        b.iter(|| {
            black_box(
                table_handle
                    .render(black_box(&mut estimated_table))
                    .unwrap(),
            )
        });
    });
    let model = std::sync::Arc::new(lib_bench::teams_model());
    let mut typed_context = sbolt::context!(model: std::sync::Arc::clone(&model));
    let model_handle = lib_bench::resolve_teams_model();
    c.bench_function("teams/model/named_fresh", |b| {
        b.iter(|| {
            black_box(lib_bench::render_teams_model_named(black_box(
                &mut typed_context,
            )))
        });
    });
    c.bench_function("teams/model/resolved_fresh", |b| {
        b.iter(|| black_box(model_handle.render(black_box(&mut typed_context)).unwrap()));
    });
    c.bench_function("teams/model/concrete_fresh", |b| {
        b.iter(|| black_box(lib_bench::render_teams_model(black_box(&mut typed_context))));
    });
    let mut estimated_model_context = sbolt::context!(model: std::sync::Arc::clone(&model));
    estimated_model_context.set_capacity_hint(128);
    c.bench_function("teams/model/concrete_fresh_estimated", |b| {
        b.iter(|| {
            black_box(lib_bench::render_teams_model(black_box(
                &mut estimated_model_context,
            )))
        });
    });
    c.bench_function("teams/resolution_only", |b| {
        b.iter(|| black_box(lib_bench::resolve_teams_only()));
    });
    c.bench_function("big_table/resolution_only", |b| {
        b.iter(|| black_box(lib_bench::resolve_table_only()));
    });
    c.bench_function("big_table/formatting_only", |b| {
        b.iter(|| black_box(lib_bench::format_table_values()));
    });
    c.bench_function("big_table/specialized_formatting_only", |b| {
        b.iter(|| black_box(lib_bench::format_table_values_specialized()));
    });
    c.bench_function("big_table/named/fresh_with_setup", |b| {
        b.iter(|| black_box(lib_bench::render_table_with_setup()));
    });
    c.bench_function("page_with_layout/named/fresh_with_setup", |b| {
        b.iter(|| {
            lib_bench::render_page_with_layout();
        });
    });
    c.bench_function("page/named/fresh_with_setup", |b| {
        b.iter(|| {
            lib_bench::render_page();
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
