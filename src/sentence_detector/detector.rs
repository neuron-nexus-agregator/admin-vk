use lazy_static::lazy_static;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashMap;

const MAX_LEN: usize = 200; // максимальная длина предложения

const STOP_WORDS: [&str; 24] = [
    "и", "в", "на", "с", "по", "что", "как", "это", "но", "а", "из", "к", "у", "не", "за", "от",
    "для", "о", "бы", "же", "ли", "то", "до", "при",
];

lazy_static! {
    static ref SENTENCE_RE: Regex = Regex::new(r"[.!?]\s+").unwrap();
    static ref WORD_RE: Regex = Regex::new(r"\b\p{L}+\b").unwrap();
}

fn split_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut start = 0;

    for (i, c) in text.char_indices() {
        if c == '.' || c == '!' || c == '?' {
            let end = i + 1;
            let slice = text[start..end].trim();
            if !slice.is_empty() {
                sentences.push(slice.to_string());
            }
            // Пропускаем пробелы и переносы
            start = end;
            while start < text.len() && text[start..].starts_with(|ch: char| ch.is_whitespace()) {
                start += 1;
            }
        }
    }

    // Добавим хвост, если остался (например, без точки)
    if start < text.len() {
        let slice = text[start..].trim();
        if !slice.is_empty() {
            sentences.push(slice.to_string());
        }
    }

    sentences
}

fn clean_words(sentence: &str, stemmer: &Stemmer) -> Vec<String> {
    WORD_RE
        .find_iter(&sentence.to_lowercase())
        .map(|m| m.as_str())
        .filter(|w| !STOP_WORDS.contains(w))
        .map(|w| stemmer.stem(w).to_string())
        .collect()
}

fn word_frequencies(text: &str, stemmer: &Stemmer) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in clean_words(text, stemmer) {
        *freq.entry(word).or_insert(0) += 1;
    }
    freq
}

fn sentence_score(sentence: &str, freq: &HashMap<String, usize>, stemmer: &Stemmer) -> usize {
    clean_words(sentence, stemmer)
        .iter()
        .map(|w| *freq.get(w).unwrap_or(&0))
        .sum()
}

fn select_best_sentences(text: &str, stemmer: &Stemmer) -> Vec<String> {
    let sentences: Vec<String> = split_sentences(text); // сначала разбили на предложения
    let freq = word_frequencies(text, stemmer); // потом частоты слов из всего текста

    let mut scored: Vec<(usize, usize, String)> = sentences
        .into_iter()
        .enumerate()
        .filter(|(_, s)| s.chars().count() <= MAX_LEN)
        .map(|(i, s)| {
            let score = sentence_score(&s, &freq, stemmer);
            (score, i, s)
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

    scored.into_iter().take(2).map(|(_, _, s)| s).collect()
}

pub struct Detector {
    stemmer: Stemmer,
}

impl Detector {
    pub fn new() -> Detector {
        Detector {
            stemmer: Stemmer::create(Algorithm::Russian),
        }
    }
    pub fn detect(&self, text: &str) -> Vec<String> {
        select_best_sentences(text, &self.stemmer)
    }
}
