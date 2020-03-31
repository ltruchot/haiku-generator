use crate::common_enums;
use common_enums::{Article, Gender, Number};

use crate::word;
use word::{WordGroup, add_words};

use crate::adj_enums;
use adj_enums::{AdjId, AdjCatId};

use crate::noun_enums;
use noun_enums::{NounId, NounCatId};

#[derive(Clone)]
pub struct Noun {
    pub id: NounId,
    pub gender: Gender,
    pub emit: Vec<NounCatId>,
    pub can_be: Vec<AdjCatId>,
    pub word: WordGroup,
}

impl Noun {
    pub fn new(
        id: NounId,
        lemme: &str,
        gender: Gender,
        emissions: Vec<NounCatId>,
        adjs: Vec<AdjCatId>,
        foots: (u8, u8),
    ) -> Noun {
        Noun {
            id: id,
            gender: gender,
            emit: emissions,
            can_be: adjs,
            word: WordGroup {
                text: String::from(lemme),
                foots: foots,
            },
        }
    }
}

pub fn extract_wordgroup(noun: &Noun) -> WordGroup {
    noun.word.clone()
}

pub fn get_with_some_article(article: Article, number: Number, noun: &Noun) -> WordGroup {
    let article: WordGroup = match number {
        Number::Plural => match article {
            Article::Definite => WordGroup{ text: String::from("les "), foots: (1, 1) },
            Article::Indefinite => WordGroup{ text: String::from("des "), foots: (1, 1) },
        },
        Number::Singular => {
            let first = noun.word.text.chars().next();
            let has_ellision = match first {
                Some(letter) => check_ellision(letter),
                None => false
            };
            match noun.gender {
                Gender::Male => match article {
                    Article::Definite => match first { 
                        Some(letter) => if check_ellision(letter) {
                            WordGroup{ text: String::from("l'"), foots: (0, 0) }
                        } else {
                            WordGroup{ text: String::from("le "), foots: (1, 1) }
                        },
                        None => WordGroup { 
                            text: String::from("#error#get_with_some_article#No first letter for ellision#"), 
                            foots: (0, 0)
                        }
                    },
                    Article::Indefinite => WordGroup{ text: String::from("un "), foots: (1, 1) },
                },
                Gender::Female => match article {
                    Article::Definite => match first { 
                        Some(letter) => if check_ellision(letter) {
                            WordGroup { 
                                text: String::from("l'"), foots: (0, 0) }
                        } else {
                            WordGroup{ text: String::from("la "), foots: (1, 1) }
                        },
                        None => WordGroup { 
                            text: String::from("#error#get_with_some_article#No first letter for ellision#"),
                            foots: (0, 0)
                        }
                    },
                    Article::Indefinite => WordGroup{ text: String::from("une "), foots: (1, if has_ellision { 1 } else {2}) },
                }
            }
        }
    };
    let agreed_noun = agree_noun(&noun, number);
    add_words(article, agreed_noun)
}

pub fn agree_noun (noun: &Noun, number: Number) -> WordGroup {
    match number {
        Number::Plural => {
            let mut plural = String::from(&noun.word.text);
            plural.push('s');
            WordGroup {
                text: plural,
                foots: (noun.word.foots)
            }
        }
        Number::Singular => noun.word.clone(),
    }
}

pub fn check_ellision (letter: char) -> bool {
    let ellisions = ['a', 'e', 'i', 'o', 'u', 'é', 'h'];
    ellisions.contains(&letter)
}

pub fn get_apposition(noun: &Noun) -> WordGroup {
    let first = noun.word.text.chars().next();
    add_words(match first {
        Some(letter) => if check_ellision(letter) {
            WordGroup { text: String::from("d'"), foots: (0, 0) }
        } else {
            WordGroup{ text: String::from("de "), foots: (1, 1) }
        },        
        None => WordGroup { 
            text: String::from("#error#get_apposition#No first letter#"),
            foots: (0, 0)
        },
    }, noun.word.clone())
}
