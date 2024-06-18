use diesel::prelude::*;
use serde::Serialize;
use crate::helpers::database::*;

impl RuWords {
    pub async fn get_by_id_list(ids: Vec<i32>) -> Vec<RuWord> {
        use crate::schema::words_ru::dsl::*;

        let connection = &mut establish_connection();
        let ruwords = words_ru
            .select(RuWord::as_select())
            .filter(id.eq_any(ids))
            .order(id.asc())
            .load(connection)
            .expect("Error to load ruword list");

        ruwords
    }

    pub async fn first(word: &str) -> Option<RuWord> {
        use crate::schema::words_ru::dsl::*;
        let connection = &mut establish_connection();
        let ruword = words_ru
            .select(RuWord::as_select())
            .filter(value.eq(word))
            .first(connection);

        match ruword {
            Ok(ruword) => Some(ruword),
            Err(_) => None
        }
    }

    pub async fn last_id() -> i32 {
        use crate::schema::words_ru::dsl::*;
        let connection = &mut establish_connection();
        words_ru
            .select(id)
            .limit(1)
            .order(id.desc())
            .first(connection)
            .expect("Error to get last ruword id")
    }
    
    pub async fn add(word: RuWord) -> QueryResult<usize> {
        use crate::schema::words_ru::dsl::*;

        let connection = &mut establish_connection();
        diesel::insert_into(words_ru)
            .values(word)
            .execute(connection)
    }
}

pub struct RuWords {}

#[derive(Queryable, Selectable, Debug, Clone, Serialize, Insertable)]
#[diesel(table_name = crate::schema::words_ru)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RuWord {
    pub id: i32,
    pub value: String,
    pub word_type_id: Option<i32>,
}
