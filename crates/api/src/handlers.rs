use super::DbPool;
use crate::models::InputLanguage;
use crate::{db, models::InputSnippet};
use actix_web::error::ErrorInternalServerError;
use actix_web::{delete, get, post, web, Error, HttpResponse};

#[get("")]
pub async fn get_languages(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        db::get_all_languages(&conn)
    })
    .await?
    .map(|langs| HttpResponse::Ok().json(langs))
    .map_err(ErrorInternalServerError)
}

#[post("")]
pub async fn add_language(
    pool: web::Data<DbPool>,
    item: web::Json<InputLanguage>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        db::add_single_language(&conn, item)
    })
    .await?
    .map(|lang| HttpResponse::Created().json(lang))
    .map_err(ErrorInternalServerError)
}

#[get("")]
pub async fn get_snippets(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        db::get_all_snippets(&conn)
    })
    .await?
    .map(|snippets| HttpResponse::Ok().json(snippets))
    .map_err(ErrorInternalServerError)
}

#[get("/random")]
pub async fn get_random_snippet(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        db::get_single_random_snippet(&conn)
    })
    .await?
    .map(|snippet| HttpResponse::Ok().json(snippet))
    .map_err(ErrorInternalServerError)
}

#[get("/{language}")]
pub async fn get_random_snippet_by_lang(
    pool: web::Data<DbPool>,
    language: web::Path<String>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        db::get_single_random_snippet_by_lang(&conn, language.into_inner())
    })
    .await?
    .map(|snippet| HttpResponse::Ok().json(snippet))
    .map_err(ErrorInternalServerError)
}

#[post("")]
pub async fn add_snippet(
    pool: web::Data<DbPool>,
    item: web::Json<InputSnippet>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        db::add_single_snippet(&conn, item)
    })
    .await?
    .map(|snippet| HttpResponse::Ok().json(snippet))
    .map_err(ErrorInternalServerError)
}

#[delete("/{snippet_id}")]
pub async fn delete_snippet(
    pool: web::Data<DbPool>,
    snippet_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = pool.get()?;
        db::delete_single_snippet(&conn, snippet_id.into_inner())
    })
    .await?
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(ErrorInternalServerError)
}
