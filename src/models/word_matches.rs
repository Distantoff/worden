use diesel::prelude::*;
use crate::helpers::database::*;
use serde::Serialize;

impl WordMatches {
    pub async fn get_matches_by_enword_id_list(ids: Vec<i32>) -> Vec<WordMatch> {
        use crate::schema::words_matching_en_ru::dsl::*;

        let connection = &mut establish_connection();
        let words_matching = words_matching_en_ru
            .select(WordMatch::as_select())
            .filter(en_word_id.eq_any(ids))
            .filter(synonym_id.eq(0))
            .order(id.asc())
            .load(connection)
            .expect("Error to load words matching list");

       words_matching
    }

    pub async fn get_by_words_id(enword_id: i32, ruword_id: i32)
        -> QueryResult<Vec<WordMatch>> {
        use crate::schema::words_matching_en_ru::dsl::*;
        let connection = &mut establish_connection();

        words_matching_en_ru 
            .select(WordMatch::as_select())
            .filter(en_word_id.eq(enword_id))
            .filter(ru_word_id.eq(ruword_id))
            .load(connection)
    }

    pub async fn add(word_match: WordMatch) -> QueryResult<usize> {
        use crate::schema::words_matching_en_ru::dsl::*;

        let connection = &mut establish_connection();
        diesel::insert_into(words_matching_en_ru)
            .values(word_match)
            .execute(connection)
    }
}

pub struct WordMatches {}

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Insertable)]
#[diesel(table_name = crate::schema::words_matching_en_ru)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct WordMatch {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub en_word_id: i32,
    pub ru_word_id: i32,
    pub synonym_id: i32,
}
