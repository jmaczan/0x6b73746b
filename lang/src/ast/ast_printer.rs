use super::expression::{Binary, Expr, Grouping, Literal, Unary, Visitor};

struct AstPrinter {}

impl Visitor for AstPrinter {
    fn visitBinaryExpr(&self, expr: &Binary) -> &str {
        self.parenthesize(expr.operator.lexeme.as_str(), vec![expr.left, expr.right])
    }

    fn visitGroupingExpr(&self, expr: &Grouping) -> &str {
        self.parenthesize("group", vec![expr.expression])
    }

    fn visitLiteralExpr(&self, expr: &Literal) -> &str {
        if expr.value == "" {
            return "nil"
        }

        expr.value.as_str()
    }

    fn visitUnaryExpr(&self, expr: &Unary) -> &str {
        self.parenthesize(expr.operator.lexeme.as_str(), vec![expr.right])
    }
}

impl AstPrinter {
    pub fn print(&self, expr: Box<dyn Expr>) -> &str {
        return expr.accept(self);
    }

    fn parenthesize(&self, name: &str, exprs: Vec<Box<dyn Expr>>) -> &str {
        //TODO
        ""
    }
}
