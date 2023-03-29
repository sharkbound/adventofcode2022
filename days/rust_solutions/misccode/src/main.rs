use std::marker::PhantomData;

fn main() {
    let t = Type::new(); // returns Type<Closed>
    let t = t.open(); // returns Type<Open>, .open() can only be called on Type<Closed>
    let t = t.close(); // returns Type<Closed>, .close() can only be called on Type<Open>
}

struct Opened;
struct Closed;
struct Type<State = Closed> {
    state: PhantomData<State>,
}

impl Type {
    pub fn new() -> Self {
        Type { state: PhantomData }
    }
}

impl Type<Opened> {
    pub fn close(self) -> Type<Closed> {
        Type {
            state: PhantomData,
        }
    }
}

impl Type<Closed> {
    pub fn open(self) -> Type<Opened> {
        Type {
            state: PhantomData,
        }
    }
}
