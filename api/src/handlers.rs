use super::Pool;
use crate::models::InputLanguage;
use crate::{db, models::InputSnippet};
use actix_web::{web, Error, HttpResponse};

pub async fn get_languages(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::get_all_languages(db))
        .await
        .map(|lang| HttpResponse::Ok().json(lang))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn add_language(
    db: web::Data<Pool>,
    item: web::Json<InputLanguage>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::add_single_language(db, item))
        .await
        .map(|lang| HttpResponse::Created().json(lang))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_snippets(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::get_all_snippets(db))
        .await
        .map(|snippets| HttpResponse::Ok().json(snippets))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_random_snippet(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::get_single_random_snippet(db))
        .await
        .map(|snippet| HttpResponse::Ok().json(snippet))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_random_snippet_by_lang(
    db: web::Data<Pool>,
    language: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let lang = language.into_inner();
    Ok(
        web::block(move || db::get_single_random_snippet_by_lang(db, lang))
            .await
            .map(|snippet| HttpResponse::Ok().json(snippet))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

pub async fn add_snippet(
    db: web::Data<Pool>,
    item: web::Json<InputSnippet>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || db::add_single_snippet(db, item))
        .await
        .map(|snippet| HttpResponse::Ok().json(snippet))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
