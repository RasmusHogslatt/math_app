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
            school VARCHAR(100) NOT NULL,
            school_id UUID NOT NULL,
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
        CREATE INDEX IF NOT EXISTS idx_leaderboard_course_time_schoolid
        ON leaderboard (course, time_seconds ASC, school_id);
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create index on course, time_seconds, school_id");

    // Fix any existing NULL values in the table
    sqlx::query(
        r#"
        UPDATE leaderboard
        SET
            name         = COALESCE(name,         'Anonymous'),
            course       = COALESCE(course,       'default'),
            school       = COALESCE(school,       'Unknown School'),
            school_id    = COALESCE(school_id,    '00000000-0000-0000-0000-000000000000'::uuid),
            time_seconds = COALESCE(time_seconds, 999999.0),
            completed_at = COALESCE(completed_at, NOW())
        WHERE name IS NULL
           OR course IS NULL
           OR school IS NULL
           OR school_id IS NULL
           OR time_seconds IS NULL
           OR completed_at IS NULL
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to update leaderboard defaults for potentially null columns");

    println!("Database setup complete with index on (course, time_seconds, school_id).");
    pool
}

// API handlers
async fn get_leaderboard(
    db_pool: web::Data<PgPool>,
    req: web::Query<LeaderboardRequest>,
) -> impl Responder {
    let limit = req
        .limit
        .unwrap_or((MAX_ENTRIES_PER_COURSE as i64).try_into().unwrap());

    // Use the macro version for compile-time checks and direct struct mapping
    let result = sqlx::query_as!(
        LeaderboardEntry, // Target struct
        r#"
        SELECT id, name, course, school, school_id, time_seconds, completed_at
        FROM leaderboard
        WHERE course = $1
          AND school = $2    -- Filter by school
          AND school_id = $3 -- Filter by school_id
        ORDER BY time_seconds ASC, completed_at DESC -- Use new index fields
        LIMIT $4             -- Limit parameter is now $4
        "#,
        &req.course,   // $1
        &req.school,   // $2
        req.school_id, // $3 - Uuid doesn't usually need a reference here
        limit as i64   // $4
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(e) => {
            eprintln!("Database error fetching leaderboard: {}", e);
            HttpResponse::InternalServerError().json("Failed to fetch leaderboard")
        }
    }
}
async fn submit_score(
    db_pool: web::Data<PgPool>,
    score: web::Json<SubmitScoreRequest>,
) -> impl Responder {
    // Trim and provide defaults
    let name = score.name.trim();
    let name = if name.is_empty() { "Anonymous" } else { name }.to_string();

    let course = score.course.trim().to_lowercase();
    let course = if course.is_empty() {
        "default"
    } else {
        &course
    }
    .to_string();

    let school = score.school.trim(); // Trim school name
    let school = if school.is_empty() {
        "Unknown School"
    } else {
        school
    }
    .to_string();

    // school_id comes directly from the request (it's a Uuid)

    // Start transaction
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // Check if user (school_id) has already submitted a score for this course
    // We check based on course and school_id, as one user/device should only submit once per course.
    let existing = sqlx::query!(
        r#"
        SELECT id FROM leaderboard
        WHERE course = $1
        AND school_id = $2 -- Check specifically for this user/device ID
        "#,
        course,          // $1
        score.school_id, // $2
    )
    .fetch_optional(&mut *tx) // Use the transaction
    .await;

    match existing {
        Ok(Some(_)) => {
            // User/Device has already submitted a score for this course
            // No need to rollback, just return conflict
            return HttpResponse::Conflict()
                .json("You (or this device) have already submitted a score for this course");
        }
        Ok(None) => {
            // Proceed with inserting the score
        }
        Err(e) => {
            eprintln!("Database error checking existing score: {}", e);
            // Rollback happens automatically when tx drops without commit
            return HttpResponse::InternalServerError().json("Failed to check existing score");
        }
    }

    // Insert new score
    let insert_result = sqlx::query!(
        r#"
        INSERT INTO leaderboard (name, course, school, school_id, time_seconds, completed_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        name,               // $1
        course,             // $2
        school,             // $3 - Use the validated school name
        score.school_id,    // $4 - Use the provided school_id
        score.time_seconds, // $5
        Utc::now()          // $6
    )
    .fetch_one(&mut *tx) // Use the transaction
    .await;

    let new_id = match insert_result {
        Ok(record) => record.id,
        Err(e) => {
            eprintln!("Database error inserting score: {}", e);
            // Rollback happens automatically when tx drops without commit
            return HttpResponse::InternalServerError().json("Failed to save score");
        }
    };

    // Clean up excess entries for the specific course
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
        course,                        // $1
        MAX_ENTRIES_PER_COURSE as i64  // $2 - Ensure type matches expected i64
    )
    .execute(&mut *tx) // Use the transaction
    .await;

    if let Err(e) = delete_result {
        eprintln!("Database error cleaning up leaderboard: {}", e);
        // Rollback happens automatically when tx drops without commit
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
