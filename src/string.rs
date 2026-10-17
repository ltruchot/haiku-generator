// IMPORTS
// externals
use unicode_segmentation::UnicodeSegmentation;

// EXPORTS
pub fn take_last_grapheme(word: &str) -> String {
    let s = String::from(word);
    // check last
    String::from(s.graphemes(true).next_back().unwrap_or_default())
}

pub fn take_last_graphemes(word: &str, n: usize) -> String {
    let last_two_rev = word
        .graphemes(true)
        .rev()
        .take(n)
        .collect::<Vec<&str>>()
        .join("");
    last_two_rev
        .graphemes(true)
        .rev()
        .collect::<Vec<&str>>()
        .join("")
}

pub fn drop_last_graphemes(word: &str, n: usize) -> String {
    let rev = word
        .graphemes(true)
        .rev()
        .skip(n)
        .collect::<Vec<&str>>()
        .join("");
    rev.graphemes(true).rev().collect::<Vec<&str>>().join("")
}

pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
