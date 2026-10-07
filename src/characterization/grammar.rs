// Deterministic grammar goldens: every word of the data tables through every
// form the grammar API can produce, and sorted dumps of the category tables.
//
// One line per case. Lines are sorted bytewise (the category tables are
// HashMaps, their iteration order changes at every run). Inside a line, the
// members of a category keep their declared order: it is a Vec, hence stable,
// and a reordering is a data change worth seeing.

use unicode_segmentation::UnicodeSegmentation;

use crate::adj_data::{ADJ_CATS, ADJS};
use crate::common_enums::{Article, Gender, Number};
use crate::noun::{
    get_apposition, get_cats_containing_affiliations, get_cats_containing_attributes,
    get_cats_containing_epithets, get_cats_containing_epithets_and_affiliations,
    get_cats_containing_int_verbs,
};
use crate::noun_data::{NOUN_CATS, NOUNS};
use crate::noun_enums::NounCatId;
use crate::verb::{as_pronominal, get_verb, get_verb_cat};
use crate::verb_data::{VERB_CATS, VERBS};

use super::golden::assert_golden_lines;
use super::labels::{adj_label, join, noun_label, opt_str, opt_wg, verb_label, wg};
use super::names::{
    adj_cat_name, adj_id_name, article_name, gender_name, noun_cat_name, noun_id_name, number_name,
    verb_cat_name, verb_group_name, verb_id_name, verb_kind_name,
};

pub const NUMBERS: [Number; 2] = [Number::Singular, Number::Plural];
pub const ARTICLES: [Article; 3] = [Article::None, Article::Definite, Article::Indefinite];
pub const GENDERS: [Gender; 2] = [Gender::Male, Gender::Female];

fn sorted(mut lines: Vec<String>) -> Vec<String> {
    lines.sort();
    lines
}

pub fn noun_lines() -> Vec<String> {
    let mut lines = vec![];
    for noun in NOUNS.iter() {
        let lemma = &noun.word.text;
        lines.push(format!(
            "noun {} | entry | id={} gender={} feet=({},{}) lookup_by_id=\"{}\"",
            lemma,
            noun_id_name(noun.id),
            gender_name(noun.gender),
            noun.word.foots.0,
            noun.word.foots.1,
            noun_label(noun.id)
        ));
        for number in NUMBERS.iter() {
            for article in ARTICLES.iter() {
                lines.push(format!(
                    "noun {} | {} {} | article={} agreed={} with_article={}",
                    lemma,
                    number_name(*number),
                    article_name(*article),
                    wg(&noun.get_article(*number, *article)),
                    wg(&noun.agreed(*number)),
                    wg(&noun.get_with_article(*article, *number))
                ));
            }
        }
        for article in ARTICLES.iter() {
            lines.push(format!(
                "noun {} | apposition {} | {}",
                lemma,
                article_name(*article),
                wg(&get_apposition(noun, *article))
            ));
        }
    }
    sorted(lines)
}

pub fn adj_lines() -> Vec<String> {
    let mut lines = vec![];
    for adj in ADJS.iter() {
        let lemma = &adj.word.text;
        lines.push(format!(
            "adj {} | entry | id={} feet=({},{}) fem={} masc_plur={} fem_plur={} invariable={} lookup_by_id=\"{}\"",
            lemma,
            adj_id_name(adj.id),
            adj.word.foots.0,
            adj.word.foots.1,
            opt_str(&adj.fem),
            opt_str(&adj.masc_plur),
            opt_str(&adj.fem_plur),
            adj.invariable,
            adj_label(adj.id)
        ));
        for gender in GENDERS.iter() {
            for number in NUMBERS.iter() {
                lines.push(format!(
                    "adj {} | {} {} | {}",
                    lemma,
                    gender_name(*gender),
                    number_name(*number),
                    wg(&adj.agreed(*gender, *number))
                ));
            }
        }
    }
    sorted(lines)
}

