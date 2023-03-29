pub trait DebugLog {
    fn debug_expanded(&self) -> &Self;
    fn debug(&self) -> &Self;
}

impl<T> DebugLog for T
where
    T: std::fmt::Debug,
{
    fn debug_expanded(&self) -> &Self {
        println!("{:#?}", self);
        self
    }

    fn debug(&self) -> &Self {
        println!("{:?}", self);
        self
    }
}

pub trait DisplayLog {
    fn display(&self) -> &Self;
}

impl<T> DisplayLog for T
where
    T: std::fmt::Display,
{
    fn display(&self) -> &Self {
        println!("{}", self);
        self
    }
}
