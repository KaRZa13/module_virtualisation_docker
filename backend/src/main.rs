use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

#[derive(Serialize, Deserialize)]
struct Counter {
    value: i32,
}

struct AppState {
    db: Pool<Postgres>,
}

#[get("/counter")]
async fn get_counter(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, (i32,)>("SELECT val FROM counters WHERE id = 1")
        .fetch_one(&data.db)
        .await
    {
        Ok(row) => HttpResponse::Ok().json(Counter { value: row.0 }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/increment")]
async fn increment(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, (i32,)>("UPDATE counters SET val = val + 1 WHERE id = 1 RETURNING val")
        .fetch_one(&data.db)
        .await 
    {
        Ok(row) => HttpResponse::Ok().json(Counter { value: row.0 }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/reset")]
async fn reset(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, (i32,)>("UPDATE counters SET val = 0 WHERE id = 1 RETURNING val")
        .fetch_one(&data.db)
        .await 
    {
        Ok(row) => HttpResponse::Ok().json(Counter { value: row.0 }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn init_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS counters (
            id SERIAL PRIMARY KEY,
            val INTEGER NOT NULL DEFAULT 0
        );"
    )
    .execute(pool)
    .await?;

    let count: (i64,) = sqlx::query_as("SELECT count(*) FROM counters")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        sqlx::query("INSERT INTO counters (id, val) VALUES (1, 0)")
            .execute(pool)
            .await?;
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    init_db(&pool).await.expect("Failed to initialize DB");

    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(get_counter)
            .service(increment)
            .service(reset)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
