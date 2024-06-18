use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Serialize, Insertable)]
#[diesel(table_name = crate::schema::user_words)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserWord {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub user_id: i32,
    pub words_en_id: i32,
    pub hidden: bool,
}
