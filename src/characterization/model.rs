// Exhaustive enumeration of what each sentence pattern can produce when it is
// called with EMPTY blacklists.
//
// The five patterns of `combination.rs` draw from a hard-coded `ThreadRng`, so
// the real code can only be sampled. This file walks the same decision tree by
// hand (every `choose` becomes a loop) and calls the real grammar functions at
// the leaves. It is a second, independent description of the control flow of
// `combination.rs`; the Monte-Carlo tests of `patterns.rs` check that the real
// closures produce exactly this set.
//
// It also knows two things a sample does not give: the probability of every
// outcome (used to size the Monte-Carlo runs) and which lemmas a line is made
// of (used by the haiku properties).
//
// Once the generator is behind a crate-local seam (part B), this file should
// be replaced by an odometer over the real closures.

use std::collections::BTreeMap;

use crate::adj::Adj;
use crate::adj_data::ADJ_CATS;
use crate::adj_enums::AdjId;
use crate::common_enums::{Article, Number};
use crate::noun::{
    get_apposition, get_cats_containing_affiliations, get_cats_containing_attributes,
    get_cats_containing_epithets, get_cats_containing_epithets_and_affiliations,
    get_cats_containing_int_verbs, Noun,
};
use crate::noun_data::NOUN_CATS;
use crate::noun_enums::NounId;
use crate::string::take_last_graphemes;
use crate::verb::{get_verb_cat, Verb};
use crate::verb_enums::{VerbCatId, VerbId, VerbKind};
use crate::wordgroup::{fold_wordgroups, WordGroup};

use super::grammar::{ARTICLES, NUMBERS};
use super::labels::{find_adj, find_noun, find_verb};

/// Same order as `combination_data::get_combinations()`.
pub const PATTERN_NAMES: [&str; 5] = [
    "adjective",
    "linking_verb",
    "intransitive_verb",
    "affiliation",
    "adjective_and_affiliation",
];

const ERR_NOUN_CAT: &str = "err#get_rand_cat_noun#Category not found";
const ERR_ADJ_CAT: &str = "err#get_rand_adj_cat#Category not found";
const ERR_NOUN: &str = "err#get_rand_noun#Noun not found";
const ERR_ADJ: &str = "err#get_rand_adj#Adjective not found";
const ERR_VERB: &str = "err#get_rand_verb#Verb not found";
const ERR_NO_INT_CAT: &str = "err#get_with_intransitive_verb#No category with intransitive verb";
const ERR_NO_INT_VERB: &str = "err#get_with_intransitive_verb#No intransitive verb for this noun";

pub struct Outcome {
    /// The golden line: `text | min max`, or `#ERR# first + second`.
    pub line: String,
    /// `None` when the pattern returns `Err`.
    pub word: Option<WordGroup>,
    // lemmas the line is made of
    pub nouns: Vec<&'static str>,
    pub adjs: Vec<&'static str>,
    pub verbs: Vec<&'static str>,
    /// Probability of this path of the decision tree.
    pub prob: f64,
}

pub fn ok_line(word: &WordGroup) -> String {
    format!("{} | {} {}", word.text, word.foots.0, word.foots.1)
}

pub fn err_line(errs: &[String]) -> String {
    format!("#ERR# {}", errs.join(" + "))
}

struct Sink {
    outcomes: Vec<Outcome>,
}

impl Sink {
    fn ok(
        &mut self,
        prob: f64,
        word: WordGroup,
        nouns: &[&'static Noun],
        adjs: &[&'static Adj],
        verbs: &[&'static Verb],
    ) {
        self.outcomes.push(Outcome {
            line: ok_line(&word),
            word: Some(word),
            nouns: nouns.iter().map(|n| (*n).word.text.as_str()).collect(),
            adjs: adjs.iter().map(|a| (*a).word.text.as_str()).collect(),
            verbs: verbs.iter().map(|v| (*v).word.text.as_str()).collect(),
            prob,
        });
    }

    fn err(&mut self, prob: f64, errs: &[String]) {
        self.outcomes.push(Outcome {
            line: err_line(errs),
            word: None,
            nouns: vec![],
            adjs: vec![],
            verbs: vec![],
            prob,
        });
    }

    fn err1(&mut self, prob: f64, err: &str) {
        self.err(prob, &[String::from(err)]);
    }
}

fn share(prob: f64, len: usize) -> f64 {
    prob / (len as f64)
}

// `candidates.choose(rng)` followed by a lookup in the table: one entry per
// equiprobable result, `None` standing for the "not found" error (unknown id,
// or nothing to choose from).
fn noun_choices(ids: &[NounId], excluded: Option<NounId>) -> Vec<Option<&'static Noun>> {
    let candidates: Vec<NounId> = ids
        .iter()
        .filter(|id| Some(**id) != excluded)
        .cloned()
        .collect();
    if candidates.is_empty() {
        return vec![None];
    }
    candidates.iter().map(|id| find_noun(*id)).collect()
}

fn adj_choices(ids: &[AdjId]) -> Vec<Option<&'static Adj>> {
    if ids.is_empty() {
        return vec![None];
    }
    ids.iter().map(|id| find_adj(*id)).collect()
}

fn verb_choices(ids: &[VerbId]) -> Vec<Option<&'static Verb>> {
    if ids.is_empty() {
        return vec![None];
    }
    ids.iter().map(|id| find_verb(*id)).collect()
}

