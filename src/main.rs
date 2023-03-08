use rustpython_parser::{parse_expression};

fn main() {
    println!("Hello, world!");
    let python_source = "print('Hello world')";
    let python_ast = parse_expression(python_source, "<embedded>").unwrap();
    println!("{:?}", python_ast);
}
