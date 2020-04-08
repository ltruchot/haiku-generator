// IMPORTS
// externals
use unicode_segmentation::UnicodeSegmentation;
use rand::seq::SliceRandom;
use rand::rngs::{ThreadRng};

// strings
use crate::string;
use string::{uppercase_first_letter};
// wordgroups
use crate::wordgroup;
use wordgroup::{WordGroup, combine_word_options};

// combinations
use crate::combination_data;
use combination_data::{Combinations};

// nouns
use crate::noun_enums;
use noun_enums::{NounId};
use crate::noun;
use noun::{pick_rand_noun};


// EXPORTS
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
                text: String::from(&result.text),
                foots: (result.foots.0, result.foots.1)
            }
            
        },
        None => WordGroup {
            text: String::from("#error#check_haiku_form#last letter should exist"),
            foots: (0, 0)
        }
    };
    // println!("{} ({}, {})", haiku_form[nb], wg.foots.0, wg.foots.1);
    if  haiku_form[nb] >= wg.foots.0 && haiku_form[nb] <= wg.foots.1 {
        Some(String::from(wg.text))
    } else {
        None
    }
}

pub fn generate_haiku(combinations: &Combinations) -> Result<[String; 3], Vec<String>> {
    //let mut noun_black_list: Vec<NounId> = vec![];
    // compose haiku
    let mut haiku = [String::from(""), String::from(""), String::from("")];
    for nb in 0..3 {
        let mut is_running = true;
        // let mut current_noun_id: Option<NounId> = None;
        while is_running {
            let _res = match combinations.get(nb) {
                Some(comb) => {
                    comb().and_then(|wg|{
                        match check_haiku_form([5, 7, 5], nb, &wg) {
                            Some(s) => {
                                let sentence = if nb == 0 { uppercase_first_letter(&s) } else { s };
                                haiku[nb] = sentence;
                                is_running = false;
                                println!("{}", &haiku[nb]);
                                Ok(haiku[nb].clone())
                            }
                            None => Err(vec![String::from("warn#generate_haiku#haiku not well formed. retry...")]),
                        }
                    });
                },
                None => (),
            };
            ()
            // println!("{} ({} - {})", result.text, result.foots.0, result.foots.1);
        };
        println!("{}", is_running);
    }
    Ok(haiku)
}