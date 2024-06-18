use serde::Serialize;
use crate::helpers::config;
use crate::helpers::modification::words::WordModification;
use crate::models::word_types::*;
use crate::models::word_matches::*;
use crate::models::words_en::*;
use crate::models::words_ru::*;

impl WordsHelper {
    pub async fn new() -> Self {
        Self {
            types: WordTypes::all().await
        }
    }

    pub async fn get_words_by_enword_list(&self, mut enwords: Vec<EnWord>)
        -> Vec<WordResult> {

        let matches = Self::get_matches_by_enwords(&enwords).await;
        let ruwords = Self::get_ruwords_by_matches(&matches).await;
        enwords.reverse();
        let mut words_result: Vec<WordResult> = Vec::with_capacity(enwords.len());

        while let Some(enword) = enwords.pop() {
            let word_matches_id =
                Self::get_word_matches_id(&matches, enword.id.unwrap());

            let filtered_ruwords: Vec<&RuWord> = ruwords.iter()
                .filter(|w| word_matches_id.contains(&w.id))
                .collect();

            let ru_words_by_type = self.get_ruwords_result_by_type(filtered_ruwords);

            let mut word_res = WordResult {
                id: enword.id.unwrap(),
                value: enword.value,
                transcription: enword.transcription.unwrap(),
                values_by_type: ru_words_by_type,
                is_user_word: false,
                is_translate_file_exists: false
            };

            word_res.is_translate_file_exists = Self::is_audio_files_exists(&word_res);
            words_result.push(word_res);
        }

        words_result
    }

    async fn get_matches_by_enwords(enwords: &Vec<EnWord>) -> Vec<WordMatch> {
        let enword_id_list: Vec<i32> = enwords.iter().map(|w| w.id.unwrap()).collect();
        WordMatches::get_matches_by_enword_id_list(enword_id_list).await
    }

    async fn get_ruwords_by_matches(matches: &Vec<WordMatch>) -> Vec<RuWord> {
        let ruwords_ids: Vec<_> = matches.iter().map(|m| m.ru_word_id).collect();
        let ruwords = RuWords::get_by_id_list(ruwords_ids).await;
        Self::get_sorted_ruwords_by_matches(ruwords, &matches)
    }

    fn get_word_matches_id(matches: &Vec<WordMatch>, enword_id: i32) -> Vec<i32> {
        matches.iter().filter_map(|m| if m.en_word_id == enword_id {
            Some(m.ru_word_id) } else { None }).collect()
    }

    fn get_ruwords_result_by_type(&self, filtered_ruwords: Vec<&RuWord>) -> Vec<WordTypeResult> {

        // Эти действия нужны, для сохранения порядка элементов
        // Что бы типы были в том же порядке, в каком идут слова на русском
        let mut word_type_id_list: Vec<i32> =
            filtered_ruwords.iter().filter_map(|w| w.word_type_id).collect();
        word_type_id_list.dedup();
        let mut words_type_res: Vec<WordTypeResult> = Vec::with_capacity(word_type_id_list.len());

        for word_type_id in &word_type_id_list {
            let ruwords_by_type: Vec<RuWord> = filtered_ruwords.iter()
                .filter(|w| w.word_type_id.is_some()
                    && &w.word_type_id.unwrap() == word_type_id)
                .cloned().cloned().collect();

            if ruwords_by_type.len() <= 0 {
                continue;
            }

            let word_type = self.types.iter()
                .filter(|t| &t.id == word_type_id)
                .collect::<Vec<&WordType>>()
                .first().copied().unwrap();

            words_type_res.push(
                WordTypeResult {
                    id: word_type.id,
                    code: word_type.code.to_owned(),
                    name: word_type.name.to_owned(),
                    short_name: word_type.short_name.to_owned(),
                    words: ruwords_by_type
                }
            );
        }
        words_type_res

    }

    fn get_sorted_ruwords_by_matches(mut ruwords: Vec<RuWord>, matches: &Vec<WordMatch>) -> Vec<RuWord> {
        let mut sorted_ruwords = Vec::with_capacity(ruwords.len());
        for word_match in matches {
            let ru_word_id = word_match.ru_word_id;
            if let Ok(ruword_index) = ruwords.binary_search_by(
                    |rw| rw.id.cmp(&ru_word_id)) {
                sorted_ruwords.push(ruwords.remove(ruword_index));
            }
        }

        sorted_ruwords
    }

    fn is_audio_files_exists(word_res: &WordResult) -> bool {
        use std::path::Path;

        let audio_dir = config::get("AUDIO_DIR");
        let ru_audio_dir = audio_dir.to_owned() + "ru/";
        let en_audio_dir = audio_dir + "en/";
        let en_audio_file = en_audio_dir + word_res.id.to_string().as_str() + ".mp3";

        if Path::new(&en_audio_file).exists() == false {
            return false;
        }

        let ruwords: Vec<&RuWord> = word_res.values_by_type.iter()
            .map(|t| &t.words)
            .flatten().collect();

        let any_audio_file_not_exists = ruwords.iter().find(|w| {
            let en_audio_file = ru_audio_dir.to_owned()
                + w.id.to_string().as_str() + ".mp3";
            Path::new(&en_audio_file).exists() == false
        });

        any_audio_file_not_exists.is_none()
    }

    pub async fn _find_words(&self, name: String) -> Vec<WordResult> {
        use crate::helpers::modification::yandex_puller;

        let mut enwords = EnWords::find_in_any_language(&name).await;
        if enwords.len() <= 0 {
            if let Some(new_word_str) = yandex_puller::get_word(&name).await {
                Self::add_word(new_word_str).await.expect("error to insert new word");
                enwords = EnWords::find_in_any_language(&name).await;
            }
        }

        self.get_words_by_enword_list(enwords).await
    }
}

#[derive(Debug, Clone)]
pub struct WordsHelper {
    types: Vec<WordType>
}

#[derive(Debug, Serialize, Clone)]
pub struct WordResult {
    pub id: i32,
    pub value: String,
    pub transcription: String,
    pub values_by_type: Vec<WordTypeResult>,
    pub is_user_word: bool,
    pub is_translate_file_exists: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct WordTypeResult {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub short_name: Option<String>,
    pub words: Vec<RuWord>,
}
