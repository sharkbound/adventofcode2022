#[derive(Debug)]
pub enum OperationSign {
    PLUS(SubstituteValue, SubstituteValue),
    MUL(SubstituteValue, SubstituteValue),
}

impl OperationSign {
    pub fn apply(&self, item: u64) -> u64 {
        match self {
            OperationSign::PLUS(sub1, sub2) => sub1.resolve(item) + sub2.resolve(item),
            OperationSign::MUL(sub1, sub2) => sub1.resolve(item) * sub2.resolve(item),
        }
    }
}

#[derive(Debug)]
pub enum SubstituteValue {
    ITEM,
    LITERAL(u64),
}

impl SubstituteValue {
    fn resolve(&self, item: u64) -> u64 {
        match &self {
            SubstituteValue::ITEM => item,
            SubstituteValue::LITERAL(v) => *v,
        }
    }
}
