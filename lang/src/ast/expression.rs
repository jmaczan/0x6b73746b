pub struct Expr {}

pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr
}