trait Accept {
    fn accept(&self);
}

pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

pub struct Grouping {
    expression: Expr,
}

pub struct Literal {
    value: String,
}

pub struct Unary {
    operator: Token,
    right: Expr,
}
