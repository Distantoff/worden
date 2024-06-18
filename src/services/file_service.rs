use crate::helpers::config;

pub async fn get_mp3(lang: &String, filename: &String) -> Vec<u8> {
    let filepath = generate_filename(&lang, &filename);
    tokio::fs::read(&filepath).await.unwrap()
}

fn generate_filename(lang: &String, filename: &String) -> String {
    let audio_dir = config::get("AUDIO_DIR");
    audio_dir + lang.as_str() + "/" + filename.as_str()
}
