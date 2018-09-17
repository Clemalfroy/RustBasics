use super::Expr

struct And {
    expression: Expr,
}

impl And {
    pub fn new(name: String) {
        expression = Expr::new(name);
    }
}

impl Expr::Expression for And {}