pub fn verb_lines() -> Vec<String> {
    let mut lines = vec![];
    for verb in VERBS.iter() {
        let lemma = &verb.word.text;
        lines.push(format!(
            "verb {} | entry | id={} feet=({},{}) group={} pronominal={} kind={} first_person={} third_person={} sixth_person={} lookup_by_id=\"{}\"",
            lemma,
            verb_id_name(verb.id),
            verb.word.foots.0,
            verb.word.foots.1,
            verb_group_name(verb.group),
            verb.is_pronominal,
            verb_kind_name(verb.kind),
            opt_wg(&verb.first_person),
            opt_wg(&verb.third_person),
            opt_wg(&verb.sixth_person),
            match get_verb(verb.id) {
                Ok(found) => found.word.text,
                Err(err) => err,
            }
        ));
        for number in NUMBERS.iter() {
            let agreed = verb.agree(number);
            lines.push(format!(
                "verb {} | {} | agree={}",
                lemma,
                number_name(*number),
                wg(&agreed)
            ));
            // `agree` already applies the pronoun to pronominal verbs; for the
            // others, also record what the pronominal helper would build.
            if !verb.is_pronominal {
                lines.push(format!(
                    "verb {} | {} | as_pronominal(agree)={}",
                    lemma,
                    number_name(*number),
                    wg(&as_pronominal(&agreed))
                ));
            }
        }
    }
    sorted(lines)
}

fn selector_line(name: &str, cats: Vec<NounCatId>) -> String {
    let mut names: Vec<&str> = cats.iter().map(|id| noun_cat_name(*id)).collect();
    names.sort();
    format!("selector {} | {} | {}", name, names.len(), names.join(", "))
}

pub fn noun_category_lines() -> Vec<String> {
    let mut lines = vec![];
    for (id, cat) in NOUN_CATS.iter() {
        let name = noun_cat_name(*id);
        let mut push = |field: &str, value: String| {
            lines.push(format!("noun_cat {} | {} | {}", name, field, value));
        };
        push("id", String::from(noun_cat_name(cat.id)));
        push("nouns", join(&cat.nouns, |noun| noun_label(*noun)));
        push(
            "attributes",
            join(&cat.rel.attributes, |c| String::from(adj_cat_name(*c))),
        );
        push(
            "epithets",
            join(&cat.rel.epithets, |c| String::from(adj_cat_name(*c))),
        );
        push(
            "functions",
            join(&cat.rel.functions, |c| String::from(verb_cat_name(*c))),
        );
        push(
            "emissions",
            join(&cat.rel.emissions, |c| String::from(noun_cat_name(*c))),
        );
        push(
            "affiliations",
            join(&cat.rel.affiliations, |c| String::from(noun_cat_name(*c))),
        );
        push(
            "has_intransitive_verb",
            format!("{}", cat.has_intransitive_verb()),
        );
    }
    lines.push(format!("noun_cat count | {}", NOUN_CATS.len()));
    lines.push(selector_line(
        "get_cats_containing_epithets",
        get_cats_containing_epithets(),
    ));
    lines.push(selector_line(
        "get_cats_containing_attributes",
        get_cats_containing_attributes(),
    ));
    lines.push(selector_line(
        "get_cats_containing_affiliations",
        get_cats_containing_affiliations(),
    ));
    lines.push(selector_line(
        "get_cats_containing_epithets_and_affiliations",
        get_cats_containing_epithets_and_affiliations(),
    ));
    lines.push(selector_line(
        "get_cats_containing_int_verbs",
        get_cats_containing_int_verbs(),
    ));
    sorted(lines)
}

pub fn adj_category_lines() -> Vec<String> {
    let mut lines = vec![];
    for (id, adjs) in ADJ_CATS.iter() {
        lines.push(format!(
            "adj_cat {} | {}",
            adj_cat_name(*id),
            join(adjs, |adj| adj_label(*adj))
        ));
    }
    lines.push(format!("adj_cat count | {}", ADJ_CATS.len()));
    sorted(lines)
}

