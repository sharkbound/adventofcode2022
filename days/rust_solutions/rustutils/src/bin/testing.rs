use std::collections::HashMap;
use rustutils::collections::{CollectToHashSet};
use rustutils::logging::DebugLog;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Circle {
    radius: i64,
}

impl AsRef<Circle> for Circle {
    fn as_ref(&self) -> &Circle {
        self
    }
}

impl AsRef<i64> for Circle {
    fn as_ref(&self) -> &i64 {
        &self.radius
    }
}

fn main() {
    let mut circles = vec![Circle { radius: 0 }, Circle { radius: 1 }, Circle { radius: 2 }].into_iter().map(|x| (x.radius, x)).collect::<HashMap<i64, Circle>>();
    let first = circles.remove(&0).unwrap();
    first.debug();
    circles.debug();
    dbg!(circles.iter().collect_to_hashset());
}