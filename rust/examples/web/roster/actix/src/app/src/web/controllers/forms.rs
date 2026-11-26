use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct CompanyForm {
    pub csrf: String,
    pub name: String,
    pub address: String,
    pub contact_no: String,
}

#[derive(Deserialize)]
pub(super) struct EmployeeForm {
    pub csrf: String,
    pub name: String,
    pub contact_no: String,
    pub email: String,
    pub comp_id: String,
}

#[derive(Deserialize)]
pub(super) struct DeleteForm {
    pub csrf: String,
}

#[derive(Deserialize)]
pub(super) struct ListQuery {
    pub saved: Option<String>,
}
