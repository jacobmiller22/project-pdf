pub struct Object {
    typ: ObjectType,
    offset: usize,
    size: usize,
}

impl Object {
    pub fn new(typ: ObjectType, offset: usize, size: usize) -> Object {
        return Object { typ, offset, size };
    }
}

//boolean values, integers, real numbers, strings, names, arrays, dictionaries, streams, and the null object.

pub enum NumericObjectType {
    Integer,
    Real,
}

pub enum StringObjectType {
    Literal,
    Hexadecimal,
}

pub enum ObjectType {
    Boolean(bool),
    Numeric(NumericObjectType),
    String(StringObjectType),
    Name,
    Array,
    Dictionary,
    Stream,
    Null,
}
