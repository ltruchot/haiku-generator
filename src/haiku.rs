// IMPORTS
// externals
use crate::common_enums::merge_blacklists;
use rand::seq::SliceRandom;
use rand::thread_rng;

// common
use crate::common_enums;
use common_enums::{BlackLists, adjust_blacklist};

// strings
use crate::string;
use string::{uppercase_first_letter};
// wordgroups
use crate::wordgroup;
use wordgroup::WordGroup;

// combinations
use crate::combination_data;
use combination_data::Combinations;

// EXPORTS
pub fn check_haiku_form(haiku_form: [u8; 3], nb: usize, result: &WordGroup) -> Option<String> {
    if haiku_form[nb] >= result.foots.0 && haiku_form[nb] <= result.foots.1 {
        // println!("{:?}", &result.foots);
        Some(String::from(&result.text))
    } else {
        None
    }
}

pub fn generate_haiku(combinations: &mut Combinations) -> Result<[String; 3], Vec<String>> {
    let mut rng = thread_rng();
    combinations.shuffle(&mut rng);
    //let mut noun_black_list: Vec<NounId> = vec![];
    // compose haiku
    let mut haiku = [String::from(""), String::from(""), String::from("")];
    let mut black_lists = BlackLists::new_empty();
    for nb in 0..3 {
        let mut is_running = true;
        let mut count = 0;
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
            count = count + 1;
            // security
            if count == 100 {
                combinations.shuffle(&mut rng);
            }
            if count > 1000 {
                haiku[nb] = String::from("#err#generate_haiku#no combination found");
                is_running = false;
            }
            ()
        }
    }
    Ok(haiku)
}
