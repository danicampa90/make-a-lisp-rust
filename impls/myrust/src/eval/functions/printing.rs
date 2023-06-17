use std::rc::Rc;

use crate::read::{AstNode, AstPrintFormat, AstPrinter};

use super::{FunctionCallData, FunctionCallResult, FunctionCallResultSuccess, NativeFunction};

pub fn functions() -> Vec<Rc<dyn NativeFunction>> {
    vec![
        Rc::new(PrintFunction::new_print(AstPrintFormat::Repr, " ", "prn")),
        Rc::new(PrintFunction::new_print(
            AstPrintFormat::Readable,
            " ",
            "println",
        )),
        Rc::new(PrintFunction::new_tostring(
            AstPrintFormat::Repr,
            " ",
            "pr-str",
        )),
        Rc::new(PrintFunction::new_tostring(
            AstPrintFormat::Readable,
            "",
            "str",
        )),
    ]
}

struct PrintFunction {
    format: AstPrintFormat,
    print: bool,
    separator: &'static str,
    name: &'static str,
}
impl PrintFunction {
    fn new_tostring(
        format: AstPrintFormat,
        separator: &'static str,
        name: &'static str,
    ) -> PrintFunction {
        PrintFunction {
            format,
            separator,
            name,
            print: false,
        }
    }
    fn new_print(
        format: AstPrintFormat,
        separator: &'static str,
        name: &'static str,
    ) -> PrintFunction {
        PrintFunction {
            format,
            separator,
            name,
            print: true,
        }
    }
}

impl NativeFunction for PrintFunction {
    fn evaluates_arguments(&self) -> bool {
        true
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn run(&self, mut data: FunctionCallData) -> FunctionCallResult {
        let mut builder = string_builder::Builder::new(64);
        let mut first_print = true;

        let printer = AstPrinter::new(self.format);

        let (mut params, env) = data.destructure();

        for ast in params.into_iter() {
            let str = printer.ast_to_string(&ast);
            if first_print {
                builder.append(str);
            } else {
                builder.append(self.separator);
                builder.append(str);
            }
            first_print = false;
        }

        if self.print {
            println!("{}", builder.string().unwrap());
            return Ok(FunctionCallResultSuccess::Value(AstNode::Nil));
        } else {
            return Ok(FunctionCallResultSuccess::Value(AstNode::String(
                builder.string().unwrap(),
            )));
        }
    }
}
