use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CompanyForm {
    pub csrf: String,
    pub name: String,
    pub address: String,
    pub contact_no: String,
}

#[derive(Deserialize)]
pub(crate) struct EmployeeForm {
    pub csrf: String,
    pub name: String,
    pub contact_no: String,
    pub email: String,
    pub comp_id: String,
}

#[derive(Deserialize)]
pub(crate) struct DeleteForm {
    pub csrf: String,
}

#[derive(Default, Deserialize)]
pub(crate) struct ListQuery {
    pub saved: Option<String>,
}
