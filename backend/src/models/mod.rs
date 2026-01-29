use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
    pub title: String,
    pub summary: String,
    pub email: Option<String>,
    pub github_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Skill {
    pub id: Uuid,
    pub category: String,
    pub name: String,
    pub proficiency: Option<i32>,
    pub created_at: DateTime<Utc>,
}
