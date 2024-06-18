use dotenvy::{dotenv, from_filename};
use std::env;
use std::fmt::Display;
use std::ffi::OsStr;

pub fn get<T>(param: T) -> String
    where T: AsRef<str> + AsRef<OsStr> + Display {
    if is_debug_build() {
        dotenv().ok();
    } else {
        dotenv().ok();
        // from_filename(".env-release").ok();
    }

    env::var(&param).expect(format!("{} must be set", param).as_str())
}

pub fn is_debug_build() -> bool {
    cfg!(debug_assertions)
}
