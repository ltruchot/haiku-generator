// A `Chooser` that walks a decision tree exhaustively.
//
// Every run replays the choices recorded so far and answers 0 to any new
// question; `advance` then moves to the next path the way an odometer moves
// to the next number (last choice + 1, with carry). Running the same
// deterministic function until `advance` returns false visits every leaf of
// its decision tree exactly once.
//
// With a uniform `index` (which is what the production `RandChooser` is), the
// probability of a leaf is the product of 1/len over its choices.

use crate::random::Chooser;

pub struct Odometer {
    /// (choice, number of options) for each decision of the current path
    path: Vec<(usize, usize)>,
    /// number of decisions already taken in the current run
    position: usize,
}

impl Odometer {
    pub fn new() -> Odometer {
        Odometer {
            path: vec![],
            position: 0,
        }
    }

    /// Moves to the next path; false when every path has been walked.
    pub fn advance(&mut self) -> bool {
        // a replayed prefix leads to the same questions: the run consumed
        // the whole recorded path
        assert_eq!(
            self.position,
            self.path.len(),
            "the tree is not deterministic"
        );
        self.position = 0;
        while let Some((choice, len)) = self.path.pop() {
            if choice + 1 < len {
                self.path.push((choice + 1, len));
                return true;
            }
        }
        false
    }

    pub fn probability(&self) -> f64 {
        self.path
            .iter()
            .map(|(_, len)| 1.0 / (*len as f64))
            .product()
    }

    /// The number of options of each decision of the current path.
    pub fn lens(&self) -> Vec<usize> {
        self.path.iter().map(|(_, len)| *len).collect()
    }
}

impl Chooser for Odometer {
    fn index(&mut self, len: usize) -> usize {
        assert!(len >= 1, "index(0) is not a choice");
        if self.position == self.path.len() {
            self.path.push((0, len));
        }
        let (choice, recorded_len) = self.path[self.position];
        assert_eq!(recorded_len, len, "the tree is not deterministic");
        self.position += 1;
        choice
    }
}

/// Every leaf of the decision tree of `run`: (result, probability, lens).
pub fn walk<R, F: FnMut(&mut Odometer) -> R>(mut run: F) -> Vec<(R, f64, Vec<usize>)> {
    let mut odometer = Odometer::new();
    let mut leaves = vec![];
    loop {
        let result = run(&mut odometer);
        leaves.push((result, odometer.probability(), odometer.lens()));
        if !odometer.advance() {
            return leaves;
        }
    }
}

// ------------------------------------------------- the seam itself: choose, shuffle

use std::collections::HashSet;

use crate::random::{choose, shuffle};

struct NeverAsked;

impl Chooser for NeverAsked {
    fn index(&mut self, len: usize) -> usize {
        panic!("index({}) called", len);
    }
}

#[test]
fn walk_of_a_function_without_choice_has_one_leaf() {
    let leaves = walk(|_| 7);
    assert_eq!(leaves, vec![(7, 1.0, vec![])]);
}

#[test]
fn walk_visits_every_path_once() {
    // 2 options, then 3 or 1 depending on the first answer: 4 leaves
    let leaves = walk(|odometer| {
        let first = odometer.index(2);
        let second = odometer.index(if first == 0 { 3 } else { 1 });
        (first, second)
    });
    let results: Vec<(usize, usize)> = leaves.iter().map(|leaf| leaf.0).collect();
    assert_eq!(results, vec![(0, 0), (0, 1), (0, 2), (1, 0)]);
    let probabilities: Vec<f64> = leaves.iter().map(|leaf| leaf.1).collect();
    assert_eq!(probabilities, vec![1.0 / 6.0, 1.0 / 6.0, 1.0 / 6.0, 0.5]);
    assert_eq!(leaves[3].2, vec![2, 1]);
}

#[test]
fn choose_on_an_empty_slice_is_none_without_any_draw() {
    let empty: [u8; 0] = [];
    assert_eq!(choose(&mut NeverAsked, &empty), None);
    let empty: Vec<String> = vec![];
    assert_eq!(choose(&mut NeverAsked, &empty), None);
}

#[test]
fn choose_draws_once_among_len_even_for_a_single_item() {
    let leaves = walk(|odometer| choose(odometer, &[42]).cloned());
    assert_eq!(leaves, vec![(Some(42), 1.0, vec![1])]);

    let leaves = walk(|odometer| choose(odometer, &['a', 'b', 'c']).cloned());
    let third = 1.0 / 3.0;
    assert_eq!(
        leaves,
        vec![
            (Some('a'), third, vec![3]),
            (Some('b'), third, vec![3]),
            (Some('c'), third, vec![3]),
        ]
    );
}

#[test]
fn shuffle_of_five_items_yields_the_120_orders_exactly_once() {
    let leaves = walk(|odometer| {
        let mut items = [0, 1, 2, 3, 4];
        shuffle(odometer, &mut items);
        items
    });
    assert_eq!(leaves.len(), 120);
    let distinct: HashSet<[i32; 5]> = leaves.iter().map(|leaf| leaf.0).collect();
    assert_eq!(distinct.len(), 120);
    for (order, probability, lens) in leaves.iter() {
        // a permutation ...
        let mut sorted = *order;
        sorted.sort();
        assert_eq!(sorted, [0, 1, 2, 3, 4]);
        // ... reached by four draws, each order as likely as the others
        assert_eq!(lens, &vec![5, 4, 3, 2]);
        assert!((probability - 1.0 / 120.0).abs() < 1e-15);
    }
    // answering "the last one" to every draw leaves the items in place
    assert_eq!(leaves[119].0, [0, 1, 2, 3, 4]);
}

#[test]
fn shuffle_of_zero_or_one_item_draws_nothing() {
    let mut none: [u8; 0] = [];
    shuffle(&mut NeverAsked, &mut none);
    let mut one = [1];
    shuffle(&mut NeverAsked, &mut one);
    assert_eq!(one, [1]);
    // two items: one draw among 2
    let leaves = walk(|odometer| {
        let mut two = ['a', 'b'];
        shuffle(odometer, &mut two);
        two
    });
    assert_eq!(leaves.len(), 2);
    assert_eq!(leaves[0], (['b', 'a'], 0.5, vec![2]));
    assert_eq!(leaves[1], (['a', 'b'], 0.5, vec![2]));
}

#[test]
fn production_chooser_answers_in_range() {
    let mut seen = [false; 3];
    for _ in 0..1000 {
        assert_eq!(super::seam::rand_index(1), 0);
        let index = super::seam::rand_index(3);
        assert!(index < 3);
        seen[index] = true;
    }
    // 1000 draws: missing one of 3 values has probability < 1e-170
    assert_eq!(seen, [true, true, true]);
}
