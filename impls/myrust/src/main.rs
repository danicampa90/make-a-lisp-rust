mod ast;
mod ast_printer;
mod input;
mod lexer;
mod parser;

use input::InputReader;

fn main() {
    let mut input = InputReader::new();
    let lexer = lexer::Lexer::create_lexer_iterator(&mut input);
    let mut parser = parser::Parser::new(lexer);

    let ast_printer = ast_printer::ASTPrinter::new();
    loop {
        match parser.read_form(true) {
            Ok(ast) => println!("{}", ast_printer.ast_to_string(&ast)),
            Err(parser::ParsingError::Eof) => return,
            Err(err) => println!("Err: {:?}", err),
        }
    }
    /*
    while let Ok(str) = lexer::Lexer::read_next_token(&mut input) {
    }*/
}
