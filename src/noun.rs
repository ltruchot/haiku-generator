use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

// commons
use crate::common_enums;
use common_enums::{Article, Gender, Number};

// wordgroups
use crate::wordgroup;
use wordgroup::{add_words, check_ellision, WordGroup};

// nouns
use crate::noun_data;
use crate::noun_enums;
use noun_data::{NOUNS, NOUN_CATS};
use noun_enums::{NounCatId, NounId};

// verbs
use crate::verb;
use crate::verb_enums;
use verb::{Verb};

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
                Article::None => WordGroup::new_empty(),
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
                        Article::None => WordGroup::new_empty(),
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
                        Article::None => WordGroup::new_empty(),
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

pub fn get_cats_containing_epithets_and_affiliations() -> Vec<NounCatId> {
    NOUN_CATS.iter().fold(vec![], |mut acc, cur| {
        let (id, cat) = cur;
        if cat.rel.epithets.len() > 0 && cat.rel.affiliations.len() > 0 {
            acc.push(id.clone());
        }
        acc
    })
}

pub fn get_cats_containing_epithets() -> Vec<NounCatId> {
    NOUN_CATS.iter().fold(vec![], |mut acc, cur| {
        let (id, cat) = cur;
        if cat.rel.epithets.len() > 0 {
            acc.push(id.clone());
        }
        acc
    })
}

pub fn get_cats_containing_attributes() -> Vec<NounCatId> {
    NOUN_CATS.iter().fold(vec![], |mut acc, cur| {
        let (id, cat) = cur;
        if cat.rel.attributes.len() > 0 {
            acc.push(id.clone());
        }
        acc
    })
}

pub fn get_cats_containing_affiliations() -> Vec<NounCatId> {
    NOUN_CATS.iter().fold(vec![], |mut acc, cur| {
        let (id, cat) = cur;
        if cat.rel.affiliations.len() > 0 {
            acc.push(id.clone());
        }
        acc
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

pub fn get_apposition(noun: &Noun, article: Article) -> WordGroup {
    let first = noun.word.text.chars().next();
    match article {
        Article::None => {
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
        Article::Definite => { 
            match noun.gender {
                Gender::Female => WordGroup {
                    text: {
                        [
                            "de ",
                            &noun
                                .get_with_article(Article::Definite, Number::Singular)
                                .text,
                        ]
                        .join("")
                    },
                    foots: (2, 2),
                },
                Gender::Male => WordGroup {
                    text: ["du ", &noun.word.text].join(""),
                    foots: (1, 1),
                },
            }
        }
        Article::Indefinite => WordGroup {
            text: {
                [
                    "d'",
                    &noun
                        .get_with_article(Article::Indefinite, Number::Singular)
                        .text,
                ]
                .join("")
            },
            foots: (1, 1),
        }
    }
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
