use jsonwebtoken::{
    Algorithm,
    EncodingKey,
    DecodingKey,
    Header,
    Validation,
    encode,
    decode,
};
use serde::{Serialize, Deserialize};
use crate::helpers::config;
use crate::models::users::User;

pub fn get_information_from_token(token: &String) -> Option<User> {
    let secret = config::get("JWT_SECRET");
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(&token,
        &DecodingKey::from_secret(secret.as_bytes()), &validation);

    match token_data {
        Ok(td) => Some(User {
            id: td.claims.sub.parse::<i32>().unwrap(),
            email: td.claims.email,
        }),
        Err(_) => None
    }
}

pub fn generate_token(uid: i32, email: &String) -> String {
    let secret = config::get("JWT_SECRET");
    let claims = Claims {
        sub: uid.to_string(),
        email: email.to_owned(),
        exp: 10000000000
    };
    let token = encode(&Header::default(), &claims,
        &EncodingKey::from_secret(secret.as_bytes())).unwrap();

    token
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    exp: u64,
}

#[cfg(test)]
mod test {
    #[test]
    fn encrypt_and_decrypt_data() {
        let uid = 11;
        let email = "deadkeny@yandex.ru".to_string();
        let token = super::generate_token(uid, &email);
        let user = super::get_information_from_token(&token).unwrap();

        assert_eq!(uid, user.id);
        assert_eq!(email, user.email);
    }
}
