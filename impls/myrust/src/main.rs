mod eval;
mod read;
use eval::{new_base_environment, EvalError, Evaluator, SharedEnvironment};
use read::{
    AstPrintFormat, AstPrinter, InputReader, InputSource, Lexer, Parser, ParsingError,
    REPLTerminalInputSource, StringInputSource,
};

fn main() {
    let evaluator = Evaluator::new();
    let environment = new_base_environment();
    let ast_printer = AstPrinter::new(AstPrintFormat::Repr);
    let mut ast_printer = Some(&ast_printer);

    // load initial environment via file parsing
    let startup_code = std::fs::read_to_string("startup.lisp");
    let startup_code = match startup_code {
        Ok(s) => s,
        Err(e) => {
            panic!(
                "Cannot load file 'startup.lisp':{:?}. Make sure it's in the current directory!",
                e
            )
        }
    };

    let mut input = InputReader::new(Box::new(StringInputSource::new(startup_code)));
    let lexer = Lexer::create_lexer_iterator(&mut input);
    let mut parser = Parser::new(lexer);
    let eval_result = run(&mut parser, &evaluator, environment.clone(), None);
    if eval_result.is_err() {
        println!("CRITICAL ERROR! Cannot load the base environment file 'env.lisp' due to the following error:");
        print_eval_result_error(eval_result);
        return;
    }

    let mut args: Vec<String> = std::env::args().collect();
    let inputsource: Box<dyn InputSource> = if args.len() >= 2 {
        let path = args.remove(1);
        let content = std::fs::read_to_string(path).unwrap();
        ast_printer = None;
        Box::new(StringInputSource::new(content))
    } else {
        Box::new(REPLTerminalInputSource::new())
    };

    // start REPL
    let mut input = InputReader::new(inputsource);
    let lexer = Lexer::create_lexer_iterator(&mut input);
    let mut parser = Parser::new(lexer);

    loop {
        let eval_result = run(&mut parser, &evaluator, environment.clone(), ast_printer);
        if eval_result.is_ok() {
            return;
        }
        print_eval_result_error(eval_result);
    }
}

fn run(
    parser: &mut Parser,
    evaluator: &Evaluator,
    environment: SharedEnvironment,
    printer: Option<&AstPrinter>,
) -> Result<(), EvalError> {
    loop {
        match parser.read_form(true) {
            Ok(ast) => {
                let eval_result = &evaluator.eval(ast, environment.clone())?;
                if let Some(printer) = printer {
                    println!("{}", printer.ast_to_string(eval_result))
                }
            }
            Err(ParsingError::EOF) => return Ok(()),
            Err(err) => {
                return Err(EvalError::custom_exception_str(format!(
                    "Parsing error: {:?}",
                    err
                )))
            }
        }
    }
}

fn print_eval_result_error(result: Result<(), EvalError>) {
    if let Err(err) = result {
        println!("Error: {}", err);
    }
}
