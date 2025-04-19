// backend/src/main.rs
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use common::{LeaderboardEntry, LeaderboardRequest, SubmitScoreRequest};
use sqlx::{PgPool, postgres::PgPoolOptions};
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

    // Fix any existing NULL values in the table
    sqlx::query(
        r#"
        UPDATE leaderboard
        SET
            name         = COALESCE(name,         'Anonymous'),
            course       = COALESCE(course,       'default'),
            completed_at = COALESCE(completed_at, NOW())
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
    let limit = req.limit.unwrap_or(10);

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
        limit as i64
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
        score.name.clone()
    };
    let course = if score.course.is_empty() {
        "default".to_string()
    } else {
        score.course.clone()
    };

    let result = sqlx::query!(
        r#"
        INSERT INTO leaderboard (name, course, time_seconds)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        name,
        course,
        score.time_seconds
    )
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Created().json(record.id),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json("Failed to save score")
        }
    }
}

// For server-side rendering, add an endpoint to get the HTML with leaderboard data
async fn render_leaderboard(
    db_pool: web::Data<PgPool>,
    req: web::Query<LeaderboardRequest>,
) -> impl Responder {
    let limit = req.limit.unwrap_or(10);

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
        limit as i64
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(entries) => {
            // Generate HTML table for the leaderboard
            let mut html = String::from(
                r#"<!DOCTYPE html>
                <html>
                <head>
                    <title>Math Quiz Leaderboard</title>
                    <style>
                        body { font-family: Arial, sans-serif; margin: 0; padding: 20px; }
                        table { width: 100%; border-collapse: collapse; }
                        th, td { padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }
                        th { background-color: #f2f2f2; }
                        .container { max-width: 800px; margin: 0 auto; }
                        h1 { color: #333; }
                    </style>
                </head>
                <body>
                    <div class="container">
                        <h1>Math Quiz Leaderboard</h1>
                        <h2>"#,
            );

            html.push_str(&req.course);
            html.push_str(
                r#"</h2>
                        <table>
                            <thead>
                                <tr>
                                    <th>Rank</th>
                                    <th>Name</th>
                                    <th>Time (seconds)</th>
                                    <th>Date</th>
                                </tr>
                            </thead>
                            <tbody>"#,
            );

            for (index, entry) in entries.iter().enumerate() {
                let date_display = entry
                    .completed_at
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| String::from("N/A"));

                html.push_str(&format!(
                    r#"
                    <tr>
                        <td>{}</td>
                        <td>{}</td>
                        <td>{:.2}</td>
                        <td>{}</td>
                    </tr>
                    "#,
                    index + 1,
                    entry.name,
                    entry.time_seconds,
                    date_display
                ));
            }

            html.push_str(
                r#"
                            </tbody>
                        </table>
                    </div>
                </body>
                </html>"#,
            );

            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch leaderboard")
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
            // Server-side rendered HTML endpoint
            .route("/leaderboard", web::get().to(render_leaderboard))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
