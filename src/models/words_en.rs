use diesel::prelude::*;
use regex::Regex;
use serde::Serialize;
use std::cmp::Ordering;
use crate::helpers::{database::*, config};

impl EnWords {
    pub async fn find(word: &str) -> Vec<EnWord> {
        use crate::schema::words_en::dsl::*;

        let limit: i64 = config::get("SEARCH_WORDS_LIMIT")
            .parse::<i64>().unwrap();
        let connection = &mut establish_connection();
        let enwords = words_en
            .select(EnWord::as_select())
            .filter(value.like(word.to_string() + "%"))
            .order(value.asc())
            .limit(limit)
            .load(connection)
            .expect("error to find enwords");

        enwords
    }

    pub async fn first(word: &str) -> Option<EnWord> {
        use crate::schema::words_en::dsl::*;
        let connection = &mut establish_connection();
        let enword = words_en
            .select(EnWord::as_select())
            .filter(value.eq(word))
            .first(connection);

        match enword {
            Ok(enword) => Some(enword),
            Err(_) => None
        }
    }

    pub async fn last_id() -> i32 {
        use crate::schema::words_en::dsl::*;
        let connection = &mut establish_connection();
        words_en
            .select(id)
            .limit(1)
            .order(id.desc())
            .first(connection)
            .expect("Error to get last enword id")
    }

    pub async fn find_by_ruword(word: &str) -> Vec<EnWord> {
        use crate::schema::*;

        let limit: i64 = config::get("SEARCH_WORDS_LIMIT")
            .parse::<i64>().unwrap();
        let connection = &mut establish_connection();
        let enwords = words_en::table
            .select(EnWord::as_select())
            .inner_join(words_matching_en_ru::table
                .on(words_matching_en_ru::en_word_id.eq(words_en::id)))
            .inner_join(words_ru::table
                .on(words_ru::id.eq(words_matching_en_ru::ru_word_id)))
            .filter(words_ru::value.like(word.to_string() + "%"))
            .limit(limit)
            .load(connection)
            .expect("error to find enwords");

        enwords
    }

    pub async fn find_in_any_language(word: &str) -> Vec<EnWord> {
        let cyrillic_pattern = Regex::new(r"\p{IsCyrillic}+").unwrap();

        match cyrillic_pattern.is_match(&word) {
            true => EnWords::find_by_ruword(&word).await,
            false => EnWords::find(&word).await,
        }
    }

    pub async fn add(word: EnWord) -> QueryResult<usize> {
        use crate::schema::words_en::dsl::*;

        let connection = &mut establish_connection();
        diesel::insert_into(words_en)
            .values(word)
            .execute(connection)
    }
}

pub struct EnWords {}

#[derive(Queryable, Selectable, Debug, Serialize, Insertable, Eq)]
#[diesel(table_name = crate::schema::words_en)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct EnWord {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub value: String,
    pub word_type_id: Option<String>,
    pub transcription: Option<String>,
}

impl Ord for EnWord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for EnWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl PartialEq for EnWord {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
