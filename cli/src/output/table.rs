use tabled::{Table, Tabled};
use tabled::settings::Style;

use super::colors;

pub fn print_table<T: Tabled>(rows: &[T], title: Option<&str>) {
    let mut table = Table::new(rows);
    table.with(Style::rounded());

    if let Some(t) = title {
        println!("\n{}", colors::header(t));
    }
    println!("{}", table);
}

#[derive(Tabled)]
pub struct KeyValueRow {
    #[tabled(rename = "Key")]
    pub key: String,
    #[tabled(rename = "Value")]
    pub value: String,
    #[tabled(rename = "Updated")]
    pub updated: String,
}

#[derive(Tabled)]
pub struct ProjectRow {
    #[tabled(rename = "Project")]
    pub project: String,
    #[tabled(rename = "Environments")]
    pub environments: String,
    #[tabled(rename = "Keys")]
    pub key_count: String,
}

#[derive(Tabled)]
pub struct StatusRow {
    #[tabled(rename = "Field")]
    pub field: String,
    #[tabled(rename = "Value")]
    pub value: String,
}
