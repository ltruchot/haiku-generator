use std::collections::HashMap;
use rand::thread_rng;
use rand::seq::SliceRandom; 

enum Gender {
    Female,
    Male,
}

enum NounType {
    Common,
    Proper,
}

struct Noun {
    lemme: String,
    gender: Gender,
    #[allow(dead_code)]
    kind: NounType,
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
    fn new(lemme: &str, gender: Gender, kind: NounType) -> Noun {
        Noun {
            lemme: String::from(lemme),
            gender: gender,
            kind: kind,
        }
    }
}

fn main() {
    let mut categories = HashMap::new();
    categories.insert(
        "astre",
        vec![
            Noun::new("Lune", Gender::Female, NounType::Proper),
            Noun::new("Soleil", Gender::Male, NounType::Proper),
        ],
    );
    categories.insert(
        "saison",
        vec![
            Noun::new("printemps", Gender::Male, NounType::Common),
            Noun::new("été", Gender::Male, NounType::Common),
            Noun::new("automne", Gender::Male, NounType::Common),
            Noun::new("hiver", Gender::Male, NounType::Common),
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
