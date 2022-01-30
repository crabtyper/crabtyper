use super::schema::languages;
use super::schema::snippets;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, Insertable, Debug)]
#[table_name = "languages"]
pub struct Language {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Insertable, Debug)]
#[table_name = "snippets"]
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

#[derive(Deserialize, Serialize, Queryable, Debug)]
pub struct SnippetView {
    pub id: String,
    pub code: String,
    pub language_id: String,
    pub language: String,
}
