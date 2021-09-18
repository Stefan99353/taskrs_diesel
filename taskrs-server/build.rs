use serde::Deserialize;
use glob::glob;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

fn main() {
    let permissions: Vec<Permission> = glob("permissions/**/*.json").unwrap()
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|path| {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            serde_json::from_reader::<_, Vec<Permission>>(reader).unwrap()
        })
        .flatten()
        .collect();

    // SQL migrations
    let migration_content: String = permissions
        .iter()
        .map(|p| {
            match &p.description {
                None => {
                    format!("INSERT INTO permissions (name, \"group\") VALUES ('{}', '{}');", p.name, p.group)
                }
                Some(desc) => {
                    format!("INSERT INTO permissions (name, \"group\", description) VALUES ('{}', '{}', '{}');", p.name, p.group, desc)
                }
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Create migration directory
    std::fs::create_dir_all("migrations/99999999999999_permissions").unwrap();
    let mut up_file = OpenOptions::new().write(true).truncate(true).create(true).open("migrations/99999999999999_permissions/up.sql").unwrap();
    let mut down_file = OpenOptions::new().write(true).truncate(true).create(true).open("migrations/99999999999999_permissions/down.sql").unwrap();
    let down_statement = String::from("DELETE FROM permissions;");

    // Write INSERT statements
    up_file.write_all(migration_content.as_bytes()).unwrap();
    down_file.write_all(down_statement.as_bytes()).unwrap();

    // Permission constants
    let constants: String = permissions
        .iter()
        .map(|p| {
            format!(
                "pub const {}: &str = \"{}\";",
                p.name.to_uppercase(),
                p.name
            );
        })
        .collect::<Vec<String>>()
        .join("\n");

    let mut permissions_file = OpenOptions::new().write(true).truncate(true).create(true).open("src/permissions.rs").unwrap();
    permissions_file.write_all(constants.as_bytes()).unwrap();
}

#[derive(Debug, Default, Clone, Deserialize)]
struct Permission {
    pub name: String,
    pub group: String,
    pub description: Option<String>,
}