// IMPORTS
use crate::common_enums;
use common_enums::{Article, Number, BlackLists};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

// wordgroups
use crate::wordgroup;
use wordgroup::{fold_wordgroups, WordGroup};

// verbs
use crate::verb;
use crate::verb_data;
use crate::verb_enums;
use verb::{get_verb_cat};
use verb_data::{VERBS};
use verb_enums::{VerbCatId, VerbId, VerbKind};

// nouns
use crate::noun;
use noun::{
    get_apposition, get_cats_containing_affiliations, get_cats_containing_attributes,
    get_cats_containing_epithets, get_cats_containing_epithets_and_affiliations,
    get_cats_containing_int_verbs,
};


// random
use crate::random;
use random::{get_rand_adj, get_rand_adj_cat, get_rand_noun, get_rand_noun_cat, get_rand_verb};

// EXPORTS
pub type Combination = Box<dyn Fn(&BlackLists) -> Result<(WordGroup, BlackLists), Vec<String>>>;

pub fn get_with_adj_and_affiliation(
    number_opt: Option<Number>,
    articles_opt: Option<Vec<Article>>,
) -> Combination {
    Box::new(move |blacklists| {
        let mut rng = rand::thread_rng();
        let mut errs: Vec<String> = vec![];
        // pick optional number and article
        let number = number_opt.unwrap_or(get_rand_number(&mut rng));
        let article_type = get_rand_article(&mut rng, &articles_opt);

        // extract categories with attributes
        let cond_cats = get_cats_containing_epithets_and_affiliations();

        let cat_noun_opt = match get_rand_noun_cat(&mut rng, &cond_cats) {
            Ok(success) => Some(success),
            Err(err) => {
                errs.push(err);
                None
            }
        };

        let affiliations = match &cat_noun_opt {
            Some(cat) => match get_rand_noun_cat(&mut rng, &cat.rel.affiliations) {
                Ok(cat) => cat.nouns,
                Err(err) => {
                    errs.push(err);
                    vec![]
                }
            },
            None => vec![],
        };

        let epithets = match &cat_noun_opt {
            Some(cat) => match get_rand_adj_cat(&mut rng, &cat.rel.epithets) {
                Ok(eps) => eps,
                Err(err) => {
                    errs.push(err);
                    vec![]
                }
            },
            None => vec![],
        };

        // early return if categories not found
        if errs.len() > 0 {
            return Err(errs);
        }

        // find words and collect errors
        let wg_empty = WordGroup::new_empty();
        let mut new_blacklists = BlackLists::new_empty();
        let (noun, adj, aff) = match get_rand_noun(&mut rng, &cat_noun_opt.unwrap().nouns) {
            Ok(noun) => {
                new_blacklists.nouns.push(noun.id);
                let noun_wg = noun.get_with_article(article_type, number);
                let adj_wg = match get_rand_adj(&mut rng, &epithets) {
                    Ok(adj) => {
                        new_blacklists.adjs.push(adj.id);
                        adj.agreed(noun.gender, number)
                    },
                    Err(err) => {
                        errs.push(err);
                        wg_empty.clone()
                    }
                };
                match get_rand_noun(&mut rng, &affiliations) {
                    Ok(aff_noun) => {
                        new_blacklists.nouns.push(aff_noun.id);
                        (
                        noun_wg,
                        adj_wg,
                        get_apposition(&aff_noun, get_rand_article(&mut rng, &None)),
                    )},
                    Err(err) => {
                        errs.push(err);
                        (noun_wg, wg_empty.clone(), wg_empty.clone())
                    }
                }
            }
            Err(err) => {
                errs.push(err);
                (wg_empty.clone(), wg_empty.clone(), wg_empty.clone())
            }
        };

        // early return if words not found
        if errs.len() > 0 {
            return Err(errs);
        }

        Ok((fold_wordgroups(vec![noun, adj, aff]), new_blacklists))
    })
}

