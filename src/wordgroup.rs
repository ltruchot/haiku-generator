#[derive(Clone)]
pub struct WordGroup {
    pub text: String,
    pub foots: (u8, u8), // min / max
}

impl WordGroup {
    pub fn new_empty() -> WordGroup {
        WordGroup {
            text: String::from(""),
            foots: (0, 0),
        }
    }
}

pub fn check_ellision (letter: &char) -> bool {
    let ellisions = ['a', 'e', 'i', 'o', 'u', 'é', 'h'];
    ellisions.contains(letter)
}

pub fn add_words (a: &WordGroup, b: &WordGroup, with_space: bool) -> WordGroup {
    WordGroup {
        text: [
            String::from(&a.text), 
            String::from(&b.text)
        ].join(if with_space {" "} else {""}),
        foots: (a.foots.0 + b.foots.0, a.foots.1 + b.foots.1)
    }
}

pub fn combine_word_options (acc: WordGroup, wg_option: &Option<WordGroup>) -> WordGroup {
    match wg_option {
        Some(wg) => add_words(&acc, &wg, acc.text != ""),
        None => acc,
    }
}
