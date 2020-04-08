use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

// commons
use crate::common_enums;
use common_enums::{Article, Gender, Number};

// wordgroups
use crate::wordgroup;
use wordgroup::{add_words, check_ellision, WordGroup};

// adjectives
use crate::adj_enums;
use adj_enums::AdjCatId;

// nouns
use crate::noun_data;
use crate::noun_enums;
use noun_data::{NOUNS, NOUN_CATS};
use noun_enums::{NounCatId, NounId};

// verbs
use crate::verb;
use crate::verb_enums;
use verb::{get_verb, Verb, get_verb_cat};
use verb_enums::VerbCatId;

// strings
use crate::string;
use string::get_plural;

#[derive(Clone)]
pub struct Noun {
    pub id: NounId,
    pub gender: Gender,
    pub word: WordGroup,
}

impl Noun {
    pub fn new(id: NounId, lemme: &str, gender: Gender, foots: (u8, u8)) -> Noun {
        Noun {
            id,
            gender,
            word: WordGroup {
                text: String::from(lemme),
                foots: foots,
            },
        }
    }

    pub fn with_verb(self: &Self, verb: &Verb, number: Number) -> WordGroup {
        let agreed_verb = verb.agreed(number);
        add_words(&self.word, &agreed_verb, true)
    }

    pub fn get_article(self: &Self, number: Number, article: Article) -> WordGroup {
        let article = match number {
            Number::Plural => match article {
                Article::Definite => WordGroup {
                    text: String::from("les "),
                    foots: (1, 1),
                },
                Article::Indefinite => WordGroup {
                    text: String::from("des "),
                    foots: (1, 1),
                },
            },
            Number::Singular => {
                let first = self.word.text.chars().next();
                let has_ellision = match first {
                    Some(letter) => check_ellision(&letter),
                    None => false,
                };
                match self.gender {
                    Gender::Male => match article {
                        Article::Definite => match first {
                            Some(letter) => {
                                if check_ellision(&letter) {
                                    WordGroup {
                                        text: String::from("l'"),
                                        foots: (0, 0),
                                    }
                                } else {
                                    WordGroup {
                                        text: String::from("le "),
                                        foots: (1, 1),
                                    }
                                }
                            }
                            None => WordGroup {
                                text: String::from(
                                    "#error#get_with_some_article#No first letter for ellision#",
                                ),
                                foots: (0, 0),
                            },
                        },
                        Article::Indefinite => WordGroup {
                            text: String::from("un "),
                            foots: (1, 1),
                        },
                    },
                    Gender::Female => match article {
                        Article::Definite => match first {
                            Some(letter) => {
                                if check_ellision(&letter) {
                                    WordGroup {
                                        text: String::from("l'"),
                                        foots: (0, 0),
                                    }
                                } else {
                                    WordGroup {
                                        text: String::from("la "),
                                        foots: (1, 1),
                                    }
                                }
                            }
                            None => WordGroup {
                                text: String::from(
                                    "#error#get_with_some_article#No first letter for ellision#",
                                ),
                                foots: (0, 0),
                            },
                        },
                        Article::Indefinite => WordGroup {
                            text: String::from("une "),
                            foots: (1, if has_ellision { 1 } else { 2 }),
                        },
                    },
                }
            }
        };
        article
    }

    pub fn agreed(self: &Self, number: Number) -> WordGroup {
        match number {
            Number::Plural => WordGroup {
                text: get_plural(&self.word.text),
                foots: (self.word.foots),
            },
            Number::Singular => self.word.clone(),
        }
    }

    pub fn get_with_article(self: &Self, article: Article, number: Number) -> WordGroup {
        let article = self.get_article(number, article);
        let agreed_noun = self.agreed(number);
        add_words(&article, &agreed_noun, false)
    }
}

pub fn get_cats_containing_attrs() -> Vec<NounCatId> {
    NOUN_CATS.iter().fold(vec![], |mut acc, cur| {
        let (id, cat) = cur;
        if cat.rel.attributes.len() > 0 {
            acc.push(id.clone());
            acc
        } else {
            acc
        }
    })
}

pub fn get_cats_containing_int_verbs() -> Vec<NounCatId> {
    NOUN_CATS.iter().fold(vec![], |mut acc, cur| {
        let (id, cat) = cur;
        if cat.has_intransitive_verb() {
            acc.push(id.clone());
        }
        acc   
    })
}

pub fn extract_wordgroup(noun: &Noun) -> WordGroup {
    noun.word.clone()
}

pub fn get_apposition(noun: &Noun) -> WordGroup {
    let first = noun.word.text.chars().next();
    let apposition = match first {
        Some(letter) => {
            if check_ellision(&letter) {
                WordGroup {
                    text: String::from("d'"),
                    foots: (0, 0),
                }
            } else {
                WordGroup {
                    text: String::from("de "),
                    foots: (1, 1),
                }
            }
        }
        None => WordGroup {
            text: String::from("#error#get_apposition#No first letter#"),
            foots: (0, 0),
        },
    };
    add_words(&apposition, &noun.word, false)
}

pub fn pick_rand_noun(
    cats: &Vec<NounCatId>,
    rng: &mut ThreadRng,
    blacklist: &Vec<NounId>,
) -> Option<Noun> {
    let mut noun: Option<Noun> = None;
    while noun.is_none() {
        let rand_noun = cats
            .choose(rng)
            .and_then(|choice| NOUN_CATS.get(&choice))
            .and_then(|cat| cat.nouns.choose(rng))
            .and_then(|id| NOUNS.iter().cloned().find(|item| &item.id == id));
        match rand_noun {
            Some(chosen) => {
                if !blacklist.contains(&chosen.id) {
                    noun = Some(chosen);
                }
            }
            None => (),
        }
    }
    noun
}
