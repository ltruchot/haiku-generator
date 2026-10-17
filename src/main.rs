fn main() {
    match haiku_generator::compose() {
        Ok(haiku) => println!("Haïku:\n{}", haiku.join("\n")),
        Err(errs) => println!("{:?}", errs),
    }
}
