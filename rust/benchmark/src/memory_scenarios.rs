use crate::{bench_views, view_models::TableModel};
use stats_alloc::{Region, StatsAlloc};
use std::{alloc::System, hint::black_box};

fn measure<T>(name: &str, allocator: &StatsAlloc<System>, mut render: impl FnMut() -> T) {
    drop(black_box(render()));
    let region = Region::new(allocator);
    drop(black_box(render()));
    let stats = region.change();
    println!(
        "{name}: allocations={}, reallocations={}, allocated_bytes={}",
        stats.allocations, stats.reallocations, stats.bytes_allocated
    );
}

pub fn big_data_table(allocator: &StatsAlloc<System>) {
    let mut context = sbolt::context!(TableModel::large());
    let template = bench_views::resolve("views/data_table", None).unwrap();

    measure("Large data table(resolved)", allocator, || {
        template.render(&mut context).unwrap()
    });
    measure("Large data table(lookup)", allocator, || {
        bench_views::render("views/data_table", None, &mut context).unwrap()
    });
}
