fn add_integers(a: u32, b: u32) -> u32 {
    b + a
}
fn add (a: u32) -> Box<dyn Fn(u32) -> u32> {
    Box::new(move |b| add_integers(a, b))
}

fn sub_integers(a: u32, b: u32) -> u32 {
    b - a
}
fn sub (a: u32) -> Box<dyn Fn(u32) -> u32> {
    Box::new(move |b| sub_integers(a, b))
 }

fn main () {
    for method in [add(1), sub(1)].iter() {
        println!("{}", method(10))
    }
}
