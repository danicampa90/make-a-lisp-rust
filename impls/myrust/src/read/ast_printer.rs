use string_builder::Builder;

use super::{MalAtom, MalType};
pub struct MalTypePrinter {}

impl MalTypePrinter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn ast_to_string(&self, ast: &MalType) -> String {
        let mut builder = Builder::new(128);
        self.append_form(ast, &mut builder);
        builder.string().unwrap()
    }

    fn append_form(&self, ast: &MalType, builder: &mut Builder) {
        match ast {
            MalType::List(vec) => {
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
            MalType::Atom(MalAtom::IntNumber(num)) => builder.append(num.to_string()),
            MalType::Atom(MalAtom::Name(id)) => builder.append(id.as_str()),
            MalType::Atom(MalAtom::String(str)) => self.append_string_repr(str, builder),
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
