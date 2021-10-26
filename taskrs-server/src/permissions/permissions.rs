use taskrs_db::models::permission::Permission;

lazy_static! {
    pub static ref PERMISSION_GET_ALL: Permission = Permission {
        id: 0,
        name: "permission_get_all".to_string(),
        group: "permission".to_string(),
        description: Some("Allows a user to get all permissions".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref PERMISSION_SET: Permission = Permission {
        id: 0,
        name: "permission_set".to_string(),
        group: "permission".to_string(),
        description: Some("Allows a user to set a users permissions".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref PERMISSION_GRANT: Permission = Permission {
        id: 0,
        name: "permission_grant".to_string(),
        group: "permission".to_string(),
        description: Some("Allows a user to grant a user permissions".to_string()),
        updated_at: None,
        created_at: None,
    };
    pub static ref PERMISSION_REVOKE: Permission = Permission {
        id: 0,
        name: "permission_revoke".to_string(),
        group: "permission".to_string(),
        description: Some("Allows a user to revoke a users permissions".to_string()),
        updated_at: None,
        created_at: None,
    };
}
