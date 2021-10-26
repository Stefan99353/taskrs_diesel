use taskrs_db::models::permission::Permission;

lazy_static! {
    pub static ref CATEGORY_GET_ALL: Permission = Permission {
        id: 0,
        name: "category_get_all".to_string(),
        group: "category".to_string(),
        description: Some("Allows a user to get all categories".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref CATEGORY_CREATE: Permission = Permission {
        id: 0,
        name: "category_create".to_string(),
        group: "category".to_string(),
        description: Some("Allows a user to create new categories".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref CATEGORY_DELETE: Permission = Permission {
        id: 0,
        name: "category_delete".to_string(),
        group: "category".to_string(),
        description: Some("Allows a user to delete categories".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref CATEGORY_UPDATE: Permission = Permission {
        id: 0,
        name: "category_update".to_string(),
        group: "category".to_string(),
        description: Some("Allows a user to update categories".to_string()),
        updated_at: None,
        created_at: None,
    };
}
