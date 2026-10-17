// Properties of `generate_haiku`, and of the blacklist contract of the
// sentence patterns it is built on.
//
// `generate_haiku` only returns three strings. To see which words a line is
// made of, each line is looked up in the exhaustive enumeration of the
// patterns (`enumeration.rs`): every producible text
// has exactly one decomposition into lemmas, see
// `every_producible_text_has_a_single_decomposition`.

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use crate::adj_enums::AdjId;
use crate::combination_data::get_combinations;
use crate::common_enums::{BlackLists, adjust_blacklist, merge_blacklists};
use crate::haiku::generate_haiku;
use crate::noun_enums::NounId;
use crate::string::uppercase_first_letter;
use crate::verb::get_verb_cat;
use crate::verb_enums::VerbCatId;

use super::enumeration::enumerate;
use super::labels::{adj_label, noun_label, verb_label};
use super::seam::draw;

const HAIKUS: usize = 3_000;
const FORM: [u8; 3] = [5, 7, 5];
/// (lines with a season adjective AND a season noun, those of them with 5 or 7 feet)
const TWO_SEASON_LINES: (usize, usize) = (864, 30);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Parsed {
    pattern: usize,
    feet: (u8, u8),
    nouns: Vec<&'static str>,
    adjs: Vec<&'static str>,
    verbs: Vec<&'static str>,
}

struct Index {
    /// producible text -> its decompositions (distinct)
    by_text: HashMap<String, Vec<Parsed>>,
    /// same, keyed by the text with its first letter uppercased
    by_capitalised: HashMap<String, Vec<Parsed>>,
}

fn index() -> &'static Index {
    static INDEX: OnceLock<Index> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut by_text: HashMap<String, Vec<Parsed>> = HashMap::new();
        for pattern in 0..5 {
            for outcome in enumerate(pattern) {
                let word = match outcome.word {
                    Some(word) => word,
                    None => continue,
                };
                let parsed = Parsed {
                    pattern,
                    feet: word.foots,
                    nouns: outcome.nouns,
                    adjs: outcome.adjs,
                    verbs: outcome.verbs,
                };
                let all = by_text.entry(word.text).or_default();
                if !all.contains(&parsed) {
                    all.push(parsed);
                }
            }
        }
        let mut by_capitalised: HashMap<String, Vec<Parsed>> = HashMap::new();
        for (text, parsed) in by_text.iter() {
            by_capitalised
                .entry(uppercase_first_letter(text))
                .or_default()
                .extend(parsed.iter().cloned());
        }
        Index {
            by_text,
            by_capitalised,
        }
    })
}

fn season_nouns() -> Vec<String> {
    [
        NounId::Printemps,
        NounId::Ete,
        NounId::Automne,
        NounId::Hiver,
    ]
    .iter()
    .map(|id| noun_label(*id))
    .collect()
}

fn season_adjs() -> Vec<String> {
    [
        AdjId::Printanier,
        AdjId::Estival,
        AdjId::Automnal,
        AdjId::Hivernal,
    ]
    .iter()
    .map(|id| adj_label(*id))
    .collect()
}

fn season_words(parsed: &Parsed) -> usize {
    let nouns = season_nouns();
    let adjs = season_adjs();
    parsed
        .nouns
        .iter()
        .filter(|n| nouns.iter().any(|s| s == *n))
        .count()
        + parsed
            .adjs
            .iter()
            .filter(|a| adjs.iter().any(|s| s == *a))
            .count()
}

#[test]
fn every_producible_text_has_a_single_decomposition() {
    let index = index();
    // 6599 + 6504 + 392 + 1496 + 54918 Ok lines of the five pattern goldens:
    // no text is produced by two patterns
    assert_eq!(index.by_text.len(), 69_909);
    assert_eq!(index.by_capitalised.len(), index.by_text.len());
    for (text, parsed) in index.by_text.iter() {
        assert_eq!(parsed.len(), 1, "{:?} -> {:?}", text, parsed);
    }
}

