// backend/src/main.rs
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use common::{
    LeaderboardEntry, LeaderboardRequest, SubmitScoreRequest, TopUserSchoolEntry,
    config::MAX_ENTRIES_PER_COURSE,
};
use sqlx::{
    PgPool,
    postgres::PgPoolOptions,
    types::{Uuid, chrono::Utc},
};
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
        .unwrap_or(MAX_ENTRIES_PER_COURSE.try_into().unwrap());

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

    let school = score.school.trim();
    let school = if school.is_empty() {
        "Unknown School"
    } else {
        school
    }
    .to_string();

    // Start transaction
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {}", e);
            return HttpResponse::InternalServerError().json("Failed to start transaction");
        }
    };

    // 1. Fetch the user's current scores for this specific course, ordered worst first
    let user_scores = sqlx::query!(
        r#"
        SELECT id, time_seconds
        FROM leaderboard
        WHERE course = $1
        AND school_id = $2
        ORDER BY time_seconds DESC -- Worst score first
        "#,
        course,          // $1
        score.school_id, // $2
    )
    .fetch_all(&mut *tx) // Use the transaction
    .await;

    let (current_scores_count, worst_score_id, worst_score_time) = match user_scores {
        Ok(scores) => {
            let count = scores.len();
            let worst_id = scores.first().map(|r| r.id);
            let worst_time = scores.first().map(|r| r.time_seconds);
            (count, worst_id, worst_time)
        }
        Err(e) => {
            eprintln!("Database error fetching user scores: {}", e);
            // Rollback happens automatically
            return HttpResponse::InternalServerError().json("Failed to check existing scores");
        }
    };

    let mut delete_worst = false;

    // 2. Decide whether to insert the new score based on user's limit
    if current_scores_count < MAX_ENTRIES_PER_COURSE as usize {
        // User has space, always insert
        println!(
            "User {:?} has {} scores for course '{}' (limit {}). Inserting.",
            score.school_id, current_scores_count, course, MAX_ENTRIES_PER_COURSE
        );
        // No deletion needed yet
    } else {
        // User is at the limit, check if the new score is better than their worst
        match worst_score_time {
            Some(worst_time) if score.time_seconds < worst_time => {
                // New score is better than the worst, allow insertion and mark worst for deletion
                println!(
                    "User {:?} at limit for course '{}'. New score {:.2} is better than worst {:.2}. Replacing.",
                    score.school_id, course, score.time_seconds, worst_time
                );
                delete_worst = true; // Mark the user's own worst score for deletion
            }
            _ => {
                // User is at limit, and the new score is not better than their worst
                println!(
                    "User {:?} at limit for course '{}'. New score {:.2} is not better than worst {:.2}. Rejecting.",
                    score.school_id,
                    course,
                    score.time_seconds,
                    worst_score_time.unwrap_or(f64::INFINITY)
                );
                // Don't insert, return a conflict/rejection message
                // Transaction will roll back automatically as it won't be committed.
                return HttpResponse::Conflict().json(
                    "Score is not fast enough to replace your slowest time for this course.",
                );
            }
        }
    }

    // 3. Insert the new score if allowed
    let insert_result = sqlx::query!(
        r#"
        INSERT INTO leaderboard (name, course, school, school_id, time_seconds, completed_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        name,
        course,
        school,
        score.school_id,
        score.time_seconds,
        Utc::now()
    )
    .fetch_one(&mut *tx) // Use the transaction
    .await;

    let new_id = match insert_result {
        Ok(record) => record.id,
        Err(e) => {
            eprintln!("Database error inserting score: {}", e);
            // Rollback happens automatically
            return HttpResponse::InternalServerError().json("Failed to save score");
        }
    };

    // 4. Delete the *user's own* worst score *if* needed (if delete_worst is true)
    if delete_worst {
        if let Some(id_to_delete) = worst_score_id {
            println!(
                "Deleting user's ({:?}) worst score (ID: {}) for course '{}'",
                score.school_id, id_to_delete, course
            );
            let delete_user_worst_result = sqlx::query!(
                "DELETE FROM leaderboard WHERE id = $1 AND school_id = $2", // Double-check school_id for safety
                id_to_delete,
                score.school_id
            )
            .execute(&mut *tx) // Use the transaction
            .await;

            match delete_user_worst_result {
                Ok(result) if result.rows_affected() == 1 => {
                    println!(
                        "Successfully deleted user's worst score ID: {}",
                        id_to_delete
                    );
                }
                Ok(_) => {
                    // This case (0 rows affected) might happen if something changed between the check and delete, though unlikely within a transaction. Log it.
                    eprintln!(
                        "Warning: Attempted to delete user's worst score ID {} but it was not found or did not match school_id {:?}.",
                        id_to_delete, score.school_id
                    );
                }
                Err(e) => {
                    eprintln!("Database error deleting user's worst score: {}", e);
                    // Rollback happens automatically
                    return HttpResponse::InternalServerError()
                        .json("Failed to replace worst score");
                }
            }
        } else {
            // Should not happen if delete_worst is true, but log defensively
            eprintln!(
                "Warning: delete_worst was true, but no worst_score_id found for user {:?} course {}.",
                score.school_id, course
            );
        }
    }

    // 5. REMOVED - No overall leaderboard cleanup in this operation.
    //    The leaderboard might grow indefinitely for a course if many users submit scores.
    //    Cleanup would need to be handled separately (e.g., a background job or admin task)
    //    if you want to limit the *total* size of the leaderboard per course.

    // 6. Commit the transaction
    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({ "id": new_id })),
        Err(e) => {
            eprintln!("Database error committing transaction: {}", e);
            HttpResponse::InternalServerError().json("Failed to finalize score submission")
        }
    }
}