/// (probability, number, article) for the two draws every pattern starts with.
fn number_and_article(articles: &[Article]) -> Vec<(f64, Number, Article)> {
    let mut all = vec![];
    for number in NUMBERS.iter() {
        for article in articles.iter() {
            all.push((
                share(share(1.0, NUMBERS.len()), articles.len()),
                *number,
                *article,
            ));
        }
    }
    all
}

// get_with_adjective(None, None)
fn adjective(sink: &mut Sink) {
    for (p0, number, article) in number_and_article(&ARTICLES) {
        let cats = get_cats_containing_epithets();
        if cats.is_empty() {
            sink.err1(p0, ERR_NOUN_CAT);
            continue;
        }
        for cat_id in cats.iter() {
            let p1 = share(p0, cats.len());
            let cat = &NOUN_CATS[cat_id];
            for adj_cat_id in cat.rel.epithets.iter() {
                let p2 = share(p1, cat.rel.epithets.len());
                let adj_ids = match ADJ_CATS.get(adj_cat_id) {
                    Some(ids) => ids,
                    None => {
                        sink.err1(p2, ERR_ADJ_CAT);
                        continue;
                    }
                };
                let nouns = noun_choices(&cat.nouns, None);
                for noun in nouns.iter() {
                    let p3 = share(p2, nouns.len());
                    let noun = match noun {
                        Some(noun) => *noun,
                        None => {
                            sink.err1(p3, ERR_NOUN);
                            continue;
                        }
                    };
                    let adjs = adj_choices(adj_ids);
                    for adj in adjs.iter() {
                        let p4 = share(p3, adjs.len());
                        let adj = match adj {
                            Some(adj) => *adj,
                            None => {
                                sink.err1(p4, ERR_ADJ);
                                continue;
                            }
                        };
                        let word = fold_wordgroups(vec![
                            &noun.get_with_article(article, number),
                            &adj.agreed(noun.gender, number),
                        ]);
                        sink.ok(p4, word, &[noun], &[adj], &[]);
                    }
                }
            }
        }
    }
}

