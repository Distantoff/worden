use diesel::prelude::*;
use crate::helpers::database::*;
// use crate::helpers::words_builder::WordsBuilder;
use crate::schema::users2::dsl::*;
use crate::models::{
    users::{User, Users},
    user_words::UserWord
};

pub trait UserModification {
    fn create_user_by_email(email_addr: &String) -> QueryResult<User> {
        use diesel::result::Error;

        if email_addr.contains("@") == false {
            println!("Incorrect email address");
            return Err(Error::NotFound);
        }

        let connection = &mut establish_connection();
        let new_user_res = diesel::insert_into(users2)
            .values(email.eq(email_addr))
            .execute(connection);

        match new_user_res {
            Ok(_) => Ok(Users::get_user_info_by_email(email_addr).unwrap()),
            Err(err) => Err(err),
        }
    }

    fn add_word(user_id: i32, enword_id: i32) -> QueryResult<usize> {
        use crate::schema::user_words;
        if Users::user_word_exists(user_id, enword_id).is_ok() {
            return Self::show_word(user_id, enword_id);
        }

        let new_user_word = UserWord {
            id: None,
            user_id,
            words_en_id: enword_id,
            hidden: false,
        };

        let connection = &mut establish_connection();
        diesel::insert_into(user_words::table)
            .values(new_user_word)
            .execute(connection)
    }

    fn hide_word(user_id: i32, enword_id: i32) -> QueryResult<usize> {
        use crate::schema::user_words;
        if Users::user_word_exists(user_id, enword_id).is_err() {
            return Ok(1);
        }

        let connection = &mut establish_connection();
        diesel::update(user_words::table
            .filter(user_words::user_id.eq(user_id))
            .filter(user_words::words_en_id.eq(enword_id)))
            .set(user_words::hidden.eq(true))
            .execute(connection)
    }

    fn show_word(user_id: i32, enword_id: i32) -> QueryResult<usize> {
        use crate::schema::user_words;
        if Users::user_word_exists(user_id, enword_id).is_err() {
            return Ok(1);
        }

        let connection = &mut establish_connection();
        diesel::update(user_words::table
                .filter(user_words::user_id.eq(user_id))
                .filter(user_words::words_en_id.eq(enword_id)))
            .set(user_words::hidden.eq(false))
            .execute(connection)
    }
}

impl UserModification for Users {}
