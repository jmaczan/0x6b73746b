use std::io::Write;
use std::{
    fs::{DirBuilder, File},
    path::Path,
};

pub fn generate_ast(output_directory: &str) {
    define_ast(
        output_directory,
        "expression.rs",
        Vec::from([
            "Binary = left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>",
            "Grouping = expression: Box<dyn Expr>",
            "Literal = value: String", // originally it was Java Object, it needs changes in the future; see lexical_analysis -> struct Token.literal comments
            "Unary = operator: Token, right: Box<dyn Expr>",
        ]),
    );
}

fn define_ast(output_directory: &str, file_name: &str, types: Vec<&str>) {
    let output_directory_path = Path::new(output_directory);

    if !output_directory_path.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(output_directory)
            .unwrap();
    }

    let mut file = File::create(output_directory_path.join(file_name)).unwrap();

    writeln!(file, "use crate::lexical_analysis::Token;").unwrap();

    writeln!(
        file,
        "#[derive(Copy)]\npub trait Expr {{\n    fn accept(&self, visitor: Box<dyn Visitor>) -> &str;\n}}\n"
    )
    .unwrap();

    writeln!(file, "pub trait Visitor {{").unwrap();

    for ast_type in &types {
        let ast_type_components = ast_type.split("=").collect::<Vec<&str>>();
        let name: &str = ast_type_components.get(0).unwrap().trim();
        let struct_signature = format!("    fn visit{name}Expr(&self, expr: &{name}) -> &str;");
        writeln!(file, "{struct_signature}").unwrap();
    }

    writeln!(file, "}}").unwrap();

    for ast_type in types {
        let ast_type_components = ast_type.split("=").collect::<Vec<&str>>();
        let name: &str = ast_type_components.get(0).unwrap().trim();
        let fields: &str = ast_type_components.get(1).unwrap().trim();
        define_type(&mut file, name, fields);

        let accept =
            format!("impl Expr for {name} {{\nfn accept(&self, visitor: Box<dyn Visitor>) -> &'static str {{return visitor.visit{name}Expr(&self);}}");
        writeln!(file, "{accept}").unwrap();
        writeln!(file, "}}\n").unwrap();
    }
}

fn define_type(file: &mut File, name: &str, fields: &str) {
    let struct_signature = format!("pub struct {name} {{");
    writeln!(file, "{struct_signature}").unwrap();
    for field in fields.split(",").collect::<Vec<&str>>() {
        let field_trimmed = field.trim();
        writeln!(file, "    pub {field_trimmed},").unwrap();
    }
    writeln!(file, "}}\n").unwrap();
}