// get_with_linking_verb(None, Some(vec![Article::Definite]))
fn linking_verb(sink: &mut Sink) {
    for (p0, number, article) in number_and_article(&[Article::Definite]) {
        let mut early_errs: Vec<String> = vec![];
        let verb_ids = match get_verb_cat(&VerbCatId::Etat) {
            Ok(ids) => ids,
            Err(err) => {
                early_errs.push(err);
                vec![]
            }
        };
        let cats = get_cats_containing_attributes();
        if cats.is_empty() {
            early_errs.push(String::from(ERR_NOUN_CAT));
            sink.err(p0, &early_errs);
            continue;
        }
        for cat_id in cats.iter() {
            let p1 = share(p0, cats.len());
            let cat = &NOUN_CATS[cat_id];
            for adj_cat_id in cat.rel.attributes.iter() {
                let p2 = share(p1, cat.rel.attributes.len());
                let mut errs = early_errs.clone();
                let adj_ids = match ADJ_CATS.get(adj_cat_id) {
                    Some(ids) => ids.clone(),
                    None => {
                        errs.push(String::from(ERR_ADJ_CAT));
                        vec![]
                    }
                };
                if !errs.is_empty() {
                    sink.err(p2, &errs);
                    continue;
                }
                let nouns = noun_choices(&cat.nouns, None);
                for noun in nouns.iter() {
                    let p3 = share(p2, nouns.len());
                    let noun = match noun {
                        Some(noun) => *noun,
                        None => {
                            sink.err1(p3, ERR_NOUN);
                            continue;
                        }
                    };
                    let adjs = adj_choices(&adj_ids);
                    for adj in adjs.iter() {
                        let p4 = share(p3, adjs.len());
                        let adj = match adj {
                            Some(adj) => *adj,
                            None => {
                                sink.err1(p4, ERR_ADJ);
                                continue;
                            }
                        };
                        let verbs = verb_choices(&verb_ids);
                        for verb in verbs.iter() {
                            let p5 = share(p4, verbs.len());
                            let verb = match verb {
                                Some(verb) => *verb,
                                None => {
                                    sink.err1(p5, ERR_VERB);
                                    continue;
                                }
                            };
                            let agreed = verb.agree(&number);
                            // "plural verb needs a foot update"
                            let bonus = if take_last_graphemes(&agreed.text, 3) == "ent" {
                                1
                            } else {
                                0
                            };
                            let final_verb = WordGroup {
                                text: agreed.text.clone(),
                                foots: (agreed.foots.0 + bonus, agreed.foots.1 + bonus),
                            };
                            let word = fold_wordgroups(vec![
                                &noun.get_with_article(article, number),
                                &final_verb,
                                &adj.agreed(noun.gender, number),
                            ]);
                            sink.ok(p5, word, &[noun], &[adj], &[verb]);
                        }
                    }
                }
            }
        }
    }
}

// get_with_intransitive_verb(None, Some(vec![Article::Definite]))
fn intransitive_verb(sink: &mut Sink) {
    for (p0, number, article) in number_and_article(&[Article::Definite]) {
        let cats = get_cats_containing_int_verbs();
        if cats.is_empty() {
            sink.err1(p0, ERR_NO_INT_CAT);
            continue;
        }
        for cat_id in cats.iter() {
            let p1 = share(p0, cats.len());
            let cat = &NOUN_CATS[cat_id];
            let nouns = noun_choices(&cat.nouns, None);
            for noun in nouns.iter() {
                let p2 = share(p1, nouns.len());
                let noun = match noun {
                    Some(noun) => *noun,
                    None => {
                        sink.err1(p2, ERR_NOUN);
                        continue;
                    }
                };
                let mut errs: Vec<String> = vec![];
                let mut verb_ids: Vec<VerbId> = vec![];
                for verb_cat_id in cat.rel.functions.iter() {
                    match get_verb_cat(verb_cat_id) {
                        Ok(ids) => {
                            for id in ids.iter() {
                                let intransitive = match find_verb(*id) {
                                    Some(verb) => verb.kind == VerbKind::Intransitive,
                                    None => false,
                                };
                                if intransitive {
                                    verb_ids.push(*id);
                                }
                            }
                        }
                        Err(err) => errs.push(err),
                    }
                }
                if verb_ids.is_empty() {
                    errs.push(String::from(ERR_NO_INT_VERB));
                    sink.err(p2, &errs);
                    continue;
                }
                for verb_id in verb_ids.iter() {
                    let p3 = share(p2, verb_ids.len());
                    let verb = find_verb(*verb_id).expect("filtered on the verb table just above");
                    let word = fold_wordgroups(vec![
                        &noun.get_with_article(article, number),
                        &verb.agree(&number),
                    ]);
                    sink.ok(p3, word, &[noun], &[], &[verb]);
                }
            }
        }
    }
}

// get_with_affiliation(None, None)
fn affiliation(sink: &mut Sink) {
    for (p0, number, article) in number_and_article(&ARTICLES) {
        let cats = get_cats_containing_affiliations();
        if cats.is_empty() {
            sink.err1(p0, ERR_NOUN_CAT);
            continue;
        }
        for cat_id in cats.iter() {
            let p1 = share(p0, cats.len());
            let cat = &NOUN_CATS[cat_id];
            for aff_cat_id in cat.rel.affiliations.iter() {
                let p2 = share(p1, cat.rel.affiliations.len());
                let aff_ids = match NOUN_CATS.get(aff_cat_id) {
                    Some(aff_cat) => &aff_cat.nouns,
                    None => {
                        sink.err1(p2, ERR_NOUN_CAT);
                        continue;
                    }
                };
                let nouns = noun_choices(&cat.nouns, None);
                for noun in nouns.iter() {
                    let p3 = share(p2, nouns.len());
                    let noun = match noun {
                        Some(noun) => *noun,
                        None => {
                            sink.err1(p3, ERR_NOUN);
                            continue;
                        }
                    };
                    // the head noun is NOT excluded here (unlike in
                    // adjective_and_affiliation)
                    let affs = noun_choices(aff_ids, None);
                    for aff in affs.iter() {
                        let p4 = share(p3, affs.len());
                        let aff = match aff {
                            Some(aff) => *aff,
                            None => {
                                sink.err1(p4, ERR_NOUN);
                                continue;
                            }
                        };
                        let word = fold_wordgroups(vec![
                            &noun.get_article(number, article),
                            &noun.agreed(number),
                            &get_apposition(aff, Article::None),
                        ]);
                        sink.ok(p4, word, &[noun, aff], &[], &[]);
                    }
                }
            }
        }
    }
}

