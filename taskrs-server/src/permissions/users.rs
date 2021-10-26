use taskrs_db::models::permission::Permission;

lazy_static! {
    pub static ref USER_GET_ALL: Permission = Permission {
        id: 0,
        name: "user_get_all".to_string(),
        group: "user".to_string(),
        description: Some("Allows a user to get information from other users".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref USER_CREATE: Permission = Permission {
        id: 0,
        name: "user_create".to_string(),
        group: "user".to_string(),
        description: Some("Allows a user to create new users".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref USER_DELETE: Permission = Permission {
        id: 0,
        name: "user_delete".to_string(),
        group: "user".to_string(),
        description: Some("Allows a user to delete users".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref USER_UPDATE: Permission = Permission {
        id: 0,
        name: "user_update".to_string(),
        group: "user".to_string(),
        description: Some("Allows a user to update users".to_string()),
        updated_at: None,
        created_at: None,
    };
}
