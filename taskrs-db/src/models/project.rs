use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, OptionalExtension, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::schema::{project_members, projects};
use crate::DbConnection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProjectColumns {
    Id,
    Name,
    Description,
    CategoryId,
    OwnerId,
    CreatorId,
    UpdatedAt,
    CreatedAt,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub owner_id: i32,
    pub creator_id: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl Project {
    pub fn insert(self, conn: &DbConnection) -> diesel::QueryResult<Project> {
        let new_project: NewProject = self.into();
        diesel::insert_into(projects::table)
            .values(new_project)
            .get_result(conn)
    }

    pub fn exists(&self, conn: &DbConnection) -> diesel::QueryResult<bool> {
        projects::table
            .filter(projects::name.eq(&self.name))
            .first::<Self>(conn)
            .optional()
            .map(|project| project.is_some())
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "projects"]
struct NewProject {
    pub name: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub owner_id: i32,
    pub creator_id: Option<i32>,
}

impl From<Project> for NewProject {
    fn from(
        Project {
            name,
            description,
            category_id,
            owner_id,
            creator_id,
            ..
        }: Project,
    ) -> Self {
        Self {
            name,
            description,
            category_id,
            owner_id,
            creator_id,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct ProjectMember {
    pub project_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
    pub updated_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
}

impl ProjectMember {
    pub fn new(project_id: i32, user_id: i32, is_admin: bool) -> Self {
        Self {
            project_id,
            user_id,
            is_admin,
            updated_at: None,
            created_at: None,
        }
    }

    pub fn insert(self, conn: &DbConnection) -> diesel::QueryResult<ProjectMember> {
        let new_project_member: NewProjectMember = self.into();

        diesel::insert_into(project_members::table)
            .values(new_project_member)
            .get_result(conn)
    }
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "project_members"]
pub struct NewProjectMember {
    pub project_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

impl From<ProjectMember> for NewProjectMember {
    fn from(
        ProjectMember {
            project_id,
            user_id,
            is_admin,
            ..
        }: ProjectMember,
    ) -> Self {
        Self {
            project_id,
            user_id,
            is_admin,
        }
    }
}
