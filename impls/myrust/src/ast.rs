pub enum MalAtom {
    Name(String),
    IntNumber(i64),
    String(String),
    /*
    Bool(bool),
    Null, */
}

pub enum MalType {
    List(Vec<MalType>),
    Atom(MalAtom),
}
