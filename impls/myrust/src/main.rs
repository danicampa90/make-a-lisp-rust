mod read;
use read::{InputReader, Lexer, MalTypePrinter, Parser, ParsingError};

fn main() {
    let mut input = InputReader::new();
    let lexer = Lexer::create_lexer_iterator(&mut input);
    let mut parser = Parser::new(lexer);

    let ast_printer = MalTypePrinter::new();
    loop {
        match parser.read_form(true) {
            Ok(ast) => println!("{}", ast_printer.ast_to_string(&ast)),
            Err(ParsingError::Eof) => return,
            Err(err) => println!("Err: {:?}", err),
        }
    }
    /*
    while let Ok(str) = lexer::Lexer::read_next_token(&mut input) {
    }*/
}
