use diesel::prelude::*;
use crate::helpers::database::*;

impl WordTypes {
    pub async fn new() -> Self {
        Self {
            list: Self::all().await,
        }
    }

    pub async fn all() -> Vec<WordType> {
        use crate::schema::word_types::dsl::*;
        let connection = &mut establish_connection();
        let types = word_types
            .select(WordType::as_select())
            .load(connection)
            .expect("Error to load ruword list");

        types
    }

    pub async fn get_id_by_val(val: &String) -> Option<i32> {
        let types = Self::all().await;
        let type_id = types.iter()
            .filter(|t| t.code == *val)
            .map(|t| t.id).collect::<Vec<i32>>()
            .first().copied();
        type_id
    }
}

pub struct WordTypes {
    list: Vec<WordType>
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::word_types)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct WordType {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub short_name: Option<String>,
}
