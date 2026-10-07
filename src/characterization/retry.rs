// The retry / reshuffle loop of `generate_haiku_with`, made deterministic:
// a scripted `Chooser` decides the shuffles, and hand-made patterns (which
// draw nothing) decide what each try returns.
//
// Reminder of the shuffle (random.rs): for i = 4, 3, 2, 1 it swaps item i
// with item `index(i + 1)`. Answering 4, 3, 2, 1 leaves the patterns in place,
// which is what the script does once its explicit answers are used up.

use std::cell::{Cell, RefCell};
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;
use std::rc::Rc;

use crate::adj_enums::AdjId;
use crate::combination::Combination;
use crate::combination_data::Combinations;
use crate::common_enums::BlackLists;
use crate::haiku::generate_haiku_with;
use crate::noun_enums::NounId;
use crate::random::Chooser;
use crate::wordgroup::WordGroup;

const NO_COMBINATION: &str = "#err#generate_haiku#no combination found";

struct Script {
    answers: VecDeque<usize>,
    /// the `len` of every question asked
    asked: Vec<usize>,
}

impl Script {
    fn new(answers: &[usize]) -> Script {
        Script {
            answers: answers.iter().cloned().collect(),
            asked: vec![],
        }
    }
}

impl Chooser for Script {
    fn index(&mut self, len: usize) -> usize {
        self.asked.push(len);
        match self.answers.pop_front() {
            Some(answer) => {
                assert!(answer < len);
                answer
            }
            None => len - 1,
        }
    }
}

type Calls = Rc<Cell<usize>>;

fn calls() -> Calls {
    Rc::new(Cell::new(0))
}

/// Always the same line.
fn fixed(text: &'static str, feet: u8, calls: &Calls) -> Combination {
    let calls = Rc::clone(calls);
    Box::new(move |_, _| {
        calls.set(calls.get() + 1);
        Ok((WordGroup::new(text, feet), BlackLists::new_empty()))
    })
}

/// Always Err.
fn failing(calls: &Calls) -> Combination {
    let calls = Rc::clone(calls);
    Box::new(move |_, _| {
        calls.set(calls.get() + 1);
        Err(vec![String::from("err#test")])
    })
}

fn run(mut patterns: Combinations, script: &mut Script) -> [String; 3] {
    match generate_haiku_with(&mut patterns, script) {
        Ok(haiku) => haiku,
        Err(errs) => panic!("{:?}", errs),
    }
}

fn haiku(lines: [&str; 3]) -> [String; 3] {
    [
        String::from(lines[0]),
        String::from(lines[1]),
        String::from(lines[2]),
    ]
}

#[test]
fn line_n_uses_the_nth_pattern_after_the_shuffle() {
    let (a, b, c, d, e) = (calls(), calls(), calls(), calls(), calls());
    let patterns = || {
        [
            fixed("a five", 5, &a),
            fixed("b seven", 7, &b),
            fixed("c wrong", 6, &c),
            fixed("d wrong", 6, &d),
            fixed("e five", 5, &e),
        ]
    };
    // shuffle left in place: a, b, c on the three lines; c never fits
    let mut script = Script::new(&[]);
    assert_eq!(
        run(patterns(), &mut script),
        haiku(["A five", "b seven", NO_COMBINATION])
    );
    assert_eq!(a.get(), 1);
    assert_eq!(b.get(), 1);

    // first draw swaps items 4 and 2: a, b, e
    let mut script = Script::new(&[2]);
    assert_eq!(
        run(patterns(), &mut script),
        haiku(["A five", "b seven", "e five"])
    );
    // one shuffle, nothing else: the hand-made patterns draw nothing
    assert_eq!(script.asked, vec![5, 4, 3, 2]);
    assert_eq!((a.get(), b.get(), e.get(), d.get()), (2, 2, 1, 0));
}

// After 1000 failed tries of a line the patterns are shuffled again, ONCE,
// and the later lines use the new order too: here pattern a serves line 0 and,
// after the reshuffle, line 2.
#[test]
fn reshuffle_after_1000_failed_tries() {
    let (a, bad, c, d, e) = (calls(), calls(), calls(), calls(), calls());
    let patterns = [
        fixed("a five", 5, &a),
        fixed("bad", 1, &bad),
        fixed("c five", 5, &c),
        fixed("d seven", 7, &d),
        fixed("e five", 5, &e),
    ];
    // first shuffle: in place. Second: [a, bad, c, d, e] -> swap(3, 1) ->
    // [a, d, c, bad, e] -> swap(2, 0) -> [c, d, a, bad, e]
    let mut script = Script::new(&[4, 3, 2, 1, 4, 1, 0, 1]);
    assert_eq!(
        run(patterns, &mut script),
        haiku(["A five", "d seven", "a five"])
    );
    assert_eq!(script.asked, vec![5, 4, 3, 2, 5, 4, 3, 2]);
    assert_eq!(bad.get(), 1000);
    assert_eq!((a.get(), c.get(), d.get(), e.get()), (2, 0, 1, 0));
}

