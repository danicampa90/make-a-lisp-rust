use std::fmt::{Debug, Display};

use string_builder::Builder;

use super::{AstNode, Lexer};
pub struct AstPrinter {
    format: AstPrintFormat,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AstPrintFormat {
    Readable,
    Repr,
}

impl AstPrinter {
    pub fn new(format: AstPrintFormat) -> Self {
        Self { format }
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
            AstNode::Vector(vec) => {
                builder.append("[");
                let mut first_element = true;
                for form in vec.iter() {
                    if !first_element {
                        builder.append(" ");
                    }
                    self.append_form(form, builder);
                    first_element = false;
                }
                builder.append("]")
            }
            AstNode::Int(num) => builder.append(num.to_string()),
            AstNode::UnresolvedSymbol(id) => builder.append(id.as_str()),
            AstNode::String(str) => match self.format {
                AstPrintFormat::Readable => self.append_string_readable(str, builder),
                AstPrintFormat::Repr => self.append_string_repr(str, builder),
            },
            AstNode::Bool(true) => builder.append("true"),
            AstNode::Bool(false) => builder.append("false"),
            AstNode::Nil => builder.append("nil"),
            AstNode::FunctionPtr(fptr) => builder.append(fptr.to_string()),
            AstNode::Lambda(_) => builder.append("#<function>"),
            AstNode::Atom(atom) => {
                builder.append("(atom ");
                self.append_form(&(*atom.borrow()), builder);
                builder.append(")");
            }
            AstNode::HashMap(hm) => {
                builder.append("{");
                let mut first_element = true;
                for item in hm {
                    if !first_element {
                        builder.append(" ");
                    }
                    self.append_form(&AstNode::String(item.0.clone()), builder);
                    builder.append(" ");
                    self.append_form(item.1, builder);
                    first_element = false;
                }
                builder.append("}")
            }
        }
    }

    fn append_string_readable(&self, str: &str, builder: &mut Builder) {
        if str.starts_with(Lexer::KEYWORD_PREFIX) {
            builder.append(":");
            builder.append(&str[Lexer::KEYWORD_PREFIX.len()..]);
            return;
        } else {
            builder.append(str)
        }
    }
    fn append_string_repr(&self, str: &str, builder: &mut Builder) {
        if str.starts_with(Lexer::KEYWORD_PREFIX) {
            builder.append(":");
            builder.append(&str[Lexer::KEYWORD_PREFIX.len()..]);
            return;
        }
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
        let ast_printer = AstPrinter::new(AstPrintFormat::Repr);

        write!(f, "{}", ast_printer.ast_to_string(self))
    }
}

impl Debug for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
