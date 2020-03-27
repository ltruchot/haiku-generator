use crate::enums;
use enums::{Gender, AdjId, Article, Number}; 

#[derive(Clone)]
pub struct Noun {
    pub lemme: String,
    pub gender: Gender,
    pub emit: Vec<String>,
    pub can_be: Vec<AdjId>,
}

impl Noun {
    pub fn new(lemme: &str, gender: Gender, emissions: Vec<String>, adjs: Vec<AdjId>) -> Noun {
        Noun {
            lemme: String::from(lemme),
            gender: gender,
            emit: emissions,
            can_be: adjs,
        }
    }
}

pub fn get_with_some_article(article: Article, number: Number, noun: &Noun) -> String {
    let article = match number {
        Number::Plural => {
            match article {
                Article::Definite => "les",
                Article::Indefinite => "des",
            }
        },
        Number::Singular => { 
            match noun.gender {
                Gender::Male => match article {
                    Article::Definite => "le",
                    Article::Indefinite => "un",
                },
                Gender::Female => match article {
                    Article::Definite => "la",
                    Article::Indefinite => "une",
                },
            }
        }
    };
    let mut word = String::new();
    word.push_str(article);
    word.push(' ');
    word.push_str(
        &match number {
            Number::Plural => {
                let mut plural = String::from(&noun.lemme);
                plural.push('s');
                plural
            }
            Number::Singular => String::from(&noun.lemme)
        }
    );
    word
}

pub fn get_apposition(noun: &Noun) -> String {
    let first = noun.lemme.chars().next();
    match first {
        Some(letter) => {
            let ellisions = ['a', 'é', 'h'];
            [
                if ellisions.contains(&letter) {
                    "d'"
                } else {
                    "de "
                },
                &noun.lemme,
            ]
            .join("")
        }
        None => String::from(""),
    }
}
