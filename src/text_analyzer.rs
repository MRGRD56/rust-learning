use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

const WORDS_REGEX: Regex = Regex::new(r"\b[\w-]+\b").unwrap();

fn count_words(text: &str) -> HashMap<char, i32> {
    WORDS_REGEX.find_iter(text)
        .into_iter()
        .group_by(|x| x.as_str())
        .into()
        .map(|g| (g.key, g.vals.count()))
}