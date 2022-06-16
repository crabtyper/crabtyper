use actix_web::web;
use diesel::prelude::*;
use std::vec::Vec;
use uuid::Uuid;

use crate::models::{InputLanguage, InputSnippet, Language, Snippet, SnippetView};
use crate::schema::random;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_all_languages(conn: &SqliteConnection) -> Result<Vec<Language>, DbError> {
    use crate::schema::languages::dsl::*;

    Ok(languages.load::<Language>(conn)?)
}

pub fn add_single_language(
    conn: &SqliteConnection,
    item: web::Json<InputLanguage>,
) -> Result<Language, DbError> {
    use crate::schema::languages::dsl::*;

    let new_lang = Language {
        id: Uuid::new_v4().to_string(),
        name: item.name.to_string(),
    };

    diesel::insert_into(languages)
        .values(&new_lang)
        .execute(conn)?;

    Ok(new_lang)
}

pub fn add_single_snippet(
    conn: &SqliteConnection,
    item: web::Json<InputSnippet>,
) -> Result<Snippet, DbError> {
    use crate::schema::languages::dsl::*;
    use crate::schema::snippets::dsl::*;

    let lang = languages
        .filter(name.eq(&item.language))
        .first::<Language>(conn)
        .unwrap();

    let new_snippet = Snippet {
        id: Uuid::new_v4().to_string(),
        code: item.code.to_string(),
        language_id: lang.id,
    };

    diesel::insert_into(snippets)
        .values(&new_snippet)
        .execute(conn)?;

    Ok(new_snippet)
}

pub fn get_single_random_snippet(conn: &SqliteConnection) -> Result<SnippetView, DbError> {
    use crate::schema::languages::dsl::{languages, name as language_name};
    use crate::schema::snippets::dsl::*;

    let item = snippets
        .inner_join(languages)
        .select((id, code, language_id, language_name))
        .order_by(random)
        .limit(1)
        .first::<SnippetView>(conn)?;

    Ok(item)
}

pub fn get_single_random_snippet_by_lang(
    conn: &SqliteConnection,
    language: String,
) -> Result<SnippetView, DbError> {
    use crate::schema::languages::dsl::{languages, name as language_name};
    use crate::schema::snippets::dsl::*;

    let snippet = snippets
        .inner_join(languages)
        .select((id, code, language_id, language_name))
        .filter(language_name.eq(language))
        .order_by(random)
        .limit(1)
        .first::<SnippetView>(conn)?;

    Ok(snippet)
}

pub fn get_all_snippets(conn: &SqliteConnection) -> Result<Vec<SnippetView>, DbError> {
    use crate::schema::languages::dsl::{languages, name as language_name};
    use crate::schema::snippets::dsl::*;

    let items = snippets
        .inner_join(languages)
        .select((id, code, language_id, language_name))
        .load::<SnippetView>(conn)?;

    Ok(items)
}

pub fn delete_single_snippet(
    conn: &SqliteConnection,
    snippet_id: String,
) -> Result<usize, DbError> {
    use crate::schema::snippets::dsl::*;

    let count = diesel::delete(snippets.find(snippet_id)).execute(conn)?;

    Ok(count)
}
