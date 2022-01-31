use super::schema::languages::dsl::*;
use super::schema::snippets::dsl::*;
use super::Pool;
use super::{models::Language, schema::languages};
use crate::diesel::QueryDsl;
use crate::models::{InputSnippet, Snippet};
use crate::schema::random;
use crate::{
    diesel::RunQueryDsl,
    models::{InputLanguage, SnippetView},
};
use actix_web::web;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use std::vec::Vec;

pub fn get_all_languages(pool: web::Data<Pool>) -> Result<Vec<Language>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = languages.load::<Language>(&conn)?;

    Ok(items)
}

pub fn add_single_language(
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

pub fn add_single_snippet(
    db: web::Data<Pool>,
    item: web::Json<InputSnippet>,
) -> Result<SnippetView, String> {
    let conn = db.get().unwrap();

    let lang = languages
        .filter(languages::name.eq(&item.language))
        .first::<Language>(&conn)
        .unwrap();

    let new_snippet = Snippet {
        id: uuid::Uuid::new_v4().to_string(),
        code: item.code.clone(),
        language_id: lang.id,
    };

    let result = insert_into(snippets).values(&new_snippet).execute(&conn);

    match result {
        Ok(_) => Ok(SnippetView {
            id: new_snippet.id,
            code: new_snippet.code,
            language_id: new_snippet.language_id,
            language: lang.name,
        }),
        Err(e) => Err(format!("Can't create a new snippet: {:?}", e)),
    }
}

pub fn get_single_random_snippet(
    pool: web::Data<Pool>,
) -> Result<SnippetView, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let item = snippets
        .inner_join(languages)
        .select((languages::id, code, language_id, languages::name))
        .order_by(random)
        .limit(1)
        .first::<SnippetView>(&conn)?;

    Ok(item)
}

pub fn get_single_random_snippet_by_lang(
    pool: web::Data<Pool>,
    language: String,
) -> Result<SnippetView, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let snippet = snippets
        .inner_join(languages)
        .select((languages::id, code, language_id, languages::name))
        .filter(languages::name.eq(language))
        .order_by(random)
        .limit(1)
        .first::<SnippetView>(&conn)?;

    Ok(snippet)
}

pub fn get_all_snippets(pool: web::Data<Pool>) -> Result<Vec<SnippetView>, diesel::result::Error> {
    let conn = pool.get().unwrap();

    let items = snippets
        .inner_join(languages)
        .select((languages::id, code, language_id, languages::name))
        .load::<SnippetView>(&conn)?;

    Ok(items)
}
