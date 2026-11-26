use criterion::{Criterion, criterion_group, criterion_main};
use lib_benchmark::template_scenarios;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Benchmark");
    // Large data table benchmarks
    group.bench_with_input(
        "Large data table(resolved)",
        &true,
        template_scenarios::big_data_table,
    );
    group.bench_with_input(
        "Large data table(lookup)",
        &false,
        template_scenarios::big_data_table,
    );
    // teams
    group.bench_with_input("Team list(resolved)", &true, template_scenarios::team_list);
    group.bench_with_input("Team list(lookup)", &false, template_scenarios::team_list);
    // model page
    group.bench_with_input(
        "Model page(resolved)",
        &true,
        template_scenarios::model_page,
    );
    group.bench_with_input("Model page(lookup)", &false, template_scenarios::model_page);

    // layout page.
    group.bench_with_input(
        "Layout page(resolved)",
        &true,
        template_scenarios::layout_page,
    );
    group.bench_with_input(
        "Layout page(lookup)",
        &false,
        template_scenarios::layout_page,
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
