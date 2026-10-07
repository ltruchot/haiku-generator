// The five sentence patterns, called with empty blacklists.
//
// For each pattern the golden `tests/golden/pattern_<n>_<name>.txt` is the
// sorted set of everything the pattern can return:
//
//     <text> | <feet min> <feet max>          for Ok
//     #ERR# <message> [+ <message>...]        for Err
//
// Three tests look at that set from two independent sides:
//
//   * enumeration (always on): the hand-written decision tree of `model.rs`
//     must produce exactly the golden. With UPDATE_GOLDEN=1 this is the test
//     that rewrites the file.
//   * Monte-Carlo, quick (always on): a few thousand draws of the REAL closure,
//     each of which must belong to the golden.
//   * Monte-Carlo, full (release builds only, #[ignore]d in debug where it
//     is ten times slower): draws the REAL closure on every core until nothing
//     new has shown up for a long stretch, and requires the set of everything
//     seen to be EQUAL to the golden. About 17 million draws in total.
//
//       cargo test --release -- monte_carlo_full --nocapture     (prints the figures)
//       cargo test -- --ignored monte_carlo_full                 (debug, slow)
//
// Seeded goldens are not an option: the category tables are HashMaps, so the
// same seed walks a different path in every process.

use std::collections::{BTreeSet, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::combination_data::get_combinations;
use crate::common_enums::BlackLists;

use super::golden::{assert_golden_lines, read_golden_lines, updating};
use super::grammar::graphemes_differ_from_chars;
use super::model::{distribution, enumerate, err_line, ok_line, PATTERN_NAMES};
use super::seam::draw;

pub fn golden_name(index: usize) -> String {
    format!("pattern_{}_{}.txt", index, PATTERN_NAMES[index])
}

/// One draw of the real pattern `index`, as a golden line.
fn draw_line(pattern: &crate::combination::Combination, empty: &BlackLists) -> String {
    match draw(pattern, empty) {
        Ok((word, _)) => ok_line(&word),
        Err(errs) => err_line(&errs),
    }
}

// ---------------------------------------------------------------- enumeration

fn enumeration_matches_golden(index: usize) {
    let lines: Vec<String> = distribution(&enumerate(index)).keys().cloned().collect();
    assert_golden_lines(&golden_name(index), &lines);
}

#[test]
fn enumeration_0_adjective() {
    enumeration_matches_golden(0);
}

#[test]
fn enumeration_1_linking_verb() {
    enumeration_matches_golden(1);
}

#[test]
fn enumeration_2_intransitive_verb() {
    enumeration_matches_golden(2);
}

#[test]
fn enumeration_3_affiliation() {
    enumeration_matches_golden(3);
}

#[test]
fn enumeration_4_adjective_and_affiliation() {
    enumeration_matches_golden(4);
}

#[test]
fn enumerated_probabilities_sum_to_one() {
    for index in 0..5 {
        let total: f64 = enumerate(index).iter().map(|outcome| outcome.prob).sum();
        assert!(
            (total - 1.0).abs() < 1e-9,
            "pattern {}: probabilities sum to {}",
            index,
            total
        );
    }
}

// Error markers a WordGroup text could carry (see noun.rs and verb.rs):
// `#error#get_with_some_article#...`, `#error#get_apposition#...` and
// `error#Verb::agree#third group is not implemented yet`. None is reachable
// with the current tables, and no producible text contains a `#` at all.
#[test]
fn no_producible_text_carries_an_error_marker() {
    for index in 0..5 {
        for outcome in enumerate(index).iter() {
            if let Some(word) = &outcome.word {
                assert!(
                    !word.text.contains('#') && !word.text.contains("error"),
                    "pattern {}: {:?}",
                    index,
                    word.text
                );
                assert!(!word.text.is_empty());
                assert!(word.foots.0 <= word.foots.1, "{:?}", word);
            }
        }
    }
}

#[test]
fn every_producible_text_has_one_char_per_grapheme() {
    for index in 0..5 {
        for outcome in enumerate(index).iter() {
            assert!(
                !graphemes_differ_from_chars(&outcome.line),
                "pattern {}: {:?}",
                index,
                outcome.line
            );
        }
    }
}

// ---------------------------------------------------------------- Monte-Carlo

const QUICK_DRAWS: usize = 20_000;

fn monte_carlo_quick(index: usize) {
    if updating() {
        // the golden is being rewritten by the enumeration test
        return;
    }
    let golden: HashSet<String> = read_golden_lines(&golden_name(index)).into_iter().collect();
    let patterns = get_combinations();
    let empty = BlackLists::new_empty();
    let mut seen: HashSet<String> = HashSet::new();
    for _ in 0..QUICK_DRAWS {
        let line = draw_line(&patterns[index], &empty);
        assert!(
            golden.contains(&line),
            "pattern {} produced a line that is not in {}: {:?}",
            index,
            golden_name(index),
            line
        );
        seen.insert(line);
    }
    // sanity: the sample is not degenerate
    assert!(seen.len() > 50, "only {} distinct lines", seen.len());
}

#[test]
fn monte_carlo_quick_0_adjective() {
    monte_carlo_quick(0);
}

#[test]
fn monte_carlo_quick_1_linking_verb() {
    monte_carlo_quick(1);
}

#[test]
fn monte_carlo_quick_2_intransitive_verb() {
    monte_carlo_quick(2);
}

#[test]
fn monte_carlo_quick_3_affiliation() {
    monte_carlo_quick(3);
}

#[test]
fn monte_carlo_quick_4_adjective_and_affiliation() {
    monte_carlo_quick(4);
}

// Stop rule of the full run: no new line during `STRETCH_FACTOR / p_min`
// consecutive draws, where p_min is the probability of the rarest line of the
// pattern (known from the enumeration). A line still missing at the start of
// such a stretch stays missing with probability at most exp(-STRETCH_FACTOR)
// = 2e-9; summed over the largest set (125 000 lines) that is still below
// 3e-4, and the comparison with the golden would catch it anyway.
const STRETCH_FACTOR: f64 = 20.0;
const BATCH: u64 = 20_000;

struct Shared {
    seen: BTreeSet<String>,
    draws: u64,
    last_new_at: u64,
    done: bool,
}

fn monte_carlo_full(index: usize) {
    if updating() {
        // the golden is being rewritten by the enumeration test
        return;
    }
    let golden: BTreeSet<String> = read_golden_lines(&golden_name(index)).into_iter().collect();
    let dist = distribution(&enumerate(index));
    let p_min = dist.values().cloned().fold(1.0_f64, f64::min);
    let stretch = (STRETCH_FACTOR / p_min).ceil() as u64;
    let threads = match std::thread::available_parallelism() {
        Ok(n) => n.get(),
        Err(_) => 4,
    };
    let shared = Arc::new(Mutex::new(Shared {
        seen: BTreeSet::new(),
        draws: 0,
        last_new_at: 0,
        done: false,
    }));
    let start = Instant::now();
    let mut handles = vec![];
    for _ in 0..threads {
        let shared = Arc::clone(&shared);
        handles.push(std::thread::spawn(move || {
            // the closures are not Send: each thread builds its own
            let patterns = get_combinations();
            let empty = BlackLists::new_empty();
            let mut mine: HashSet<String> = HashSet::new();
            loop {
                let mut fresh: Vec<String> = vec![];
                for _ in 0..BATCH {
                    let line = draw_line(&patterns[index], &empty);
                    if !mine.contains(&line) {
                        mine.insert(line.clone());
                        fresh.push(line);
                    }
                }
                let mut state = shared.lock().unwrap();
                if state.done {
                    return;
                }
                state.draws += BATCH;
                for line in fresh {
                    if state.seen.insert(line) {
                        state.last_new_at = state.draws;
                    }
                }
                if state.draws - state.last_new_at >= stretch {
                    state.done = true;
                    return;
                }
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let state = shared.lock().unwrap();
    let missing: Vec<&String> = golden.difference(&state.seen).collect();
    let extra: Vec<&String> = state.seen.difference(&golden).collect();
    eprintln!(
        "monte_carlo_full pattern {} ({}): golden {} lines, seen {} lines, rarest line p = 1/{:.0}, stretch {} draws, {} draws in total, last new line at draw {}, {} threads, {:.1}s",
        index,
        PATTERN_NAMES[index],
        golden.len(),
        state.seen.len(),
        1.0 / p_min,
        stretch,
        state.draws,
        state.last_new_at,
        threads,
        start.elapsed().as_secs_f64()
    );
    assert!(
        missing.is_empty() && extra.is_empty(),
        "pattern {}: {} golden lines never drawn (first: {:?}), {} drawn lines not in the golden (first: {:?})",
        index,
        missing.len(),
        missing.first(),
        extra.len(),
        extra.first()
    );
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn monte_carlo_full_0_adjective() {
    monte_carlo_full(0);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn monte_carlo_full_1_linking_verb() {
    monte_carlo_full(1);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn monte_carlo_full_2_intransitive_verb() {
    monte_carlo_full(2);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn monte_carlo_full_3_affiliation() {
    monte_carlo_full(3);
}

#[test]
#[cfg_attr(debug_assertions, ignore)]
fn monte_carlo_full_4_adjective_and_affiliation() {
    monte_carlo_full(4);
}
