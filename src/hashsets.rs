use core::hash::Hash;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn merge_hashsets<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T> where T: Eq + Hash + Clone {
    HashSet::from_iter(a.union(&b).cloned())
}


// pub type HashSetClosure<T> = Box<dyn Fn(&HashSet<T>) -> HashSet<T>>;

// pub fn closure<T> (u: Option<Vec<String>>) -> HashSetClosure<T> where T: Eq + Hash + Clone {
//     Box::new(move |hs| {
//         // println!("{:?}", u.as_ref().unwrap());
//         HashSet::from_iter(hs.iter().cloned())
//     })
// }

// fn main () {
//     let hs: HashSet<String> = HashSet::new();
//     let v = Some(vec![String::from("a")]);
//     let cl = closure(v);
//     for nb in 0..3 {
//         let new_hs = cl(&hs);
//         // !("{:?}", hs);
//     }
// }
