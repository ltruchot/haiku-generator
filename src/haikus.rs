use unicode_segmentation::UnicodeSegmentation;

use crate::word;
use word::{WordGroup};

pub fn check_haiku_form (haiku_form: [u8; 3], nb: usize, result: &WordGroup) -> Option<String> {
    // ellision on final "e" implie foot max decrement
    let last = result.text.graphemes(true).last();
    let wg = match last {
        Some(letter) => if letter == "e" { 
            WordGroup {
                text: String::from(&result.text),
                foots: (result.foots.0, result.foots.1 - 1)
            }
        } else {
            WordGroup {
                text: String::from("#error#check_haiku_form#last letter should exist"),
                foots: (0, 0)
            }
        },
        None => WordGroup {
                text: String::from(&result.text),
                foots: (result.foots.0, result.foots.1 - 1)
            }
    };
    if  haiku_form[nb] >= wg.foots.0 && haiku_form[nb] <= wg.foots.1 {
        Some(String::from(wg.text))
    } else {
        None
    }
}