const NO_COMBINATION: &str = "#err#generate_haiku#no combination found";

/// One sample of haikus shared by all the property tests below.
fn haikus() -> &'static Vec<[String; 3]> {
    static SAMPLE: OnceLock<Vec<[String; 3]>> = OnceLock::new();
    SAMPLE.get_or_init(|| {
        let mut all = vec![];
        for _ in 0..HAIKUS {
            let mut combinations = get_combinations();
            match generate_haiku(&mut combinations) {
                Ok(lines) => all.push(lines),
                // cannot happen: the function always ends with `Ok(haiku)`
                Err(errs) => panic!("generate_haiku returned Err({:?})", errs),
            }
        }
        all
    })
}

/// The decomposition of each line; `None` for the NO_COMBINATION marker.
fn parse(haiku: &[String; 3]) -> Vec<Option<Parsed>> {
    let index = index();
    let mut all = vec![];
    for nb in 0..3 {
        if haiku[nb] == NO_COMBINATION {
            all.push(None);
            continue;
        }
        let found = if nb == 0 {
            index.by_capitalised.get(&haiku[nb])
        } else {
            index.by_text.get(&haiku[nb])
        };
        match found {
            Some(parsed) => all.push(Some(parsed[0].clone())),
            None => panic!(
                "line {} of {:?} is not something a sentence pattern can produce",
                nb, haiku
            ),
        }
    }
    all
}

fn words(parsed: &[Option<Parsed>]) -> Vec<&Parsed> {
    parsed.iter().filter_map(|line| line.as_ref()).collect()
}

