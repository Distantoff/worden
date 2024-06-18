// @generated automatically by Diesel CLI.

diesel::table! {
    check_reg (id) {
        id -> Integer,
        #[max_length = 255]
        ip -> Varchar,
        date -> Datetime,
        col -> Integer,
    }
}

diesel::table! {
    examples_en (id) {
        id -> Integer,
        word_en_id -> Integer,
        word_type_id -> Integer,
        #[max_length = 255]
        en_example -> Varchar,
        #[max_length = 255]
        ru_example -> Varchar,
    }
}

diesel::table! {
    social_auth (social_id) {
        social_id -> Integer,
        #[max_length = 255]
        social_code -> Varchar,
        #[max_length = 255]
        social_name -> Varchar,
        #[max_length = 255]
        social_img -> Varchar,
        #[max_length = 255]
        social_desc -> Varchar,
        #[max_length = 255]
        social_client_id -> Varchar,
        #[max_length = 255]
        social_client_public_key -> Varchar,
        #[max_length = 255]
        social_secret_code -> Varchar,
        #[max_length = 255]
        social_url -> Varchar,
        #[max_length = 255]
        social_token_url -> Varchar,
        social_enabled -> Bool,
        social_sort -> Integer,
    }
}

diesel::table! {
    user_words (id) {
        id -> Integer,
        user_id -> Integer,
        words_en_id -> Integer,
        hidden -> Bool,
        learned -> Nullable<Bool>,
        metric -> Integer,
        create_date -> Datetime,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        login -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        oauth -> Longtext,
        phone -> Nullable<Integer>,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        is_enable -> Bool,
        create_date -> Datetime,
        update_date -> Nullable<Datetime>,
    }
}

diesel::table! {
    users2 (id) {
        id -> Integer,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        login -> Nullable<Varchar>,
        phone -> Nullable<Integer>,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        is_enable -> Bool,
        create_date -> Datetime,
    }
}

diesel::table! {
    users_settings (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 255]
        code -> Varchar,
        value -> Text,
        sort -> Integer,
    }
}

diesel::table! {
    word_types (id) {
        id -> Integer,
        #[max_length = 255]
        code -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 10]
        short_name -> Nullable<Varchar>,
        create_date -> Date,
    }
}

diesel::table! {
    words_en (id) {
        id -> Integer,
        #[max_length = 255]
        value -> Varchar,
        #[max_length = 20]
        word_type_id -> Nullable<Varchar>,
        #[max_length = 255]
        transcription -> Nullable<Varchar>,
        #[max_length = 255]
        syn_ids -> Nullable<Varchar>,
        create_user_id -> Nullable<Integer>,
        create_date -> Date,
    }
}

diesel::table! {
    words_matching_en_ru (id) {
        id -> Integer,
        en_word_id -> Integer,
        ru_word_id -> Integer,
        synonym_id -> Integer,
        metric -> Integer,
    }
}

diesel::table! {
    words_ru (id) {
        id -> Integer,
        #[max_length = 255]
        value -> Varchar,
        word_type_id -> Nullable<Integer>,
        #[max_length = 255]
        transcription -> Nullable<Varchar>,
        #[max_length = 255]
        syn_ids -> Nullable<Varchar>,
        create_user_id -> Nullable<Integer>,
        create_date -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    check_reg,
    examples_en,
    social_auth,
    user_words,
    users,
    users2,
    users_settings,
    word_types,
    words_en,
    words_matching_en_ru,
    words_ru,
);
