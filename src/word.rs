use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone)]
pub struct WordGroup {
    pub text: String,
    pub foots: (u8, u8), // min / max
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


pub fn take_last_grapheme(word: &str) -> String {
    let s = String::from(word);
    // check last
    let last = s.graphemes(true).last();
    String::from(match last {
        Some(letter) => letter,
        None => "",
    })
}

pub fn take_last_graphemes(word: &str, n: usize) -> String {
    let last_two_rev = word
        .graphemes(true)
        .rev()
        .take(n)
        .collect::<Vec<&str>>()
        .join("");
    String::from(
        last_two_rev
            .graphemes(true)
            .rev()
            .collect::<Vec<&str>>()
            .join(""),
    )
}

pub fn drop_last_graphemes(word: &str, n: usize) -> String {
    let rev = word
        .graphemes(true)
        .rev()
        .skip(n)
        .collect::<Vec<&str>>()
        .join("");
    String::from(rev.graphemes(true).rev().collect::<Vec<&str>>().join(""))
}