mod expr;
mod and;
mod or;
mod xor;
mod base;

enum Operation {
    And(and::And),
    Or(or::Or),
    Xor(xor::Xor),
    Base(base::Base),
}

pub struct Condition {
    name: String,
    true: bool,
    ambig: bool,
    condition_list: Vec<Operation>,
}

impl Condition {
    pub fn new(name: String) -> Condition {
        Condition {
            name,
            true: false,
            ambig: false,
            condition_list: Vec::new(),
        }
    }
    pub fn add_true(&self, condition) {
        self.trueif.push(condition)
    }
}