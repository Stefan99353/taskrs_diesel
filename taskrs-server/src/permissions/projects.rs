use taskrs_db::models::permission::Permission;

lazy_static! {
    pub static ref PROJECT_GET_ALL: Permission = Permission {
        id: 0,
        name: "project_get_all".to_string(),
        group: "project".to_string(),
        description: Some("Allows a user to get all projects".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref PROJECT_CREATE: Permission = Permission {
        id: 0,
        name: "project_create".to_string(),
        group: "project".to_string(),
        description: Some("Allows a user to create new projects".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref PROJECT_DELETE: Permission = Permission {
        id: 0,
        name: "project_delete".to_string(),
        group: "project".to_string(),
        description: Some("Allows a user to delete projects".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref PROJECT_UPDATE: Permission = Permission {
        id: 0,
        name: "project_update".to_string(),
        group: "project".to_string(),
        description: Some("Allows a user to update projects".to_string()),
        updated_at: None,
        created_at: None,
    };
}
