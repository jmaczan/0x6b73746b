use crate::lexical_analysis::Token;
pub trait Expr {
    fn accept(&self, visitor: Box<dyn Visitor>) -> &'static str;
}

pub trait Visitor {
    fn visitBinaryExpr(&self, expr: &Binary) -> &str;
    fn visitGroupingExpr(&self, expr: &Grouping) -> &str;
    fn visitLiteralExpr(&self, expr: &Literal) -> &str;
    fn visitUnaryExpr(&self, expr: &Unary) -> &str;
}
pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Binary {
fn accept(&self, visitor: Box<dyn Visitor>) -> &'static str {return visitor.visitBinaryExpr(&self);}
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

impl Expr for Grouping {
fn accept(&self, visitor: Box<dyn Visitor>) -> &'static str {return visitor.visitGroupingExpr(&self);}
}

pub struct Literal {
    pub value: String,
}

impl Expr for Literal {
fn accept(&self, visitor: Box<dyn Visitor>) -> &'static str {return visitor.visitLiteralExpr(&self);}
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>, 
}

impl Expr for Unary {
fn accept(&self, visitor: Box<dyn Visitor>) -> &'static str {return visitor.visitUnaryExpr(&self);}
}

