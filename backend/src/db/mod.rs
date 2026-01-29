use sqlx::PgPool;

use crate::models::{Profile, Project, Skill};

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_profile(&self) -> Result<Option<Profile>, sqlx::Error> {
        sqlx::query_as!(
            Profile,
            "SELECT * FROM profile LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as!(
            Project,
            "SELECT * FROM projects ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_skills(&self) -> Result<Vec<Skill>, sqlx::Error> {
        sqlx::query_as!(
            Skill,
            "SELECT * FROM skills ORDER BY category, name"
        )
        .fetch_all(&self.pool)
        .await
    }
}
