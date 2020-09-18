#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde_json;

use regex::Regex;
use serde_json::Value;
use std::borrow::Borrow;
use std::str;
use wasm_bindgen::prelude::*;

// include the json in the bin
const AFFIN: &[u8; 32811] = include_bytes!("./afinn.json");

lazy_static! {
    static ref AFFIN_VALUE: Value = {
        let json = str::from_utf8(AFFIN).unwrap();
        serde_json::from_str(json).unwrap()
    };
}

/// Struct for return the outcome of individual sentiments
pub struct Sentiment {
    pub score: f32,
    pub comparative: f32,
    pub words: Vec<String>,
}

fn tokenize_with_no_punctuation(phrase: &str) -> Vec<String> {
    let re = Regex::new(r"[^a-zA-Z0 -]+").unwrap();
    let re2 = Regex::new(r" {2,}").unwrap();

    let no_punctuation = re.replace_all(phrase, " ");
    let no_punctuation = re2.replace_all(no_punctuation.borrow(), " ");

    no_punctuation
        .to_lowercase()
        .split(' ')
        .map(|s| s.to_string())
        .collect()
}

/// Calculates the negativity of a sentence
pub fn negativity(phrase: &str) -> Sentiment {
    let tokens = tokenize_with_no_punctuation(phrase);
    let tokens_len = tokens.len() as f32;
    let mut score = 0f32;
    let mut words = Vec::new();

    for t in tokens {
        let word = t.clone();
        if let Value::Number(ref val) = AFFIN_VALUE[t] {
            let diff = val.as_f64().unwrap() as f32;
            if diff < 0f32 {
                score -= diff;
                words.push(word);
            }
        }
    }

    Sentiment {
        score,
        comparative: score / tokens_len,
        words,
    }
}

/// Calculates the positivity of a sentence
pub fn positivity(phrase: &str) -> Sentiment {
    let tokens = tokenize_with_no_punctuation(phrase);
    let tokens_len = tokens.len() as f32;
    let mut score = 0f32;
    let mut words = Vec::new();

    for t in tokens {
        let word = t.clone();
        if let Value::Number(ref val) = AFFIN_VALUE[t] {
            let diff = val.as_f64().unwrap() as f32;
            if diff > 0f32 {
                score += diff;
                words.push(word);
            }
        }
    }

    Sentiment {
        score,
        comparative: score / tokens_len,
        words,
    }
}

#[wasm_bindgen]
pub fn analyze(phrase: &str) ->  String{
    let neg = negativity(phrase);
    let pos = positivity(phrase);
    let score = pos.score - neg.score;
    let comparative = pos.comparative - neg.comparative;
    return score.to_string()+", Comparative:"+&comparative.to_string()
}