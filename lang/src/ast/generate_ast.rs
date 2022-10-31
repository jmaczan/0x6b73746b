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
            "Binary = left: Expr, operator: Token, right: Expr",
            "Grouping = expression: Expr",
            "Literal = value: String", // originally it was Java Object, it needs changes in the future; see lexical_analysis -> struct Token.literal comments
            "Unary = operator: Token, right: Expr",
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

    for ast_type in types {
        let ast_type_components = ast_type.split("=").collect::<Vec<&str>>();
        let name: &str = ast_type_components.get(0).unwrap().trim();
        let fields: &str = ast_type_components.get(1).unwrap().trim();
        define_type(&mut file, name, fields);
    }
}

fn define_type(file: &mut File, name: &str, fields: &str) {
    let struct_signature = format!("pub struct {name} {{");
    writeln!(file, "{struct_signature}").unwrap();
    for field in fields.split(",").collect::<Vec<&str>>() {
        writeln!(file, "{field},").unwrap();
    }
    writeln!(file, "}}\n").unwrap();
}
