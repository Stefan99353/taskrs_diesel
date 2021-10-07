use chrono::NaiveDateTime;
use diesel::{Insertable, PgConnection, Queryable};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::{projects, project_members};
use diesel::pg::Pg;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProjectColumns {
    Id,
    Name,
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
    pub fn insert(self, conn: &PgConnection) -> diesel::QueryResult<Project> {
        use crate::db::schema::projects::dsl::*;

        let new_project: NewProject = self.into();
        diesel::insert_into(projects)
            .values(new_project)
            .get_result(conn)
    }

    pub fn exists(&self, conn: &PgConnection) -> diesel::QueryResult<bool> {
        use crate::db::schema::projects::dsl::*;

        projects
            .filter(name.eq(&self.name))
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
    fn from(Project {name, description, category_id, owner_id, creator_id, ..}: Project) -> Self {
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

    pub fn insert(self, conn: &PgConnection) -> diesel::QueryResult<ProjectMember> {
        use super::schema::project_members::dsl::*;

        let new_project_member: NewProjectMember = self.into();

        diesel::insert_into(project_members)
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
    fn from(ProjectMember { project_id, user_id, is_admin, .. }: ProjectMember) -> Self {
        Self {
            project_id,
            user_id,
            is_admin
        }
    }
}