fn duplicates(words: &[&'static str]) -> Vec<&'static str> {
    let mut seen: HashSet<&str> = HashSet::new();
    let mut twice = vec![];
    for word in words.iter() {
        if !seen.insert(*word) {
            twice.push(*word);
        }
    }
    twice
}

// `generate_haiku` cannot return Err at all (it always ends with `Ok(haiku)`),
// and the result is a `[String; 3]`. What it can put inside a line:
//
//   * `warn#generate_haiku#combination not found` and all the `err#get_rand_*`
//     / `err#get_with_*` messages are swallowed by the retry loop: never.
//   * `#error#get_with_some_article#...`, `#error#get_apposition#...`,
//     `error#Verb::agree#third group is not implemented yet` would come from
//     the grammar: unreachable with the current tables (see patterns.rs).
//   * `#err#generate_haiku#no combination found` REALLY HAPPENS, about once in
//     300 000 haikus (measured: 1 in 300 000), on line 1 or 2: the pattern
//     that fell on that line cannot reach the required number of feet with the
//     words the blacklists left. Seen:
//       ["Les mousses sont bleues", "un automne diapré",
//        "#err#generate_haiku#no combination found"]
//     It is far too rare to be asserted, so it is tolerated here: a line is
//     either that exact literal or a clean pattern line.
//     On line 0 the blacklists are empty and it would take 10001 failed tries
//     of a pattern that succeeds at least once in 300: never.
#[test]
fn haiku_has_three_clean_lines() {
    for haiku in haikus().iter() {
        assert_eq!(haiku.len(), 3);
        assert_ne!(haiku[0], NO_COMBINATION, "{:?}", haiku);
        for line in haiku.iter() {
            if line == NO_COMBINATION {
                continue;
            }
            assert!(!line.is_empty(), "{:?}", haiku);
            assert!(!line.contains('\n'), "{:?}", haiku);
            assert!(!line.contains('#'), "{:?}", haiku);
            assert!(!line.contains("error"), "{:?}", haiku);
            assert!(!line.contains("warn"), "{:?}", haiku);
            assert_eq!(line.trim(), line, "{:?}", haiku);
            assert!(!line.contains("  "), "{:?}", haiku);
        }
    }
}

#[test]
fn only_the_first_line_is_capitalised() {
    for haiku in haikus().iter() {
        let first = |nb: usize| haiku[nb].chars().next().unwrap();
        assert!(first(0).is_uppercase(), "{:?}", haiku);
        for nb in 1..3 {
            assert!(
                first(nb).is_lowercase() || haiku[nb] == NO_COMBINATION,
                "{:?}",
                haiku
            );
        }
        // and nothing but the first letter is uppercase
        for line in haiku.iter() {
            assert!(
                line.chars().skip(1).all(|c| !c.is_uppercase()),
                "{:?}",
                haiku
            );
        }
    }
}

#[test]
fn every_line_is_a_pattern_line_with_the_right_feet() {
    for haiku in haikus().iter() {
        let parsed = parse(haiku);
        for nb in 0..3 {
            if let Some(line) = &parsed[nb] {
                let (min, max) = line.feet;
                assert!(
                    min <= FORM[nb] && FORM[nb] <= max,
                    "line {} of {:?} has feet ({},{})",
                    nb,
                    haiku,
                    min,
                    max
                );
            }
        }
    }
}

// Nouns and adjectives are never used twice in a haiku.
//
// Verbs CAN be (measured: 82 haikus in 300 000), e.g.
//   ["L'automne est brûlant", "le rougegorge s'éclipse", "les soleils s'éclipsent"]
// It takes two things: the reshuffle after 1000 failed tries putting the
// intransitive-verb pattern on a second line (see
// `the_same_pattern_can_serve_two_lines`), and that pattern ignoring the verb
// blacklist (see `intransitive_pattern_ignores_the_verb_blacklist`). So the
// true rule is: a repeated verb is always between two intransitive-verb lines.
#[test]
fn no_noun_or_adjective_is_used_twice_and_verbs_almost_never() {
    for haiku in haikus().iter() {
        let parsed = parse(haiku);
        let mut nouns = vec![];
        let mut adjs = vec![];
        let mut verbs = vec![];
        let mut verbs_outside_intransitive = vec![];
        let mut intransitive_lines = 0;
        for line in words(&parsed) {
            nouns.extend(line.nouns.iter().cloned());
            adjs.extend(line.adjs.iter().cloned());
            verbs.extend(line.verbs.iter().cloned());
            if line.pattern == 2 {
                intransitive_lines += 1;
            } else {
                verbs_outside_intransitive.extend(line.verbs.iter().cloned());
            }
        }
        assert!(
            duplicates(&nouns).is_empty(),
            "{:?} nouns {:?}",
            haiku,
            nouns
        );
        assert!(
            duplicates(&adjs).is_empty(),
            "{:?} adjectives {:?}",
            haiku,
            adjs
        );
        assert!(
            duplicates(&verbs_outside_intransitive).is_empty(),
            "{:?} verbs {:?}",
            haiku,
            verbs
        );
        if !duplicates(&verbs).is_empty() {
            assert!(intransitive_lines >= 2, "{:?} verbs {:?}", haiku, verbs);
        }
    }
}

// Why the reshuffle of `generate_haiku` (after 1000 failed tries of a line)
// is not a theoretical branch: with empty blacklists, one try of the
// adjective_and_affiliation pattern has 5 feet with probability 0.56 % only,
// so when the shuffle puts it on line 0 or 2, 1000 tries all fail about once
// in 270. After the reshuffle the three lines no longer come from three
// different patterns, e.g.
//   ["Le rayon est brun", "les rougegorges moirés", "le renard immense"]
// There is therefore NO test that the three lines use three patterns.
// (Before "raie", a one-foot noun, could be drawn: 0.38 %, once in 45, and
// 3096 such haikus measured in 300 000.)
#[test]
fn the_same_pattern_can_serve_two_lines() {
    // (lines with 5 feet, lines with 7 feet) among the Ok lines of each golden
    let expected: [(usize, usize); 5] =
        [(2270, 941), (399, 1991), (144, 58), (523, 254), (261, 8870)];
    let mut counts = [(0, 0); 5];
    for parsed in index().by_text.values() {
        let line = &parsed[0];
        if line.feet.0 <= 5 && 5 <= line.feet.1 {
            counts[line.pattern].0 += 1;
        }
        if line.feet.0 <= 7 && 7 <= line.feet.1 {
            counts[line.pattern].1 += 1;
        }
    }
    assert_eq!(counts, expected);

    let five_feet: f64 = enumerate(4)
        .iter()
        .filter(|outcome| match &outcome.word {
            Some(word) => word.foots.0 <= 5 && 5 <= word.foots.1,
            None => false,
        })
        .map(|outcome| outcome.prob)
        .sum();
    assert!(0.0055 < five_feet && five_feet < 0.0057, "{}", five_feet);
}

// "A haiku should contain only one mention of a season" (common_enums.rs) is
// enforced BETWEEN lines only: once a line has a season word, the other lines
// cannot have one. Inside one line, the adjective_and_affiliation pattern can
// put a season adjective next to a season noun (measured: 30 haikus in
// 300 000), see `one_line_can_hold_two_season_words`.
#[test]
fn at_most_one_line_mentions_a_season() {
    for haiku in haikus().iter() {
        let parsed = parse(haiku);
        let counts: Vec<usize> = words(&parsed).into_iter().map(season_words).collect();
        let lines_with_season = counts.iter().filter(|count| **count > 0).count();
        assert!(lines_with_season <= 1, "{:?} {:?}", haiku, counts);
        assert!(
            counts.iter().sum::<usize>() <= 2,
            "{:?} {:?}",
            haiku,
            counts
        );
    }
}

// Seen in real haikus:
//   ["La mousse est orange", "soleil hivernal d'été", "un parfum d'aurore"]
//   ["Le merle se couche", "soleils estivaux d'hiver", "le prunier est libre"]
#[test]
fn one_line_can_hold_two_season_words() {
    let index = index();
    let line = &index.by_text["soleil hivernal d'été"][0];
    assert_eq!(line.pattern, 4);
    assert_eq!(line.feet, (7, 7));
    assert_eq!(season_words(line), 2);

    let mut two = 0;
    let mut two_that_fit = 0;
    for parsed in index.by_text.values() {
        let line = &parsed[0];
        assert!(season_words(line) <= 2);
        if season_words(line) == 2 {
            assert_eq!(line.pattern, 4);
            two += 1;
            let fits = |feet: u8| line.feet.0 <= feet && feet <= line.feet.1;
            if fits(5) || fits(7) {
                two_that_fit += 1;
            }
        }
    }
    assert_eq!((two, two_that_fit), TWO_SEASON_LINES);
}

#[test]
fn midi_and_minuit_never_meet() {
    let midi = noun_label(NounId::Midi);
    let minuit = noun_label(NounId::Minuit);
    for haiku in haikus().iter() {
        let parsed = parse(haiku);
        let count = words(&parsed)
            .iter()
            .flat_map(|line| line.nouns.iter())
            .filter(|noun| **noun == midi || **noun == minuit)
            .count();
        assert!(count <= 1, "{:?}", haiku);
    }
}

// ------------------------------------------------- blacklist contract, per pattern

fn disjoint<T: std::hash::Hash + Eq>(a: &HashSet<T>, b: &HashSet<T>) -> bool {
    a.intersection(b).next().is_none()
}

// What `generate_haiku` relies on, observed where it is observable: a pattern
// never returns a noun or an adjective that is in the blacklists it was given.
// Verbs: true for the linking-verb pattern only, see the next test.
#[test]
fn patterns_respect_the_noun_and_adjective_blacklists() {
    let patterns = get_combinations();
    for round in 0..600 {
        let mut blacklists = BlackLists::new_empty();
        // three different patterns, every ordered triple in turn
        let order = [round % 5, (round % 5 + 1 + (round / 5) % 4) % 5];
        let third = (0..5)
            .filter(|i| *i != order[0] && *i != order[1])
            .nth((round / 20) % 3)
            .unwrap();
        for index in [order[0], order[1], third].iter() {
            let mut result = None;
            for _ in 0..10_000 {
                if let Ok(ok) = draw(&patterns[*index], &blacklists) {
                    result = Some(ok);
                    break;
                }
            }
            let (word, new) = result.expect("no Ok in 10000 draws");
            assert!(disjoint(&new.nouns, &blacklists.nouns), "{:?}", word);
            assert!(disjoint(&new.adjs, &blacklists.adjs), "{:?}", word);
            if *index == 1 {
                assert!(disjoint(&new.verbs, &blacklists.verbs), "{:?}", word);
            }
            blacklists = adjust_blacklist(merge_blacklists(&blacklists, &new));
        }
    }
}

// The blacklists returned by a pattern are exactly the words of the line.
#[test]
fn returned_blacklists_are_the_words_of_the_line() {
    let patterns = get_combinations();
    let index = index();
    let empty = BlackLists::new_empty();
    let sorted = |mut labels: Vec<String>| {
        labels.sort();
        labels
    };
    for (pattern, closure) in patterns.iter().enumerate() {
        for _ in 0..2_000 {
            if let Ok((word, new)) = draw(closure, &empty) {
                let parsed = &index.by_text[&word.text][0];
                assert_eq!(parsed.pattern, pattern);
                assert_eq!(
                    sorted(new.nouns.iter().map(|id| noun_label(*id)).collect()),
                    sorted(parsed.nouns.iter().map(|s| String::from(*s)).collect()),
                    "{:?}",
                    word
                );
                assert_eq!(
                    sorted(new.adjs.iter().map(|id| adj_label(*id)).collect()),
                    sorted(parsed.adjs.iter().map(|s| String::from(*s)).collect()),
                    "{:?}",
                    word
                );
                assert_eq!(
                    sorted(new.verbs.iter().map(|id| verb_label(*id)).collect()),
                    sorted(parsed.verbs.iter().map(|s| String::from(*s)).collect()),
                    "{:?}",
                    word
                );
            }
        }
    }
}

// CURRENT BEHAVIOUR (looks like an oversight): get_with_intransitive_verb
// picks its verb with a bare `choose`, without looking at `blacklists.verbs`.
// It is not visible in a haiku because the pattern is used once per haiku and
// shares no verb with the linking-verb pattern.
#[test]
fn intransitive_pattern_ignores_the_verb_blacklist() {
    let patterns = get_combinations();
    let mut blacklists = BlackLists::new_empty();
    for verb in get_verb_cat(&VerbCatId::EtatDEveil).unwrap() {
        blacklists.verbs.insert(verb);
    }
    for verb in get_verb_cat(&VerbCatId::Etat).unwrap() {
        blacklists.verbs.insert(verb);
    }
    for _ in 0..500 {
        // no noun is blacklisted and every category has verbs: always Ok
        let (word, new) = draw(&patterns[2], &blacklists).expect("always Ok");
        assert_eq!(new.verbs.len(), 1, "{:?}", word);
        assert!(new.verbs.is_subset(&blacklists.verbs), "{:?}", word);
    }
}

// ... whereas the linking-verb pattern honours it.
#[test]
fn linking_pattern_fails_when_every_linking_verb_is_blacklisted() {
    let patterns = get_combinations();
    let mut blacklists = BlackLists::new_empty();
    for verb in get_verb_cat(&VerbCatId::Etat).unwrap() {
        blacklists.verbs.insert(verb);
    }
    for _ in 0..500 {
        match draw(&patterns[1], &blacklists) {
            Ok((word, _)) => panic!("unexpected Ok({:?})", word),
            Err(errs) => {
                assert_eq!(errs.len(), 1);
                assert!(
                    errs[0] == "err#get_rand_verb#Verb not found"
                        || errs[0] == "err#get_rand_adj#Adjective not found",
                    "{:?}",
                    errs
                );
            }
        }
    }
}
