use super::schema::languages::dsl::*;
use super::schema::random;
use super::schema::snippets::dsl::*;
use super::Pool;
use super::{models::Language, schema::languages};
use crate::diesel::QueryDsl;
use crate::models::{InputSnippet, Snippet};
use crate::{
    diesel::RunQueryDsl,
    models::{InputLanguage, SnippetView},
};
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::insert_into;
use diesel::prelude::*;

use std::vec::Vec;

pub async fn get_languages(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_languages(db))
        .await
        .map(|lang| HttpResponse::Ok().json(lang))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn add_language(
    db: web::Data<Pool>,
    item: web::Json<InputLanguage>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_language(db, item))
        .await
        .map(|lang| HttpResponse::Created().json(lang))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_snippet(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_random_snippet(db))
        .await
        .map(|snippet| HttpResponse::Ok().json(snippet))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn add_snippet(
    db: web::Data<Pool>,
    item: web::Json<InputSnippet>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_single_snippet(db, item))
        .await
        .map(|snippet| HttpResponse::Ok().json(snippet))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_languages(pool: web::Data<Pool>) -> Result<Vec<Language>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = languages.load::<Language>(&conn)?;

    Ok(items)
}

fn add_single_language(
    db: web::Data<Pool>,
    item: web::Json<InputLanguage>,
) -> Result<Language, String> {
    let conn = db.get().unwrap();

    let new_lang = Language {
        id: uuid::Uuid::new_v4().to_string(),
        name: item.name.to_string(),
    };

    let result = insert_into(languages).values(&new_lang).execute(&conn);

    match result {
        Ok(_) => Ok(new_lang),
        Err(e) => Err(format!("Can't create a new language: {:?}", e)),
    }
}

fn add_single_snippet(
    db: web::Data<Pool>,
    item: web::Json<InputSnippet>,
) -> Result<SnippetView, diesel::result::Error> {
    let conn = db.get().unwrap();

    let lang = languages
        .filter(languages::name.eq(&item.language))
        .first::<Language>(&conn)?;

    let new_snippet = Snippet {
        id: uuid::Uuid::new_v4().to_string(),
        code: item.code.clone(),
        language_id: lang.id,
    };

    insert_into(snippets).values(&new_snippet).execute(&conn);

    let view = SnippetView {
        id: new_snippet.id,
        code: new_snippet.code,
        language_id: new_snippet.language_id,
        language: lang.name,
    };

    Ok(view)
}

fn get_random_snippet(pool: web::Data<Pool>) -> Result<SnippetView, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let item = snippets
        .inner_join(languages)
        .order_by(random)
        .limit(1)
        .select((languages::id, code, language_id, languages::name))
        .first::<SnippetView>(&conn)?;

    Ok(item)
}
