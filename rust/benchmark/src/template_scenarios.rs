sbolt::include_views!();

use crate::view_models::{PageModel, TableModel, Team, TeamsModel};

pub fn big_data_table(b: &mut criterion::Bencher<'_>, resolved: &bool) {
    let model = TableModel::large();
    let mut context = sbolt::context!(model);
    match *resolved {
        true => {
            let template = bench_views::resolve("views/data_table", None).unwrap();
            b.iter(|| template.render(&mut context));
        }
        false => {
            b.iter(|| bench_views::render("views/data_table", None, &mut context));
        }
    };
}

pub fn team_list(b: &mut criterion::Bencher<'_>, resolved: &bool) {
    let teams = vec![
        Team {
            name: "Jiangsu".into(),
            score: 43,
        },
        Team {
            name: "Beijing".into(),
            score: 27,
        },
        Team {
            name: "Guangzhou".into(),
            score: 22,
        },
        Team {
            name: "Shandong".into(),
            score: 12,
        },
    ];

    let model = TeamsModel { teams };
    let mut context = sbolt::context!(model);
    match *resolved {
        true => {
            let template = bench_views::resolve("views/team_list", None).unwrap();
            b.iter(|| template.render(&mut context));
        }
        false => {
            b.iter(|| bench_views::render("views/team_list", None, &mut context));
        }
    };
}

pub fn model_page(b: &mut criterion::Bencher<'_>, resolved: &bool) {
    let model = PageModel {
        name: "Alice".into(),
        age: 30,
        msg: "Hello".into(),
    };
    let mut context = sbolt::context!(model);
    match *resolved {
        true => {
            let template = bench_views::resolve("views/model_page", None).unwrap();
            b.iter(|| template.render(&mut context));
        }
        false => {
            b.iter(|| bench_views::render("views/model_page", None, &mut context));
        }
    };
}

pub fn layout_page(b: &mut criterion::Bencher<'_>, resolved: &bool) {
    let model = PageModel {
        name: "Alice".into(),
        age: 30,
        msg: "Hello".into(),
    };
    let mut context = sbolt::context!(model);
    match *resolved {
        true => {
            let template = bench_views::resolve("views/sectioned_page", None).unwrap();
            b.iter(|| template.render(&mut context));
        }
        false => {
            b.iter(|| bench_views::render("views/sectioned_page", None, &mut context));
        }
    };
}
