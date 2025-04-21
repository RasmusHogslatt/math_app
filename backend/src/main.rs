// backend/src/main.rs
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use common::{
    LeaderboardEntry, LeaderboardRequest, SubmitScoreRequest, config::MAX_ENTRIES_PER_COURSE,
};
use sqlx::{PgPool, postgres::PgPoolOptions, types::chrono::Utc};
use std::env;

// Database connection setup
async fn setup_database() -> PgPool {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at {}", database_url);

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations to ensure tables exist and fix any existing null values
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS leaderboard (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            course VARCHAR(50) NOT NULL,
            time_seconds FLOAT NOT NULL,
            completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create leaderboard table");

    // Create index for faster lookups per course
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_leaderboard_course_time ON leaderboard (course, time_seconds ASC, completed_At DESC);
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create index on course");

    // Fix any existing NULL values in the table
    sqlx::query(
        r#"
        UPDATE leaderboard
        SET
            name         = COALESCE(name,         'Anonymous'),
            course       = COALESCE(course,       'default'),
            completed_at = COALESCE(completed_at, NOW())
            WHERE name IS NULL OR course IS NULL OR completed_at IS NULL
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to update leaderboard defaults");

    pool
}

// API handlers
async fn get_leaderboard(
    db_pool: web::Data<PgPool>,
    req: web::Query<LeaderboardRequest>,
) -> impl Responder {
    let result = sqlx::query_as!(
        LeaderboardEntry,
        r#"
        SELECT id, name, course, time_seconds, completed_at
        FROM leaderboard
        WHERE course = $1
        ORDER BY time_seconds ASC
        LIMIT $2
        "#,
        req.course,
        MAX_ENTRIES_PER_COURSE as i64
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to fetch leaderboard")
        }
    }
}

async fn submit_score(
    db_pool: web::Data<PgPool>,
    score: web::Json<SubmitScoreRequest>,
) -> impl Responder {
    // Ensure we don't insert empty values
    let name = if score.name.is_empty() {
        "Anonymous".to_string()
    } else {
        score.name.trim().to_string()
    };
    let course = if score.course.is_empty() {
        "default".to_string()
    } else {
        score.course.trim().to_lowercase().to_string()
    };

    // Start transaction to ensure atomicity
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Insert new score
    let insert_result = sqlx::query!(
        r#"
        INSERT INTO leaderboard (name, course, time_seconds, completed_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
        name,
        course,
        score.time_seconds,
        Utc::now()
    )
    .fetch_one(&mut *tx)
    .await;

    let new_id = match insert_result {
        Ok(record) => record.id,
        Err(e) => {
            eprintln!("Database error inserting score: {}", e);
            // Rollback is implicit when tx goes out of scope without commit
            return HttpResponse::InternalServerError().json("Failed to save score");
        }
    };

    let delete_result = sqlx::query!(
        r#"
        DELETE FROM leaderboard
        WHERE course = $1 AND id NOT IN (
            SELECT id
            FROM leaderboard
            WHERE course = $1
            ORDER BY time_seconds ASC, completed_at ASC -- Tie-breaker is crucial!
            LIMIT $2
        )
        "#,
        course,
        MAX_ENTRIES_PER_COURSE
    )
    .execute(&mut *tx) // Use the transaction
    .await;

    if let Err(e) = delete_result {
        eprintln!("Database error cleaning up leaderboard: {}", e);
        // Rollback is implicit when tx goes out of scope without commit
        return HttpResponse::InternalServerError().json("Failed to update leaderboard ranking");
    }

    // Commit the transaction
    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({ "id": new_id })),
        Err(e) => {
            eprintln!("Database error committing transaction: {}", e);
            HttpResponse::InternalServerError().json("Failed to finalize score submission")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database
    let db_pool = setup_database().await;
    let db_pool = web::Data::new(db_pool);

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        // Configure CORS to allow frontend requests
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(db_pool.clone())
            // API endpoints
            .service(
                web::scope("/api")
                    .route("/leaderboard", web::get().to(get_leaderboard))
                    .route("/submit", web::post().to(submit_score)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
