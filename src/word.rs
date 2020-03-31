#[derive(Clone)]
pub struct WordGroup {
    pub text: String,
    pub foots: (u8, u8), // min / max
}

pub fn add_words (a: WordGroup, b: WordGroup) -> WordGroup {
    WordGroup {
        text: [a.text, b.text].join(""),
        foots: (a.foots.0 + b.foots.0, a.foots.1 + b.foots.1)
    }
}