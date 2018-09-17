struct Rule {
    expression: String,
}

pub trait Expr {
    pub fn is_true(&self) {}
    pub fn is_valid(&self) {}
}
