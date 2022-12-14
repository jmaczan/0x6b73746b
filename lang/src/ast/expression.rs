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

impl Visitor for Binary {
    fn accept<R>(&self, visitor: Visitor<R>) -> R {
        return visitor.visitBinaryExpr(&self);
    }
}

pub struct Grouping {
    expression: Expr,
}

impl Visitor for Grouping {
    fn accept<R>(&self, visitor: Visitor<R>) -> R {
        return visitor.visitGroupingExpr(&self);
    }
}

pub struct Literal {
    value: String,
}

impl Visitor for Literal {
    fn accept<R>(&self, visitor: Visitor<R>) -> R {
        return visitor.visitLiteralExpr(&self);
    }
}

pub struct Unary {
    operator: Token,
    right: Expr,
}

impl Visitor for Unary {
    fn accept<R>(&self, visitor: Visitor<R>) -> R {
        return visitor.visitUnaryExpr(&self);
    }
}
