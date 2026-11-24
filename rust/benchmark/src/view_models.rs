pub(crate) struct Team {
    pub(crate) name: String,
    pub(crate) score: i32,
}

pub(crate) struct TeamsModel {
    pub(crate) teams: Vec<Team>,
}

pub(crate) struct TableModel {
    pub(crate) rows: Vec<Vec<i32>>,
}

pub(crate) struct PageModel {
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) msg: String,
}

impl TableModel {
    pub(crate) fn large() -> Self {
        const SIZE: usize = 100;
        let mut rows = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            let mut row = Vec::with_capacity(SIZE);
            for value in 0..SIZE {
                row.push(i32::try_from(value).unwrap());
            }
            rows.push(row);
        }
        Self { rows }
    }
}
