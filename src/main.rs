use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom; 

enum Gender {
    Female,
    Male,
}

struct Noun {
    lemme: String,
    gender: Gender
}
type StringMaker = fn(&Noun) -> String;

fn get_with_definite_article(noun: &Noun) -> String {
    let article = match noun.gender {
        Gender::Male => "le",
        Gender::Female => "la",
    };
    let mut word = String::new();
    word.push_str(article);
    word.push(' ');
    word.push_str(&noun.lemme);
    word
}

fn get_as_noun_apposition(noun: &Noun) -> String {
/*     let french_vowels = Regex::new(r"[aeiouyàèìòùÀÈÌÒÙáéíóúýÁÉÍÓÚÝâêîôûÂÊÎÔÛãñõÃÑÕäëïöüÿÄËÏÖÜŸhH]").unwrap();
    let first = noun.lemme.chars().next();
    match first {
        Some(letter) => {
            let apposition = if french_vowels.is_match(letter) { "d'" } else { "de "};
            [apposition, &noun.lemme].join("")
        },
        None => String::from(""),
    } */

    let ellisions = [
        'a', 'é', 'h'
    ];

    let first = noun.lemme.chars().next();
    match first {
        Some(letter) => [
            if ellisions.contains(&letter) { "d'" } else { "de " },
            &noun.lemme
        ].join(""),
        None => String::from("")
    }
}

impl Noun {
    fn new(lemme: &str, gender: Gender) -> Noun {
        Noun {
            lemme: String::from(lemme),
            gender: gender
        }
    }
}

fn main() {
    let mut categories = HashMap::new();
    categories.insert(
        "astre",
        vec![
            Noun::new("Lune", Gender::Female),
            Noun::new("Soleil", Gender::Male),
        ],
    );
    categories.insert(
        "saison",
        vec![
            Noun::new("printemps", Gender::Male),
            Noun::new("été", Gender::Male),
            Noun::new("automne", Gender::Male),
            Noun::new("hiver", Gender::Male),
        ],
    );
    let combination: [(&str, StringMaker); 2] = [
        ("astre", get_with_definite_article),
        ("saison", get_as_noun_apposition),
    ];
    let random_result: Vec<Option<String>> = combination
        .iter()
        .map(|key|categories
            .get(key.0)
            .and_then(|cat|{
                let mut rng = thread_rng();
                cat.choose(&mut rng)
            })
            .and_then(|i|Some(key.1(i)))
        ).collect();

    let result = random_result
        .iter()
        .map(|res_option|match res_option {
            Some(str) => str,
            None => ""
        })
        .collect::<Vec<&str>>();
    println!("Result:");
    println!("{}", result.join(" ")); 
}
