use std::fmt;

#[derive(Debug)]
pub enum ObjectError {
    General(String), // TODO: Remove this and replace with more concrete. For now, general error
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectError::General(ref err) => write!(f, "ObjectError: {}", err),
        }
    }
}

impl std::error::Error for ObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            // Error cases involving std::error go here
            _ => None,
        }
    }
}
struct ObjectLabel {
    number: i64,
    generation: i64,
}

impl ObjectLabel {
    pub fn new(number: i64, generation: i64) -> ObjectLabel {
        return ObjectLabel { number, generation };
    }

    pub fn number(&self) -> i64 {
        return self.number;
    }

    pub fn generation(&self) -> i64 {
        return self.generation;
    }
}

pub struct Object {
    label: Option<ObjectLabel>,
    typ: ObjectType,
    offset: usize,
    size: usize,
}

impl Object {
    pub fn new(typ: ObjectType, offset: usize, size: usize, label: Option<ObjectLabel>) -> Object {
        return Object {
            typ,
            offset,
            size,
            label,
        };
    }

    pub fn typ(self) -> ObjectType {
        return self.typ;
    }

    pub fn offset(&self) -> usize {
        return self.offset;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }
}

pub struct ObjectBuilder {
    label: Option<ObjectLabel>,
    typ: Option<ObjectType>,
    pub offset: Option<usize>,
    size: Option<usize>,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        return ObjectBuilder {
            label: None,
            typ: None,
            offset: None,
            size: None,
        };
    }

    pub fn label(mut self, label: ObjectLabel) -> Self {
        self.label = Some(label);
        return self;
    }

    pub fn typ(mut self, typ: ObjectType) -> Self {
        self.typ = Some(typ);
        return self;
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        return self;
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        return self;
    }

    pub fn build(self) -> Result<Object, ObjectError> {
        return Ok(Object::new(
            self.typ.ok_or(ObjectError::General(
                "Object type must be specified".to_string(),
            ))?,
            self.offset.ok_or(ObjectError::General(
                "Object offset must be specified".to_string(),
            ))?,
            self.size.ok_or(ObjectError::General(
                "Object size must be specified".to_string(),
            ))?,
            self.label,
        ));
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