#[derive(serde::Deserialize, Debug, Default)] // Query parameters for fetching top users
pub struct GetTopUsersBySchoolQuery {
    pub school: String,
    pub school_id: Uuid,
    pub limit: Option<u32>, // Number of top users to return
}

async fn get_top_users_by_school(
    db_pool: web::Data<PgPool>,
    req: web::Query<GetTopUsersBySchoolQuery>,
) -> impl Responder {
    // Default limit to 3, max reasonable limit (e.g., 20) to prevent abuse
    let limit = req.limit.unwrap_or(3).min(20) as i64;

    let query_str = r#"
    WITH UserCourseRanks AS (
        SELECT
            l.name,
            l.school,
            l.school_id,
            l.course,
            l.completed_at,
            l.time_seconds,
            ROW_NUMBER() OVER (PARTITION BY l.course ORDER BY l.time_seconds ASC, l.completed_at ASC) as rank_in_course
        FROM leaderboard l
    )
    SELECT
        ucr.name,
        ucr.school,
        COUNT(DISTINCT ucr.course) AS leaderboard_count,
        COALESCE(SUM(CASE WHEN ucr.rank_in_course = 1 THEN 1 ELSE 0 END), 0) AS gold_medals,
        COALESCE(SUM(CASE WHEN ucr.rank_in_course = 2 THEN 1 ELSE 0 END), 0) AS silver_medals,
        COALESCE(SUM(CASE WHEN ucr.rank_in_course = 3 THEN 1 ELSE 0 END), 0) AS bronze_medals,
        COALESCE(SUM(CASE WHEN ucr.rank_in_course > 3 THEN 1 ELSE 0 END), 0) AS generic_medals
    FROM UserCourseRanks ucr
    WHERE ucr.school_id = $1 AND ucr.school = $2
    GROUP BY ucr.name, ucr.school
    ORDER BY leaderboard_count DESC, MIN(ucr.completed_at) ASC
    LIMIT $3;
"#;

    let result = sqlx::query_as::<_, TopUserSchoolEntry>(query_str)
        .bind(&req.school_id) // $1: school_id (Uuid of the school)
        .bind(&req.school) // $2: school_name (String, name of the school)
        .bind(limit) // $3: limit (i64, number of top users to return)
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Database error fetching top users by school: {}", e);
            HttpResponse::InternalServerError().json("Failed to fetch top users by school")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = setup_database().await;
    let db_pool_data = web::Data::new(db_pool);

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(db_pool_data.clone())
            .service(
                web::scope("/api")
                    .route("/leaderboard", web::get().to(get_leaderboard))
                    .route("/submit", web::post().to(submit_score))
                    .route(
                        "/top_users_by_school",
                        web::get().to(get_top_users_by_school),
                    ),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
