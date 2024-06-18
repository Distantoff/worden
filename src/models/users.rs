use crate::schema::users2::dsl::*;
use crate::helpers::database::*;
use diesel::prelude::*;
use serde::Serialize;

impl Users {
    pub fn get_user_info_by_email(email_addr: &String) -> Option<User> {
        let connection = &mut establish_connection();
        let user_res = users2
            .select(User::as_select())
            .filter(email.eq(email_addr))
            .first(connection);

        match user_res {
            Ok(user) => Some(user),
            Err(_) => None
        }
    }

    pub fn user_word_exists(user_id: i32, enword_id: i32) -> QueryResult<bool> {
        use crate::schema::user_words;
        let connection = &mut establish_connection();
        let user_res: Result<i32, _> = user_words::table
            .select(user_words::id)
            .filter(user_words::words_en_id.eq(enword_id))
            .filter(user_words::user_id.eq(user_id))
            .first(connection);

        match user_res {
            Ok(_) => Ok(true),
            Err(err) => Err(err)
        }
    }
}

pub struct Users {}

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::users2)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub email: String
}

#[cfg(test)]
mod test {
    #[test]
    fn get_user_info_by_email() {
        let email = "deadkeny@yandex.ru".to_string();
        let user = super::Users::get_user_info_by_email(&email);
        assert_eq!(email, user.as_ref().unwrap().email);
        assert!(user.unwrap().id > 0);
    }
}
