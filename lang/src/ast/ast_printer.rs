use super::expression::{Binary, Expr, Grouping, Literal, Unary, Visitor};

pub struct AstPrinter {}

impl Visitor for AstPrinter {
    fn visitBinaryExpr(&self, expr: &Binary) -> String {
        self.parenthesize(expr.operator.lexeme.to_string(), vec![&expr.left, &expr.right])
    }

    fn visitGroupingExpr(&self, expr: &Grouping) -> String {
        self.parenthesize("group".to_string(), vec![&expr.expression])
    }

    fn visitLiteralExpr(&self, expr: &Literal) -> String {
        if expr.value == "" {
            return "nil".to_string()
        }

        expr.value.to_string()
    }

    fn visitUnaryExpr(&self, expr: &Unary) -> String {
        self.parenthesize(expr.operator.lexeme.to_string(), vec![&expr.right])
    }
}

impl AstPrinter {
    pub fn print(&self, expr: Box<dyn Expr>) -> String {
        return expr.accept(self);
    }

    fn parenthesize(&self, name: String, exprs: Vec<&Box<dyn Expr>>) -> String {
        //TODO
        "".to_string()
    }
}
