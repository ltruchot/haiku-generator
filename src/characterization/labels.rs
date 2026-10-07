// Labels shared by the dumps: a word is named by its lemma, an id without
// any entry in its table by `<no entry: Variant>`.

use crate::adj::Adj;
use crate::adj_data::ADJS;
use crate::adj_enums::AdjId;
use crate::noun::Noun;
use crate::noun_data::NOUNS;
use crate::noun_enums::NounId;
use crate::verb::Verb;
use crate::verb_data::VERBS;
use crate::verb_enums::VerbId;
use crate::wordgroup::WordGroup;

use super::names::{adj_id_name, noun_id_name, verb_id_name};

/// `"text" (min,max)`
pub fn wg(word: &WordGroup) -> String {
    format!("\"{}\" ({},{})", word.text, word.foots.0, word.foots.1)
}

pub fn opt_wg(word: &Option<WordGroup>) -> String {
    match word {
        Some(word) => wg(word),
        None => String::from("-"),
    }
}

pub fn opt_str(text: &Option<String>) -> String {
    match text {
        Some(text) => format!("\"{}\"", text),
        None => String::from("-"),
    }
}

// The three lookups below are the ones production code uses: first entry of
// the table carrying the id.
pub fn find_noun(id: NounId) -> Option<&'static Noun> {
    NOUNS.iter().find(|noun| noun.id == id)
}

pub fn find_adj(id: AdjId) -> Option<&'static Adj> {
    ADJS.iter().find(|adj| adj.id == id)
}

pub fn find_verb(id: VerbId) -> Option<&'static Verb> {
    VERBS.iter().find(|verb| verb.id == id)
}

pub fn noun_label(id: NounId) -> String {
    match find_noun(id) {
        Some(noun) => noun.word.text.clone(),
        None => format!("<no entry: {}>", noun_id_name(id)),
    }
}

pub fn adj_label(id: AdjId) -> String {
    match find_adj(id) {
        Some(adj) => adj.word.text.clone(),
        None => format!("<no entry: {}>", adj_id_name(id)),
    }
}

pub fn verb_label(id: VerbId) -> String {
    match find_verb(id) {
        Some(verb) => verb.word.text.clone(),
        None => format!("<no entry: {}>", verb_id_name(id)),
    }
}

pub fn join<T, F: Fn(&T) -> String>(items: &[T], label: F) -> String {
    if items.is_empty() {
        String::from("-")
    } else {
        items.iter().map(|item| label(item)).collect::<Vec<String>>().join(", ")
    }
}
