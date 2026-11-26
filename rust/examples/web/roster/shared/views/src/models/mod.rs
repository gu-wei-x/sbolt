use roster_data::{companies, employees};

#[derive(Debug, Default)]
pub struct Company {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub contact_no: String,
}

impl From<companies::Company> for Company {
    fn from(company: companies::Company) -> Self {
        Self {
            id: company.id,
            name: company.name,
            address: company.address,
            contact_no: company.contact_no,
        }
    }
}

#[derive(Debug, Default)]
pub struct Employee {
    pub id: i64,
    pub name: String,
    pub contact_no: String,
    pub email: String,
    pub comp_id: Option<i64>,
    pub company_name: Option<String>,
}

impl From<employees::Employee> for Employee {
    fn from(employee: employees::Employee) -> Self {
        Self {
            id: employee.id,
            name: employee.name,
            contact_no: employee.contact_no,
            email: employee.email,
            comp_id: employee.comp_id,
            company_name: employee.company_name,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SidebarItem {
    pub href: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub active: bool,
}

#[derive(Clone, Copy)]
pub struct Sidebar {
    pub items: [SidebarItem; 3],
}

impl Sidebar {
    pub fn new(active_nav: &str) -> Self {
        Self {
            items: [
                ("/", "Overview", "#icon-overview"),
                ("/companies", "Companies", "#icon-company"),
                ("/employees", "Employees", "#icon-people"),
            ]
            .map(|(href, label, icon)| SidebarItem {
                href,
                label,
                icon,
                active: href == active_nav,
            }),
        }
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new("")
    }
}

#[derive(Default)]
pub struct Page {
    pub title: &'static str,
    pub sidebar: Sidebar,
    pub framework: &'static str,
    pub company_count: i64,
    pub employee_count: i64,
    pub csrf: String,
    pub message: String,
    pub error: String,
    pub companies: Vec<Company>,
    pub employees: Vec<Employee>,
    pub company: Option<Company>,
    pub employee: Option<Employee>,
}
