// IMPORTS
// commons
use crate::common_enums;
use common_enums::{Article, Gender, Number};

// wordgroups
use crate::wordgroup;
use wordgroup::{add_words, check_ellision, WordGroup};

// nouns
use crate::noun_data;
use crate::noun_enums;
use noun_data::{NOUN_CATS};
use noun_enums::{NounCatId, NounId};

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
                            foots: (1, 1),
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
    let apposition = match article {
        Article::None => match first {
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
        },
        Article::Definite => match noun.gender {
            Gender::Female => WordGroup {
                text: {
                    [
                        "de ",
                        &noun.get_article(Number::Singular, Article::Definite).text,
                    ]
                    .join("")
                },
                foots: (2, 2),
            },
            Gender::Male => WordGroup {
                text: match first {
                    Some(letter) => {
                        if check_ellision(&letter) {
                            String::from("de l'")
                        } else {
                            String::from("du ")
                        }
                    }
                    None => String::from("#error#get_apposition#No first letter#"),
                },
                foots: (1, 1),
            },
        },
        Article::Indefinite => match noun.gender {
            Gender::Female => WordGroup {
                text: {
                    [
                        "d'",
                        &noun.get_article(Number::Singular, Article::Indefinite).text,
                    ]
                    .join("")
                },
                foots: (1, 2),
            },
            Gender::Male => WordGroup {
                text: {
                    [
                        "d'",
                        &noun.get_article(Number::Singular, Article::Indefinite).text,
                    ]
                    .join("")
                },
                foots: (1, 1),
            },
        },
    };
    add_words(&apposition, &noun.word, false)
}

