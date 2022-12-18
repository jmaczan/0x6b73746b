use super::ast_printer::AstPrinter;
use crate::lexical_analysis::Token;
pub trait Expr {
    fn accept(&self, visitor: &AstPrinter) -> String;
}

pub trait Visitor {
    fn visit_binary_expr(&self, expr: &Binary) -> String;
    fn visit_grouping_expr(&self, expr: &Grouping) -> String;
    fn visit_literal_expr(&self, expr: &Literal) -> String;
    fn visit_unary_expr(&self, expr: &Unary) -> String;
}
pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn accept(&self, visitor: &AstPrinter) -> String {
        return visitor.visit_binary_expr(&self).to_string();
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn accept(&self, visitor: &AstPrinter) -> String {
        return visitor.visit_grouping_expr(&self).to_string();
    }
}

pub struct Literal {
    pub value: String,
}

impl Expr for Literal {
    fn accept(&self, visitor: &AstPrinter) -> String {
        return visitor.visit_literal_expr(&self).to_string();
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn accept(&self, visitor: &AstPrinter) -> String {
        return visitor.visit_unary_expr(&self).to_string();
    }
}
