use std::collections::HashMap;

type Lemme = String;

enum Gender {
    Female,
    Male,
}

enum Noun {
    Common(Lemme, Gender),
    Proper(Lemme, Gender),
}

enum Article {
    Definite(Lemme, Gender)
}

enum Lexeme {
    Noun(Noun),
    Article(Article),
    Preposition(Lemme)
}


fn main() {
    let mut categories = HashMap::new();
    categories.insert("article défini", vec![
        Lexeme::Article(Article::Definite(String::from("le"), Gender::Female)),
        Lexeme::Article(Article::Definite(String::from("la"), Gender::Male)),
    ]);
    categories.insert("astre", vec![
        Lexeme::Noun(Noun::Proper(String::from("Lune"), Gender::Female)),
        Lexeme::Noun(Noun::Proper(String::from("Soleil"), Gender::Male)),
    ]);
    categories.insert("préposition de temps", vec![
        Lexeme::Preposition(String::from("de")),
    ]);
    categories.insert("saison", vec![
        Lexeme::Noun(Noun::Common(String::from("printemps"), Gender::Male)),
        Lexeme::Noun(Noun::Common(String::from("été"), Gender::Male)),
        Lexeme::Noun(Noun::Common(String::from("automne"), Gender::Male)),
        Lexeme::Noun(Noun::Common(String::from("hiver"), Gender::Male)),
    ]);
    let combination = [        
        categories.get("article défini").and_then(|val|val.first()),
        categories.get("astre").and_then(|val|val.first()),
        categories.get("préposition de temps").and_then(|val|val.first()),
        categories.get("saison").and_then(|val|val.first()),        
    ];
    let result: Vec<&str> = combination
    .iter()
    .map(|cat_option| match cat_option {
        Some(cat) => {
            match cat {
                Lexeme::Article(variant) => match variant {
                    Article::Definite(lemme, _gender) => lemme,
                } 
                Lexeme::Noun(variant) => match variant {
                    Noun::Proper(lemme, _gender) => lemme,
                    Noun::Common(lemme, _gender) => lemme,
                } 
                Lexeme::Preposition(lemme) => lemme,
            }
        },
        _ => "error"
    })
    .collect();
    println!("{}", result.join(" "));
}
