use super::schema::languages;
use super::schema::snippets;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
pub struct Language {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
pub struct Snippet {
    pub id: String,
    pub code: String,
    pub language_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputLanguage {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputSnippet {
    pub code: String,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct SnippetView {
    pub id: String,
    pub code: String,
    pub language_id: String,
    pub language: String,
}
