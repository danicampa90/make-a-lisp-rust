mod eval;
mod read;
use eval::{new_base_environment, Evaluator};
use read::{InputReader, Lexer, MalTypePrinter, Parser, ParsingError};

fn main() {
    let mut input = InputReader::new();
    let lexer = Lexer::create_lexer_iterator(&mut input);
    let mut parser = Parser::new(lexer);
    let evaluator = Evaluator::new();
    let environment = new_base_environment();

    let ast_printer = MalTypePrinter::new();
    loop {
        match parser.read_form(true) {
            Ok(ast) => {
                let eval_result = &evaluator.eval(ast, &environment);
                match eval_result {
                    Ok(res) => println!("{}", ast_printer.ast_to_string(res)),
                    Err(err) => println!("Error: {:?}", err),
                }
            }
            Err(ParsingError::Eof) => return,
            Err(err) => println!("Err: {:?}", err),
        }
    }
    /*
    while let Ok(str) = lexer::Lexer::read_next_token(&mut input) {
    }*/
}
