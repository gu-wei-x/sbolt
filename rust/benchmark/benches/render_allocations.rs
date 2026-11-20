use stats_alloc::{INSTRUMENTED_SYSTEM, Region, Stats, StatsAlloc};
use std::{alloc::System, hint::black_box};

#[global_allocator]
static ALLOCATOR: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn measure<T>(name: &str, mut render: impl FnMut() -> T) -> Stats {
    drop(black_box(render()));
    let region = Region::new(&ALLOCATOR);
    drop(black_box(render()));
    let stats = region.change();
    println!(
        "{name}: allocations={}, reallocations={}, allocated_bytes={}",
        stats.allocations, stats.reallocations, stats.bytes_allocated
    );
    stats
}

fn main() {
    let mut teams = lib_bench::teams_context();
    let mut table = lib_bench::table_context();
    let teams_handle = lib_bench::resolve_teams();
    let table_handle = lib_bench::resolve_table();
    measure("teams/named/fresh", || lib_bench::render_teams(&mut teams));
    measure("big_table/named/fresh", || {
        lib_bench::render_table(&mut table)
    });
    measure("teams/resolved/fresh", || {
        teams_handle.render(&mut teams).unwrap()
    });
    measure("big_table/resolved/fresh", || {
        table_handle.render(&mut table).unwrap()
    });
    let mut estimated_teams = lib_bench::teams_context();
    estimated_teams.set_capacity_hint(128);
    let mut estimated_table = lib_bench::table_context();
    estimated_table.set_capacity_hint(140_000);
    let stats = measure("teams/resolved/fresh_estimated", || {
        teams_handle.render(&mut estimated_teams).unwrap()
    });
    assert_eq!((stats.allocations, stats.reallocations), (1, 0));
    let stats = measure("big_table/resolved/fresh_estimated", || {
        table_handle.render(&mut estimated_table).unwrap()
    });
    assert_eq!((stats.allocations, stats.reallocations), (1, 0));
    let model = std::sync::Arc::new(lib_bench::teams_model());
    let mut typed_context = sbolt::context!(model: std::sync::Arc::clone(&model));
    let model_handle = lib_bench::resolve_teams_model();
    measure("teams/model/named_fresh", || {
        lib_bench::render_teams_model_named(&mut typed_context)
    });
    measure("teams/model/resolved_fresh", || {
        model_handle.render(&mut typed_context).unwrap()
    });
    measure("teams/model/concrete_fresh", || {
        lib_bench::render_teams_model(&mut typed_context)
    });
    let mut estimated_model_context = sbolt::context!(model: std::sync::Arc::clone(&model));
    estimated_model_context.set_capacity_hint(128);
    let stats = measure("teams/model/concrete_fresh_estimated", || {
        lib_bench::render_teams_model(&mut estimated_model_context)
    });
    assert_eq!((stats.allocations, stats.reallocations), (1, 0));
    measure("teams/resolution_only", lib_bench::resolve_teams_only);
    measure("big_table/resolution_only", lib_bench::resolve_table_only);
    measure("big_table/formatting_only", lib_bench::format_table_values);
    measure(
        "big_table/specialized_formatting_only",
        lib_bench::format_table_values_specialized,
    );
}
