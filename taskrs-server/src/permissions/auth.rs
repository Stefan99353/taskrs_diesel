use taskrs_db::models::permission::Permission;

lazy_static! {
    pub static ref AUTH_REVOKE_REFRESH_TOKEN: Permission = Permission {
        id: 0,
        name: "auth_revoke_refresh_token".to_string(),
        group: "auth".to_string(),
        description: Some("Allows a user to revoke refresh tokens of other users".to_string()),
        updated_at: None,
        created_at: None,
    };
}