// A line gives up after 10001 tries (Err and wrong feet both count), with one
// reshuffle on the way, and the next line starts again from scratch.
#[test]
fn a_line_gives_up_after_10001_tries() {
    let (a, b, c, d, e) = (calls(), calls(), calls(), calls(), calls());
    let patterns = [
        failing(&a),
        fixed("b wrong", 4, &b),
        fixed("c wrong", 6, &c),
        fixed("d wrong", 6, &d),
        fixed("e wrong", 6, &e),
    ];
    let mut script = Script::new(&[]);
    // not even the first line is capitalised: the marker is put as is
    assert_eq!(
        run(patterns, &mut script),
        haiku([NO_COMBINATION, NO_COMBINATION, NO_COMBINATION])
    );
    assert_eq!((a.get(), b.get(), c.get()), (10_001, 10_001, 10_001));
    assert_eq!((d.get(), e.get()), (0, 0));
    // the initial shuffle and one reshuffle per line
    assert_eq!(script.asked.len(), 16);
}

/// Wrong feet, except at call number `good_call`.
fn late(text: &'static str, good_call: usize, calls: &Calls) -> Combination {
    let calls = Rc::clone(calls);
    Box::new(move |_, _| {
        calls.set(calls.get() + 1);
        let feet = if calls.get() == good_call { 5 } else { 1 };
        let mut used = BlackLists::new_empty();
        if calls.get() == good_call {
            used.nouns.insert(NounId::Lune);
        }
        Ok((WordGroup::new(text, feet), used))
    })
}

/// Fits line 1 and records the noun blacklist it is given.
fn recorder(seen: &Rc<RefCell<Vec<HashSet<NounId>>>>) -> Combination {
    let seen = Rc::clone(seen);
    Box::new(move |_, blacklists| {
        seen.borrow_mut().push(blacklists.nouns.clone());
        Ok((WordGroup::new("seven", 7), BlackLists::new_empty()))
    })
}

// CURRENT BEHAVIOUR (off-by-one): the give-up test runs after the try, so a
// line found at the 10001st try is replaced by the marker, although its words
// have already been blacklisted for the following lines.
#[test]
fn a_line_found_at_the_last_try_is_thrown_away_but_blacklisted() {
    let lune: HashSet<NounId> = HashSet::from_iter([NounId::Lune].iter().cloned());
    for (good_call, first_line) in [(10_000, "Late"), (10_001, NO_COMBINATION)].iter() {
        let (a, c) = (calls(), calls());
        let seen = Rc::new(RefCell::new(vec![]));
        let patterns = [
            late("late", *good_call, &a),
            recorder(&seen),
            fixed("c five", 5, &c),
            fixed("d", 1, &c),
            fixed("e", 1, &c),
        ];
        // the reshuffle of line 0 (answers 5 to 8) leaves the order in place too
        let mut script = Script::new(&[]);
        assert_eq!(
            run(patterns, &mut script),
            haiku([*first_line, "seven", "c five"])
        );
        assert_eq!(a.get(), *good_call);
        assert_eq!(*seen.borrow(), vec![lune.clone()]);
    }
}

// What a line returns is merged into the blacklists given to the next tries,
// after `adjust_blacklist` (one season word bans them all, midi bans minuit).
// Failed tries (Err or wrong feet) leave the blacklists alone.
#[test]
fn blacklists_flow_from_line_to_line() {
    let seen: Rc<RefCell<Vec<(HashSet<NounId>, usize, usize)>>> = Rc::new(RefCell::new(vec![]));
    let tries = calls();
    let line = |text: &'static str, feet: u8, good_try: usize, noun: NounId| -> Combination {
        let seen = Rc::clone(&seen);
        let tries = Rc::clone(&tries);
        Box::new(move |_, blacklists: &BlackLists| {
            tries.set(tries.get() + 1);
            seen.borrow_mut().push((
                blacklists.nouns.clone(),
                blacklists.adjs.len(),
                blacklists.verbs.len(),
            ));
            let mut used = BlackLists::new_empty();
            used.nouns.insert(noun);
            used.adjs.insert(AdjId::Bleu);
            // the words of a try with the wrong feet must not be blacklisted
            let feet = if tries.get() == good_try { feet } else { 1 };
            Ok((WordGroup::new(text, feet), used))
        })
    };
    let patterns = [
        line("zero", 5, 2, NounId::Ete),
        line("one", 7, 4, NounId::Midi),
        line("two", 5, 5, NounId::Lune),
        line("unused", 5, 0, NounId::Cerf),
        line("unused", 5, 0, NounId::Cerf),
    ];
    let mut script = Script::new(&[]);
    assert_eq!(run(patterns, &mut script), haiku(["Zero", "one", "two"]));

    let set = |nouns: &[NounId]| -> HashSet<NounId> { HashSet::from_iter(nouns.iter().cloned()) };
    let seasons = [
        NounId::Printemps,
        NounId::Ete,
        NounId::Automne,
        NounId::Hiver,
    ];
    let mut with_midi = seasons.to_vec();
    with_midi.extend([NounId::Midi, NounId::Minuit].iter().cloned());
    // (nouns, number of adjectives, number of verbs) seen by each try
    let expected = vec![
        // line 0: two tries, nothing blacklisted
        (set(&[]), 0, 0),
        (set(&[]), 0, 0),
        // line 1: the four seasons, and bleu + the four season adjectives
        (set(&seasons), 5, 0),
        (set(&seasons), 5, 0),
        // line 2: plus midi and minuit
        (set(&with_midi), 5, 0),
    ];
    assert_eq!(*seen.borrow(), expected);
}
