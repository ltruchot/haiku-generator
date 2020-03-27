pub struct Adj {
    pub lemme: String,
}

impl Adj {
    pub fn new(lemme: &str) -> Adj {
        Adj {
            lemme: String::from(lemme),
        }
    }
}