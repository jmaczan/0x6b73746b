use crate::lexical_analysis::Token;

use super::ast_printer::AstPrinter;
pub trait Expr {
    fn accept(&self, visitor: &AstPrinter) -> String;
}

pub trait Visitor {
    fn visitBinaryExpr(&self, expr: &Binary) -> String;
    fn visitGroupingExpr(&self, expr: &Grouping) -> String;
    fn visitLiteralExpr(&self, expr: &Literal) -> String;
    fn visitUnaryExpr(&self, expr: &Unary) -> String;
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn accept(&self, visitor: &AstPrinter) -> String {
        return visitor.visitBinaryExpr(&self).to_string();
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn accept(&self, visitor: &AstPrinter) -> String {
        return visitor.visitGroupingExpr(&self).to_string();
    }
}

pub struct Literal {
    pub value: String,
}

impl Expr for Literal {
    fn accept(&self, visitor: &AstPrinter) -> String {
        return visitor.visitLiteralExpr(&self).to_string();
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn accept(&self, visitor: &AstPrinter) -> String {
        return visitor.visitUnaryExpr(&self).to_string();
    }
}
