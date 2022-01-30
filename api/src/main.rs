#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};

pub mod db;
pub mod handlers;
pub mod models;
pub mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=debug");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed  to create pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/languages", web::get().to(handlers::get_languages))
            .route("/languages", web::post().to(handlers::add_language))
            // .route("/snippet/{lang}", web::get().to(handlers::get_snippet))
            .route("/snippet/random", web::get().to(handlers::get_snippet))
            .route("/snippet", web::post().to(handlers::add_snippet))
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
