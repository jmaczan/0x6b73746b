use super::expression::{Binary, Expr, Grouping, Literal, Unary, Visitor};

pub struct AstPrinter {}

impl Visitor for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        self.parenthesize(
            expr.operator.lexeme.to_string(),
            vec![&expr.left, &expr.right],
        )
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.parenthesize("group".to_string(), vec![&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        if expr.value == "" {
            return "nil".to_string();
        }

        expr.value.to_string()
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        self.parenthesize(expr.operator.lexeme.to_string(), vec![&expr.right])
    }
}

impl AstPrinter {
    pub fn print(&self, expr: Box<dyn Expr>) -> String {
        return expr.accept(self);
    }

    fn parenthesize(&self, name: String, exprs: Vec<&Box<dyn Expr>>) -> String {
        //TODO
        let mut parenthesized = "(".to_string() + &name;
        for expr in &exprs {
            parenthesized = parenthesized + " " + &expr.accept(self).to_string();
        }
        parenthesized = parenthesized + &")".to_string();
        return parenthesized;
    }
}