pub fn verb_category_lines() -> Vec<String> {
    let mut lines = vec![];
    for (id, verbs) in VERB_CATS.iter() {
        lines.push(format!(
            "verb_cat {} | {}",
            verb_cat_name(*id),
            join(verbs, |verb| verb_label(*verb))
        ));
        lines.push(format!(
            "verb_cat {} | get_verb_cat | {}",
            verb_cat_name(*id),
            match get_verb_cat(id) {
                Ok(found) => join(&found, |verb| verb_label(*verb)),
                Err(err) => err,
            }
        ));
    }
    lines.push(format!("verb_cat count | {}", VERB_CATS.len()));
    sorted(lines)
}

#[test]
fn golden_nouns() {
    assert_golden_lines("nouns.txt", &noun_lines());
}

#[test]
fn golden_adjectives() {
    assert_golden_lines("adjectives.txt", &adj_lines());
}

#[test]
fn golden_verbs() {
    assert_golden_lines("verbs.txt", &verb_lines());
}

#[test]
fn golden_noun_categories() {
    assert_golden_lines("noun_categories.txt", &noun_category_lines());
}

#[test]
fn golden_adjective_categories() {
    assert_golden_lines("adjective_categories.txt", &adj_category_lines());
}

#[test]
fn golden_verb_categories() {
    assert_golden_lines("verb_categories.txt", &verb_category_lines());
}

/// Every string of the data tables, with a label saying where it comes from.
pub fn table_strings() -> Vec<(String, String)> {
    let mut all = vec![];
    for noun in NOUNS.iter() {
        all.push((String::from("noun"), noun.word.text.clone()));
    }
    for adj in ADJS.iter() {
        all.push((String::from("adj"), adj.word.text.clone()));
        for form in [&adj.fem, &adj.masc_plur, &adj.fem_plur].iter() {
            if let Some(text) = form {
                all.push((String::from("adj form"), text.clone()));
            }
        }
    }
    for verb in VERBS.iter() {
        all.push((String::from("verb"), verb.word.text.clone()));
        for form in [&verb.first_person, &verb.third_person, &verb.sixth_person].iter() {
            if let Some(word) = form {
                all.push((String::from("verb form"), word.text.clone()));
            }
        }
    }
    all
}

pub fn graphemes_differ_from_chars(text: &str) -> bool {
    text.graphemes(true).count() != text.chars().count()
}

// The production code slices words by extended grapheme clusters. As long as
// every string it can meet has exactly one char per grapheme, the grapheme
// helpers are equivalent to char helpers and a unicode-segmentation upgrade
// (new Unicode tables) cannot change a result.
#[test]
fn every_table_string_has_one_char_per_grapheme() {
    let strings = table_strings();
    // 71 nouns + 59 adjectives (+ 6 special forms) + 13 verbs (+ 12 persons)
    assert_eq!(strings.len(), 161);
    let offenders: Vec<String> = strings
        .iter()
        .filter(|(_, text)| graphemes_differ_from_chars(text))
        .map(|(origin, text)| format!("{} {:?}", origin, text))
        .collect();
    assert!(
        offenders.is_empty(),
        "graphemes != chars for {:?}",
        offenders
    );
}

// Same check on everything the grammar derives from the tables (plurals,
// feminines, conjugations, articles, appositions).
#[test]
fn every_derived_form_has_one_char_per_grapheme() {
    let mut lines = noun_lines();
    lines.extend(adj_lines());
    lines.extend(verb_lines());
    let offenders: Vec<&String> = lines
        .iter()
        .filter(|line| graphemes_differ_from_chars(line))
        .collect();
    assert!(
        offenders.is_empty(),
        "graphemes != chars for {:?}",
        offenders
    );
}
