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
use combination_data::{Constructions};

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

pub fn generate_haiku(constructions: &Constructions, rng: &mut ThreadRng) -> [String; 3] {
    let mut noun_black_list: Vec<NounId> = vec![];
    // compose haiku
    let mut haiku = [String::from(""), String::from(""), String::from("")];
    for nb in 0..3 {
        let mut is_running = true;
        let mut current_noun_id: Option<NounId> = None;
        while is_running {
            let random_result: Vec<Option<WordGroup>> = constructions
                .get(nb)
                .unwrap_or(&vec![])
                .iter()
                .map(|item| {
                    let (noun_cats, callbacks) = &item;
                    pick_rand_noun(noun_cats, rng, &noun_black_list)
                        .and_then(|noun| {
                            current_noun_id = Some(noun.id);
                            
                            callbacks
                                .choose(rng)
                                .and_then(|callback| Some(callback(&noun)))
                        })
                })
                .collect();

            let result = random_result
                .iter()
                .fold(
                    WordGroup::new_empty(),
                    combine_word_options,
                );

            match check_haiku_form([5, 7, 5], nb, &result) {
                Some(res) => {
                    
                    haiku[nb] = if nb == 0 { uppercase_first_letter(&res) } else { res };
                    is_running = false;
                    match current_noun_id {
                        Some(id) => noun_black_list.push(id),
                        None => ()
                    }   
                }
                None => (),
            }
            // println!("{} ({} - {})", result.text, result.foots.0, result.foots.1);
        }
    }
    haiku
}