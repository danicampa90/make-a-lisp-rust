use std::fmt::{Debug, Display};

use string_builder::Builder;

use super::AstNode;
pub struct MalTypePrinter {}

impl MalTypePrinter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn ast_to_string(&self, ast: &AstNode) -> String {
        let mut builder = Builder::new(128);
        self.append_form(ast, &mut builder);
        builder.string().unwrap()
    }

    fn append_form(&self, ast: &AstNode, builder: &mut Builder) {
        match ast {
            AstNode::List(vec) => {
                builder.append("(");
                let mut first_element = true;
                for form in vec.iter() {
                    if !first_element {
                        builder.append(" ");
                    }
                    self.append_form(form, builder);
                    first_element = false;
                }
                builder.append(")")
            }
            AstNode::Int(num) => builder.append(num.to_string()),
            AstNode::UnresolvedSymbol(id) => builder.append(id.as_str()),
            AstNode::String(str) => self.append_string_repr(str, builder),
            AstNode::FunctionPtr(fptr) => builder.append(fptr.to_string()),
        }
    }
    fn append_string_repr(&self, str: &str, builder: &mut Builder) {
        builder.append('"');
        for ch in str.chars() {
            match ch {
                '"' => builder.append("\\\""),
                '\\' => builder.append("\\\\"),
                '\n' => builder.append("\\n"),
                '\r' => builder.append("\\r"),
                _ => builder.append(ch),
            }
        }
        builder.append('"');
    }
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ast_printer = MalTypePrinter::new();

        write!(f, "{}", ast_printer.ast_to_string(self))
    }
}

impl Debug for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
