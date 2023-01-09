use std::collections::HashMap;
use rand::Rng;

#[derive(Eq, PartialEq, Hash, Debug)]
enum E {
    A,
    B,
    C,
}

fn main() {
    // let value = rand::thread_rng().gen_ratio(10, 20);
    // dbg!(value);
    let mut mapping: HashMap<E, u32> = HashMap::new();
    mapping.insert(E::A, 1);
    mapping.insert(E::B, 2);
    dbg!(mapping);
}
