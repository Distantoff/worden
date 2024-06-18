use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use reqwest;
use crate::helpers::config;

pub async fn get_word(word: &str) -> Option<String> {
    match get_from_file(&word).await {
        Some(file_content) => Some(file_content),
        None => fetch_and_save_word_or_add_error_log(word).await
            .expect(format!("error to fetch word {}", word).as_str())
    }
}

async fn get_from_file(word: &str) -> Option<String> {
    let path = get_word_file_path(word);

    if let Ok(mut file) = File::open(path) {
        let mut file_str = String::new();
        File::read_to_string(&mut file, &mut file_str)
            .expect("error reading the file");

        return Some(file_str);
    }
    None
}

async fn fetch_and_save_word_or_add_error_log(word: &str)
    -> Result<Option<String>, reqwest::Error> {

    match fetch_word(word).await? {
        response if response.contains(word) => {
            save_to_file(word, &response)
                .expect("error when trying to write data to a file");
            return Ok(Some(response));
        },
        _ => {
            add_error_to_fetch_log(word).unwrap();
            Ok(None)
        },
    }
}

fn save_to_file(word: &str, data: &String) -> io::Result<()> {
    let path = get_word_file_path(word);
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())
}

async fn fetch_word(word: &str) -> Result<String, reqwest::Error> {
    let url = build_url(word);
    match reqwest::get(&url).await {
        Ok(response) => {
            let text = response.text().await.unwrap();
            if text.contains(word) {
                return Ok(text);
            }
            Ok(String::new())
        },
        Err(err) => Err(err)
    }
}

fn add_error_to_fetch_log(word: &str) -> io::Result<()> {
    if word_exists_in_error_log(word).unwrap() == true {
        return Ok(());
    }

    let log_name = config::get("ERROR_LOAD_LOG");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(log_name)?;

    file.write_fmt(format_args!("{}\n", word))
}

fn word_exists_in_error_log(word: &str) -> io::Result<bool> {
    let log_name = config::get("ERROR_LOAD_LOG");
    let mut file_content = String::new();
    let _ = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(log_name)?
        .read_to_string(&mut file_content);

    let result = file_content.lines().find(|&w| w == word).is_some();
    return Ok(result);
}

fn build_url(word: &str) -> String {
    format!("{url}?key={key}&lang={lang}&text={word}",
        url = config::get("TRANSLATE_URL"),
        key = config::get("TRANSLATE_KEY"),
        lang = config::get("TRANSLATE_LANG"),
    )
}

fn get_word_file_path(word: &str) -> String {
    let cache_dir = config::get("CACHE_DIR");
    let path = cache_dir + word;
    path.to_lowercase()
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn fetch_word() {
        let words_len = super::fetch_word("halt").await;
        assert!(words_len.unwrap().contains("halt"));
    }
}

