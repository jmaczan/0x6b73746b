use crate::lexical_analysis::Token;
pub trait Expr {
    fn accept<R>(&self, visitor: dyn Visitor<R>) -> R;
}

pub trait Visitor<R> {
    fn visitBinaryExpr(&self, expr: Binary) -> R;
    fn visitGroupingExpr(&self, expr: Grouping) -> R;
    fn visitLiteralExpr(&self, expr: Literal) -> R;
    fn visitUnaryExpr(&self, expr: Unary) -> R;
}
pub struct Binary {
    pub left: dyn Expr,
    pub operator: Token,
    pub right: dyn Expr,
}

impl Expr for Binary {
    fn accept<R>(&self, visitor: dyn Visitor<R>) -> R {
        return visitor.visitBinaryExpr(&self);
    }
}

pub struct Grouping {
    pub expression: dyn Expr,
}

impl Expr for Grouping {
    fn accept<R>(&self, visitor: dyn Visitor<R>) -> R {
        return visitor.visitGroupingExpr(&self);
    }
}

pub struct Literal {
    pub value: String,
}

impl Expr for Literal {
    fn accept<R>(&self, visitor: dyn Visitor<R>) -> R {
        return visitor.visitLiteralExpr(&self);
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: dyn Expr,
}

impl Expr for Unary {
    fn accept<R>(&self, visitor: dyn Visitor<R>) -> R {
        return visitor.visitUnaryExpr(&self);
    }
}
