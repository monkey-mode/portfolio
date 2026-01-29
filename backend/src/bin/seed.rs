use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
struct Profile {
    id: Uuid,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Cleaning up existing data...");
    sqlx::query!("DELETE FROM profile").execute(&pool).await?;
    sqlx::query!("DELETE FROM projects").execute(&pool).await?;
    sqlx::query!("DELETE FROM skills").execute(&pool).await?;

    println!("Seeding profile...");
    sqlx::query!(
        r#"
        INSERT INTO profile (name, title, summary, email, github_url, linkedin_url)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        "Suphachai Pechtumrong",
        "Senior Associate Backend Developer",
        "Senior Associate Backend Developer specializing in Golang and distributed systems. Experienced in building high-scale financial platforms and real-time data pipelines. Certified GCP Professional DevOps Engineer and AWS Solutions Architect with a focus on observability, performance optimization, and scalable cloud architecture.",
        "suphachaiphetthamrong@gmail.com",
        "https://github.com/monkey-mode",
        "http://www.linkedin.com/in/suphachai-pechtumrong"
    )
    .execute(&pool)
    .await?;

    println!("Seeding projects...");
    
    // Project 1: Crypto Utils
    sqlx::query!(
        r#"
        INSERT INTO projects (title, description, technologies, repo_url, demo_url, image_url)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        "Crypto Utils",
        "Internal QA web utility using Next.js and Rust (WASM) to provide hashing, encryption, and decryption features, reducing backend support workload.",
        &["Next.js", "Rust", "WASM", "Cryptography"] as &[&str],
        "https://github.com/monkey-mode/crypto-utils",
        "https://crypto-utils.ytwok.xyz/",
        "https://placehold.co/600x400/png?text=Crypto+Utils"
    )
    .execute(&pool)
    .await?;

    // Project 2: Mutual Fund Service (T0)
    sqlx::query!(
        r#"
        INSERT INTO projects (title, description, technologies, repo_url, image_url)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        "Real-time Mutual Fund Service",
        "Contributed to T0 (real-time settlement) mutual fund service, processing ฿12 billion (THB) in transaction value within the first 3 months.",
        &["Go", "Kafka", "GCP", "Microservices"] as &[&str],
        None::<String>,
        "https://placehold.co/600x400/png?text=FinTech+Platform"
    )
    .execute(&pool)
    .await?;

    // Project 3: MFOA Platform
    sqlx::query!(
        r#"
        INSERT INTO projects (title, description, technologies, repo_url, image_url)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        "MFOA Platform Service",
        "Developed event-driven microservices using Golang and Kafka. Improved batch processing performance by 30–60% through optimization.",
        &["Golang", "Gin", "Kafka", "Redis", "k6"] as &[&str],
        None::<String>,
        "https://placehold.co/600x400/png?text=MFOA+Platform"
    )
    .execute(&pool)
    .await?;

    println!("Seeding skills...");
    let skills = vec![
        ("Backend", "Go (Golang)", 5),
        ("Backend", "Rust", 3),
        ("Backend", "Java", 4),
        ("Backend", "Python", 4),
        ("Backend", "Node.js", 4),
        ("Frontend", "TypeScript", 4),
        ("Frontend", "Next.js", 4),
        ("Frontend", "Flutter", 3),
        ("Database", "PostgreSQL", 5),
        ("Database", "MySQL", 4),
        ("Database", "Redis", 4),
        ("DevOps", "GCP", 5),
        ("DevOps", "AWS", 5),
        ("DevOps", "Kubernetes", 5),
        ("DevOps", "Docker", 5),
        ("DevOps", "Terraform", 4),
        ("Tools", "Kafka", 5),
        ("Tools", "Prometheus/Grafana", 4),
    ];

    for (category, name, proficiency) in skills {
         sqlx::query!(
            r#"
            INSERT INTO skills (category, name, proficiency)
            VALUES ($1, $2, $3)
            "#,
            category,
            name,
            proficiency
        )
        .execute(&pool)
        .await?;
    }

    println!("Seeding completed successfully.");
    Ok(())
}
