#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, http::header, middleware};
use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};
use handlers::{
    get_languages, 
    add_language,
    get_snippets, 
    get_random_snippet, 
    get_random_snippet_by_lang, 
    add_snippet, 
    delete_snippet, 
};

pub mod db;
pub mod handlers;
pub mod models;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let server_port = if let Ok(port) = std::env::var("PORT") {
        port.parse::<u16>().expect("Could not convert PORT env to string!")
    } else {
        5000
    };

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");


    log::info!("{}", format!("starting HTTP server at http://0.0.0.0:{server_port}"));
    

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                .allowed_origin("https://crabtyper.com")
                .allowed_origin("https://www.crabtyper.com")
                .allowed_origin("https://wonderful-desert-06f1b7f03-develop.westeurope.1.azurestaticapps.net")
                .allowed_origin("localhost")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                .service(
                        web::scope("/languages")
                        // .service(get_languages)
                        // .service(add_language)
                    )
                .service(
                        web::scope("/snippets")
                        // .service(get_snippets)
                        .service(get_random_snippet)
                        // .service(get_random_snippet_by_lang)
                        // .service(add_snippet)
                        // .service(delete_snippet)
                    )
            )
        })
    .bind(("0.0.0.0", server_port))?
    .run()
    .await
}