pub fn get_with_affiliation(
    number_opt: Option<Number>,
    articles_opt: Option<Vec<Article>>,
) -> Combination {
    Box::new(move |blacklists|  {
        let mut rng = rand::thread_rng();
        let mut errs: Vec<String> = vec![];
        // pick optional number and article
        let number = number_opt.unwrap_or(get_rand_number(&mut rng));
        let article_type = get_rand_article(&mut rng, &articles_opt);

        // extract categories with affiliations
        let cats_with_affiliations = get_cats_containing_affiliations();

        let cat_noun_opt = match get_rand_noun_cat(&mut rng, &cats_with_affiliations) {
            Ok(success) => Some(success),
            Err(err) => {
                errs.push(err);
                None
            }
        };

        let cat_affiliation = match &cat_noun_opt {
            Some(cat) => match get_rand_noun_cat(&mut rng, &cat.rel.affiliations) {
                Ok(cat) => cat.nouns,
                Err(err) => {
                    errs.push(err);
                    vec![]
                }
            },
            None => vec![],
        };

        // early return if categories not found
        if errs.len() > 0 {
            return Err(errs);
        }

        // find words and collect errors
        let mut new_blacklists = BlackLists::new_empty();
        let wg_empty = WordGroup::new_empty();
        let (noun, aff) = match get_rand_noun(&mut rng, &cat_noun_opt.unwrap().nouns) {
            Ok(noun) => {
                new_blacklists.nouns.push(noun.id);
                let noun_wg = noun.get_with_article(article_type, number);
                match get_rand_noun(&mut rng, &cat_affiliation) {
                    Ok(aff_noun) => {
                        new_blacklists.nouns.push(aff_noun.id);
                        (noun_wg, get_apposition(&aff_noun, Article::None))
                    },
                    Err(err) => {
                        errs.push(err);
                        (noun_wg, wg_empty.clone())
                    }
                }
            }
            Err(err) => {
                errs.push(err);
                (wg_empty.clone(), wg_empty.clone())
            }
        };

        // early return if words not found
        if errs.len() > 0 {
            return Err(errs);
        }
        
        Ok((fold_wordgroups(vec![noun, aff]), new_blacklists))
    })
}

pub fn get_with_adjective(
    number_opt: Option<Number>,
    articles_opt: Option<Vec<Article>>,
) -> Combination {
    Box::new(move |blacklists|  {
        let mut rng = rand::thread_rng();
        let mut errs: Vec<String> = vec![];
        // pick optional number and article
        let number = number_opt.unwrap_or(get_rand_number(&mut rng));
        let article_type = get_rand_article(&mut rng, &articles_opt);

        // extract categories with epithets
        let cats_with_epithets = get_cats_containing_epithets();

        let cat_noun_opt = match get_rand_noun_cat(&mut rng, &cats_with_epithets) {
            Ok(success) => Some(success),
            Err(err) => {
                errs.push(err);
                None
            }
        };

        let cat_adj = match &cat_noun_opt {
            Some(cat) => get_rand_adj_cat(&mut rng, &cat.rel.epithets).unwrap_or_else(|err| {
                errs.push(err);
                vec![]
            }),
            None => vec![],
        };

        // early return if categories not found
        if errs.len() > 0 {
            return Err(errs);
        }

        // find words and collect errors
        let mut new_blacklists = BlackLists::new_empty();
        let wg_empty = WordGroup::new_empty();
        let (noun, adj) = match get_rand_noun(&mut rng, &cat_noun_opt.unwrap().nouns) {
            Ok(noun) => {
                new_blacklists.nouns.push(noun.id);
                let noun_wg = noun.get_with_article(article_type, number);
                match get_rand_adj(&mut rng, &cat_adj) {
                    Ok(adj) => {
                        new_blacklists.adjs.push(adj.id);
                        (noun_wg, adj.agreed(noun.gender, number))
                    },
                    Err(err) => {
                        errs.push(err);
                        (noun_wg, wg_empty.clone())
                    }
                }
            }
            Err(err) => {
                errs.push(err);
                (wg_empty.clone(), wg_empty.clone())
            }
        };

        // early return if words not found
        if errs.len() > 0 {
            return Err(errs);
        }

        Ok((fold_wordgroups(vec![noun, adj]), new_blacklists))
    })
}

