trait Visitor<R> {
    fn visitBinaryExpr(&self, binary: Expr) -> R;
    fn visitGroupingExpr(&self, grouping: Expr) -> R;
    fn visitLiteralExpr(&self, literal: Expr) -> R;
    fn visitUnaryExpr(&self, unary: Expr) -> R;
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
