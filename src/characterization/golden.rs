// Zero-dependency golden-master helper.
//
// Goldens are plain text files under `tests/golden/`, compared as strings.
// They are read at run time (not `include_str!`) so that
//
//     UPDATE_GOLDEN=1 cargo test
//
// rewrites them instead of asserting.

use std::fs;
use std::path::PathBuf;

pub fn golden_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join(name)
}

pub fn updating() -> bool {
    match std::env::var("UPDATE_GOLDEN") {
        Ok(value) => value == "1",
        Err(_) => false,
    }
}

/// One line per item, `\n` separated, with a final `\n`.
pub fn lines_to_text(lines: &[String]) -> String {
    let mut text = lines.join("\n");
    text.push('\n');
    text
}

pub fn read_golden(name: &str) -> String {
    let path = golden_path(name);
    match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => panic!(
            "golden file {} cannot be read ({}); run with UPDATE_GOLDEN=1 to create it",
            path.display(),
            err
        ),
    }
}

/// The lines of a golden file, without the line terminators.
pub fn read_golden_lines(name: &str) -> Vec<String> {
    read_golden(name).lines().map(String::from).collect()
}

fn shorten(line: &str) -> String {
    let max = 200;
    if line.chars().count() <= max {
        String::from(line)
    } else {
        let mut cut: String = line.chars().take(max).collect();
        cut.push_str("[...]");
        cut
    }
}

/// `None` when both texts are equal, otherwise a short description of the
/// first difference (never the whole content).
pub fn diff_hint(expected: &str, actual: &str) -> Option<String> {
    if expected == actual {
        return None;
    }
    let exp: Vec<&str> = expected.lines().collect();
    let act: Vec<&str> = actual.lines().collect();
    let common = if exp.len() < act.len() { exp.len() } else { act.len() };
    let mut first = common;
    for i in 0..common {
        if exp[i] != act[i] {
            first = i;
            break;
        }
    }
    let show = |lines: &Vec<&str>| match lines.get(first) {
        Some(line) => shorten(line),
        None => String::from("<no such line>"),
    };
    Some(format!(
        "first difference at line {}\n  golden: {}\n  actual: {}\n  golden has {} lines / {} bytes, actual has {} lines / {} bytes",
        first + 1,
        show(&exp),
        show(&act),
        exp.len(),
        expected.len(),
        act.len(),
        actual.len()
    ))
}

/// Compares `actual` with the golden file `name`, or rewrites the file when
/// `UPDATE_GOLDEN=1`.
pub fn assert_golden(name: &str, actual: &str) {
    let path = golden_path(name);
    if updating() {
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).expect("cannot create the golden directory");
        }
        fs::write(&path, actual).expect("cannot write the golden file");
        return;
    }
    let expected = read_golden(name);
    if let Some(hint) = diff_hint(&expected, actual) {
        panic!(
            "golden mismatch for {}\n{}\n(if the change is intended: UPDATE_GOLDEN=1 cargo test)",
            path.display(),
            hint
        );
    }
}

pub fn assert_golden_lines(name: &str, lines: &[String]) {
    assert_golden(name, &lines_to_text(lines));
}

#[test]
fn diff_hint_is_none_for_equal_texts() {
    assert_eq!(diff_hint("a\nb\n", "a\nb\n"), None);
    assert_eq!(diff_hint("", ""), None);
}

#[test]
fn diff_hint_points_at_the_first_differing_line() {
    let hint = diff_hint("a\nb\nc\n", "a\nX\nc\n").unwrap();
    assert_eq!(
        hint,
        "first difference at line 2\n  golden: b\n  actual: X\n  golden has 3 lines / 6 bytes, actual has 3 lines / 6 bytes"
    );
}

#[test]
fn diff_hint_handles_a_missing_tail() {
    let hint = diff_hint("a\nb\n", "a\n").unwrap();
    assert_eq!(
        hint,
        "first difference at line 2\n  golden: b\n  actual: <no such line>\n  golden has 2 lines / 4 bytes, actual has 1 lines / 2 bytes"
    );
    // same lines, different final newline: still a mismatch
    assert!(diff_hint("a\n", "a").is_some());
}

#[test]
fn diff_hint_truncates_long_lines() {
    let long = "x".repeat(1000);
    let hint = diff_hint(&long, "y").unwrap();
    assert!(hint.len() < 500, "hint too long: {} bytes", hint.len());
}
