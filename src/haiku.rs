// IMPORTS
// externals
use crate::common_enums::merge_blacklists;

// common
use crate::common_enums;
use common_enums::{BlackLists, adjust_blacklist};

// strings
use crate::string;
use string::{uppercase_first_letter, take_last_grapheme, take_last_graphemes};
// wordgroups
use crate::wordgroup;
use wordgroup::WordGroup;

// combinations
use crate::combination_data;
use combination_data::Combinations;

// EXPORTS
pub fn check_haiku_form(haiku_form: [u8; 3], nb: usize, result: &WordGroup) -> Option<String> {
    // ellision on final "e" implie foot max decrement
    let last = take_last_grapheme(&result.text);
    let last_two = take_last_graphemes(&result.text, 2);
    let wg =  WordGroup {
        text: String::from(&result.text),
        foots: if last == "e" || last_two == "es"  {
            (result.foots.0, result.foots.1 - 1)
        } else {
            (result.foots.0, result.foots.1)
        }
    };
    
    if haiku_form[nb] >= wg.foots.0 && haiku_form[nb] <= wg.foots.1 {
        Some(String::from(wg.text))
    } else {
        None
    }
}

pub fn generate_haiku(combinations: &Combinations) -> Result<[String; 3], Vec<String>> {
    //let mut noun_black_list: Vec<NounId> = vec![];
    // compose haiku
    let mut haiku = [String::from(""), String::from(""), String::from("")];
    let mut black_lists = BlackLists::new_empty();
    for nb in 0..3 {
        let mut is_running = true;
        // let mut current_noun_id: Option<NounId> = None;
        while is_running {
            let res = match combinations.get(nb) {
                Some(comb) => comb(&black_lists),
                None => Err(vec![String::from(
                    "warn#generate_haiku#combination not found",
                )]),
            };
            match res {
                Ok((wg, bl)) => match check_haiku_form([5, 7, 5], nb, &wg) {
                    Some(s) => {
                        let sentence = if nb == 0 {
                            uppercase_first_letter(&s)
                        } else {
                            s
                        };
                        haiku[nb] = sentence;
                        is_running = false;
                        black_lists = merge_blacklists(&black_lists, &bl);
                        black_lists = adjust_blacklist(black_lists);
                    }
                    None => (),
                },
                Err(_) => (),
            }
            ()
        }
    }
    Ok(haiku)
}