pub fn get_with_linking_verb(
    number_opt: Option<Number>,
    articles_opt: Option<Vec<Article>>,
) -> Combination {
    Box::new(move |blacklists|  {
        let mut rng = rand::thread_rng();
        let mut errs: Vec<String> = vec![];
        // pick optional number and article
        let number = number_opt.unwrap_or(get_rand_number(&mut rng));
        let article_type = get_rand_article(&mut rng, &articles_opt);

        // extract categories with attributes
        let cats_with_attributes = get_cats_containing_attributes();

        // find categories and collect errors
        let cat_linking_verb = get_verb_cat(&VerbCatId::Etat).unwrap_or_else(|err| {
            errs.push(err);
            vec![]
        });

        let cat_noun_opt = match get_rand_noun_cat(&mut rng, &cats_with_attributes) {
            Ok(success) => Some(success),
            Err(err) => {
                errs.push(err);
                None
            }
        };

        let cat_adj = match &cat_noun_opt {
            Some(cat) => get_rand_adj_cat(&mut rng, &cat.rel.attributes).unwrap_or_else(|err| {
                errs.push(err);
                vec![]
            }),
            None => vec![],
        };

        // early return if categories not found
        if errs.len() > 0 {
            return Err(errs);
        }

        // find words and collect errors
        let mut new_blacklists = BlackLists::new_empty();
        let wg_empty = WordGroup::new_empty();
        let linking_verb = match get_rand_verb(&mut rng, &cat_linking_verb) {
            Ok(verb) => verb.agreed(number),
            Err(err) => {
                errs.push(err);
                wg_empty.clone()
            }
        };
        let (groupe_nominal, adj) = match get_rand_noun(&mut rng, &cat_noun_opt.unwrap().nouns) {
            Ok(noun) => {
                let noun_wg = noun.get_with_article(article_type, number);
                match get_rand_adj(&mut rng, &cat_adj) {
                    Ok(adj) => (noun_wg, adj.agreed(noun.gender, number)),
                    Err(err) => {
                        errs.push(err);
                        (noun_wg, wg_empty.clone())
                    }
                }
            }
            Err(err) => {
                errs.push(err);
                (wg_empty.clone(), wg_empty.clone())
            }
        };

        // early return if words not found
        if errs.len() > 0 {
            return Err(errs);
        }

        Ok((fold_wordgroups(vec![groupe_nominal, linking_verb, adj]), new_blacklists))
    })
}

pub fn get_with_intransitive_verb(
    number_opt: Option<Number>,
    articles_opt: Option<Vec<Article>>,
) -> Combination {
    Box::new(move |blacklists|  {
        let mut rng = rand::thread_rng();
        let mut errs: Vec<String> = vec![];
        // pick optional number and article
        let number = number_opt.unwrap_or(get_rand_number(&mut rng));
        let article_type = get_rand_article(&mut rng, &articles_opt);

        // extract categories with intransitive verbs
        let cats_with_int_verbs = get_cats_containing_int_verbs();
        // early return if categories not found
        if cats_with_int_verbs.len() == 0 {
            errs.push(String::from(
                "err#get_with_intransitive_verb#No category with intransitive verb",
            ));
            return Err(errs);
        }

        let cat_res = get_rand_noun_cat(&mut rng, &cats_with_int_verbs);

        let noun_res = cat_res
            .clone()
            .and_then(|cat| get_rand_noun(&mut rng, &cat.nouns));

        let mut new_blacklists = BlackLists::new_empty();
        match noun_res {
            Ok(noun) => {
                let wg_nominal = noun.get_with_article(article_type, number);
                let intransitive_verbs: Vec<VerbId> = match cat_res {
                    Ok(cat) => cat
                        .rel
                        .functions
                        .iter()
                        .fold(&mut vec![], |acc, cur| {
                            let verbs = get_verb_cat(cur).and_then(|verb_cat| {
                                Ok(verb_cat
                                    .iter()
                                    .filter(|verb_id| {
                                        match VERBS.iter().find(|verb| verb_id.clone() == &verb.id)
                                        {
                                            Some(verb) => verb.kind == VerbKind::Intransitive,
                                            None => false,
                                        }
                                    })
                                    .cloned()
                                    .collect::<Vec<VerbId>>())
                            });
                            match verbs {
                                Ok(filtered) => {
                                    acc.extend(filtered);
                                }
                                Err(err) => {
                                    errs.push(err);
                                }
                            }
                            acc
                        })
                        .clone(),
                    Err(err) => {
                        errs.push(err);
                        vec![]
                    }
                };
                if intransitive_verbs.len() > 0 {
                    let verb_opt = intransitive_verbs
                        .choose(&mut rng)
                        .and_then(|verb_id| VERBS.iter().find(|verb| verb_id.clone() == verb.id));
                    match verb_opt {
                        Some(verb) => Ok((fold_wordgroups(vec![wg_nominal, verb.agreed(number)]), new_blacklists)),
                        None => {
                            errs.push(String::from(
                                "err#get_with_intransitive_verb#No intransitive verb for this noun",
                            ));
                            Err(errs)
                        }
                    }
                } else {
                    errs.push(String::from(
                        "err#get_with_intransitive_verb#No intransitive verb for this noun",
                    ));
                    Err(errs)
                }
            }
            Err(err) => {
                errs.push(err.clone());
                Err(errs)
            }
        }
    })
}

