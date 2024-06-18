use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::helpers::words::words_helper::WordsHelper;
use crate::models::word_types::*;
use crate::models::words_en::*;
use crate::models::words_ru::*;
use crate::models::word_matches::*;

#[allow(async_fn_in_trait)]
pub trait WordModification {
    async fn add_word(word: String) -> QueryResult<i32>  {
        let json_word: JsonWord = serde_json::from_str(word.as_str()).unwrap();
        let new_enword_id = add_enword(&json_word).await?;

        for enword in &json_word.def {
            let json_ruwords: &Vec<JsonRuWord> = enword.tr.as_ref();
            for ruword in json_ruwords {
                let new_ruword_res = add_ruword(ruword).await?;
                add_match(new_enword_id, new_ruword_res).await?;
            }
        }

        Ok(new_enword_id)
    }
}

impl WordModification for WordsHelper { }

async fn add_enword(word: &JsonWord) -> QueryResult<i32> {
    let enword = &word.def[0];
    let word_val = enword.text.to_owned();
    if let Some(enword) = EnWords::first(&word_val).await {
        return Ok(enword.id.unwrap());
    }

    let word_types = get_enword_types_as_string(&word);
    let new_word_id = EnWords::last_id().await + 1;
    let new_word = EnWord {
        id: Some(new_word_id),
        value: word_val,
        word_type_id: Some(word_types.await),
        transcription: enword.ts.to_owned()
    };

    match EnWords::add(new_word).await {
        Ok(_) => Ok(new_word_id),
        Err(err) => Err(err)
    }
}

async fn add_ruword(word: &JsonRuWord) -> QueryResult<i32> {
    let word_val = word.text.to_owned();
    if let Some(ruword) = RuWords::first(&word_val).await {
        return Ok(ruword.id);
    }

    let new_word_id = RuWords::last_id().await + 1;
    let type_id = WordTypes::get_id_by_val(word.pos.as_ref().unwrap()).await;
    let new_word = RuWord {
        id: new_word_id,
        value: word_val,
        word_type_id: type_id
    };

    match RuWords::add(new_word).await {
        Ok(_) => Ok(new_word_id),
        Err(err) => Err(err)
    }
}

async fn add_match(enword_id: i32, ruword_id: i32) -> QueryResult<bool> {
    let matching = WordMatches::get_by_words_id(enword_id, ruword_id).await;

    if matching.is_ok() && matching.unwrap().len() > 0 {
        return Ok(true);
    }

    let new_match = WordMatch {
        id: None,
        en_word_id: enword_id,
        ru_word_id: ruword_id,
        synonym_id: 0
    };

    match WordMatches::add(new_match).await {
        Ok(_) => Ok(true),
        Err(err) => Err(err)
    }
}

async fn get_enword_types_as_string(word: &JsonWord) -> String {
    let types = WordTypes::all().await;
    word.def.iter().map(|w| {
        let type_code = w.pos.as_ref().unwrap();
        match types.iter().find(|t| &t.code == type_code) {
            Some(t) => t.id.to_string() + ",",
            None => "".to_string()
        }
    }).collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonWord {
    def: Vec<JsonEnWord>
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonEnWord {
    text: String,
    pos: Option<String>,
    ts: Option<String>,
    tr: Vec<JsonRuWord>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRuWord {
    text: String,
    pos: Option<String>,
    gen: Option<String>,
    ex: Option<Vec<JsonExample>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonExample {
    text: String,
    tr: Vec<JsonExampleTranslate>
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonExampleTranslate {
    text: String
}