// get_with_adj_and_affiliation(None, None)
fn adjective_and_affiliation(sink: &mut Sink) {
    for (p0, number, article) in number_and_article(&ARTICLES) {
        let cats = get_cats_containing_epithets_and_affiliations();
        if cats.is_empty() {
            sink.err1(p0, ERR_NOUN_CAT);
            continue;
        }
        for cat_id in cats.iter() {
            let p1 = share(p0, cats.len());
            let cat = &NOUN_CATS[cat_id];
            for aff_cat_id in cat.rel.affiliations.iter() {
                let p2 = share(p1, cat.rel.affiliations.len());
                for adj_cat_id in cat.rel.epithets.iter() {
                    let p3 = share(p2, cat.rel.epithets.len());
                    let mut errs: Vec<String> = vec![];
                    let aff_ids = match NOUN_CATS.get(aff_cat_id) {
                        Some(aff_cat) => aff_cat.nouns.clone(),
                        None => {
                            errs.push(String::from(ERR_NOUN_CAT));
                            vec![]
                        }
                    };
                    let adj_ids = match ADJ_CATS.get(adj_cat_id) {
                        Some(ids) => ids.clone(),
                        None => {
                            errs.push(String::from(ERR_ADJ_CAT));
                            vec![]
                        }
                    };
                    if !errs.is_empty() {
                        sink.err(p3, &errs);
                        continue;
                    }
                    let nouns = noun_choices(&cat.nouns, None);
                    for noun in nouns.iter() {
                        let p4 = share(p3, nouns.len());
                        let noun = match noun {
                            Some(noun) => *noun,
                            None => {
                                sink.err1(p4, ERR_NOUN);
                                continue;
                            }
                        };
                        let affs = noun_choices(&aff_ids, Some(noun.id));
                        for aff in affs.iter() {
                            let p5 = share(p4, affs.len());
                            let aff = match aff {
                                Some(aff) => *aff,
                                None => {
                                    sink.err1(p5, ERR_NOUN);
                                    continue;
                                }
                            };
                            let adjs = adj_choices(&adj_ids);
                            for adj in adjs.iter() {
                                let p6 = share(p5, adjs.len());
                                let adj = match adj {
                                    Some(adj) => *adj,
                                    None => {
                                        sink.err1(p6, ERR_ADJ);
                                        continue;
                                    }
                                };
                                for aff_article in ARTICLES.iter() {
                                    let p7 = share(p6, ARTICLES.len());
                                    let word = fold_wordgroups(vec![
                                        &noun.get_with_article(article, number),
                                        &adj.agreed(noun.gender, number),
                                        &get_apposition(aff, *aff_article),
                                    ]);
                                    sink.ok(p7, word, &[noun, aff], &[adj], &[]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Every path of the decision tree of pattern `index` (see PATTERN_NAMES).
pub fn enumerate(index: usize) -> Vec<Outcome> {
    let mut sink = Sink { outcomes: vec![] };
    match index {
        0 => adjective(&mut sink),
        1 => linking_verb(&mut sink),
        2 => intransitive_verb(&mut sink),
        3 => affiliation(&mut sink),
        4 => adjective_and_affiliation(&mut sink),
        _ => panic!("there are five patterns"),
    }
    sink.outcomes
}

/// Distinct golden lines with their total probability, sorted.
pub fn distribution(outcomes: &[Outcome]) -> BTreeMap<String, f64> {
    let mut lines: BTreeMap<String, f64> = BTreeMap::new();
    for outcome in outcomes.iter() {
        *lines.entry(outcome.line.clone()).or_insert(0.0) += outcome.prob;
    }
    lines
}