pub fn get_rand_article(rng: &mut ThreadRng, articles_opt: &Option<Vec<Article>>) -> Article {
    articles_opt
        .as_ref()
        .unwrap_or(&vec![Article::None, Article::Definite, Article::Indefinite])
        .choose(rng)
        .unwrap_or(&Article::Indefinite)
        .clone()
}

pub fn get_rand_number(rng: &mut ThreadRng) -> Number {
    [Number::Singular, Number::Plural]
        .choose(rng)
        .unwrap_or(&Number::Singular)
        .clone()
}

// pub fn get_with_linking_verb(
//     verb_cats: Vec<VerbCatId>,
//     number_opt: Option<Number>,
//     article_opt: Option<Article>,
// ) -> Combination {
//     Box::new(move |noun: &Noun| {
//         let mut rng = thread_rng();
//         let number = number_opt.unwrap_or(get_rand_number(&mut rng));
//         let article_type = article_opt.unwrap_or(get_rand_article(&mut rng));
//         let wgs = [
//             noun.get_with_article(article_type, number),
//             get_rand_verb(&mut rng, &verb_cats).agreed(number),
//             get_rand_adj(&mut rng, &noun.is).agreed(noun.gender, number),
//         ];
//         wgs.iter().fold(WordGroup::new_empty(), |acc, wg| {
//             add_words(&acc, wg, acc.text != "")
//         })
//     })
// }

// pub fn get_with_article(article: Article, number: Number) -> Combination {
//     Box::new(move |noun| noun.get_with_article(article, number))
// }

// pub fn get_with_rand_article() -> Combination {
//     let mut rng = thread_rng();
//     let article = [Article::Definite, Article::Indefinite]
//         .choose(&mut rng)
//         .unwrap_or(&Article::Indefinite)
//         .clone();
//     let number = [Number::Singular, Number::Plural]
//         .choose(&mut rng)
//         .unwrap_or(&Number::Singular)
//         .clone();
//     get_with_article(article, number)
// }

// pub fn get_with_adjective(article: Article, number: Number) -> Combination {
//     Box::new(move |noun| {
//         let mut rng = thread_rng();
//         let adj = get_rand_adj(&mut rng, &noun.can_be).agreed(noun.gender, number);
//         let wg = noun.get_with_article(article, number);
//         add_words(&wg, &adj, true)
//     })
// }

// pub fn get_as_noun_complement(nouns: &'static StaticNouns) -> Combination {
//     Box::new(move |complement| {
//         let mut rng = thread_rng();
//         let rand_noun = complement
//             .can_emit
//             .choose(&mut rng)
//             .and_then(|id| NOUN_CATS.get(id))
//             .and_then(|cat| cat.nouns.choose(&mut rng))
//             .and_then(|id| nouns.iter().find(|item| &item.id == id))
//             .and_then(|noun| Some(noun.get_with_article(Article::Indefinite, Number::Singular)));
//         let noun = match rand_noun {
//             Some(group) => group,
//             None => WordGroup {
//                 text: String::from("#error#get_as_noun_complement#Can't find noun"),
//                 foots: (0, 0),
//             },
//         };
//         let apposition = get_apposition(&complement);

//         add_words(&noun, &apposition, true)
//     })
// }
