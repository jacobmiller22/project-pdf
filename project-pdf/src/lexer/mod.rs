use std::fmt::{self, Display};
// type PDFProcessingError= ();
type Result<T> = std::result::Result<T, PDFProcessingError>;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, Clone)]
pub struct PDFProcessingError;

impl fmt::Display for PDFProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PDFProcessingError")
    }
}

// impl Error for PDFProcessingError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }

// fn from(value: Error) -> Self {
//     impl From<Error> for PDFProcessingError {
//         PDFProcessingError {
//             details: "".to_string(),
//         }
//     }
// }
//

trait Is<T> {
    fn is(v: T) -> bool;
}

#[derive(Debug)]
#[repr(u8)]
enum Whitespace {
    Null = 0,
    HorizontalTab = 9,
    LineFeed = 10,
    FormFeed = 12,
    CarriageReturn = 13,
    Space = 32,
}

impl Is<&u8> for Whitespace {
    fn is(v: &u8) -> bool {
        return Whitespace::try_from(v).is_ok();
    }
}

impl TryFrom<&u8> for Whitespace {
    type Error = ();

    fn try_from(value: &u8) -> std::result::Result<Whitespace, ()> {
        match value {
            0 => Ok(Whitespace::Null),
            9 => Ok(Whitespace::HorizontalTab),
            10 => Ok(Whitespace::LineFeed),
            12 => Ok(Whitespace::FormFeed),
            13 => Ok(Whitespace::CarriageReturn),
            32 => Ok(Whitespace::Space),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
enum Delimiter {
    //---------------------// Glyph:
    LeftParen = 40,        // (
    RightParen = 41,       // (
    LeftAngleBrack = 60,   // <
    RightAngleBrack = 62,  // >
    LeftSquareBrack = 91,  // [
    RightSquareBrack = 93, // ]
    LeftCurlyBrack = 123,  // {
    RightCurlyBrack = 125, // }
    Solidus = 47,          // /
    PercentSign = 37,      // %
}

impl Is<&u8> for Delimiter {
    fn is(v: &u8) -> bool {
        return Delimiter::try_from(v).is_ok();
    }
}

impl TryFrom<&u8> for Delimiter {
    type Error = ();

    fn try_from(value: &u8) -> std::result::Result<Delimiter, ()> {
        match value {
            40 => Ok(Delimiter::LeftParen),
            41 => Ok(Delimiter::RightParen),
            60 => Ok(Delimiter::LeftAngleBrack),
            62 => Ok(Delimiter::RightAngleBrack),
            91 => Ok(Delimiter::LeftSquareBrack),
            93 => Ok(Delimiter::RightSquareBrack),
            123 => Ok(Delimiter::LeftCurlyBrack),
            125 => Ok(Delimiter::RightCurlyBrack),
            47 => Ok(Delimiter::Solidus),
            37 => Ok(Delimiter::PercentSign),
            _ => Err(()),
        }
    }
}

enum Keyword {
    True,
    False,
    Obj,
    Endobj,
    Null,
    Stream,
    Endstream,
    R,
    Xref,
    Trailer,
    N,
    F,
    Startxref,
}

impl Is<&[u8]> for Delimiter {
    fn is(v: &[u8]) -> bool {
        return Keyword::try_from(v).is_ok();
    }
}

impl TryFrom<&[u8]> for Keyword {
    type Error = ();
    fn try_from(value: &[u8]) -> std::result::Result<Self, ()> {
        match value {
            b"true" => Ok(Keyword::True),
            b"false" => Ok(Keyword::False),
            b"obj" => Ok(Keyword::Obj),
            b"endobj" => Ok(Keyword::Endobj),
            b"null" => Ok(Keyword::Null),
            b"stream" => Ok(Keyword::Stream),
            b"endstream" => Ok(Keyword::Endstream),
            b"r" => Ok(Keyword::R),
            b"xref" => Ok(Keyword::Xref),
            b"trailer" => Ok(Keyword::Trailer),
            b"n" => Ok(Keyword::N),
            b"f" => Ok(Keyword::F),
            b"startxref" => Ok(Keyword::Startxref),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum TokenType {
    Delimiter(Delimiter),
    Whitespace(Whitespace),
    Regular,
}

enum NumericObject {
    Integer,
    Real,
}

enum StringObject {
    Literal,
    Hexadecimal,
}

enum Object {
    Boolean,
    Numeric(NumericObject),
    String(StringObject),
    Name,
    Array,
    Dictionary,
    Stream,
    Null,
}

#[derive(Debug)]
pub struct Token<'a> {
    lexeme: &'a [u8],
    offset: usize,
    typ: TokenType,
}

pub struct Lexer<'a> {
    p: usize,
    buf: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new(buf: &[u8]) -> Lexer {
        return Lexer { p: 0, buf };
    }

    /// Grabs the next token and moves pointer p forward
    pub fn next(&mut self) -> Result<Token> {
        let tok = self.next_token()?;

        self.p = tok.offset + tok.lexeme.len();

        return Ok(tok);
    }

    /// Grabs the next token based on the pointer p
    ///
    fn next_token(&self) -> Result<Token<'a>> {
        // 4 Cases
        //
        //  1. Starts with EOF
        //      idk
        //  2. Starts with Whitespace
        //      Skip, the skip can be inferred later
        //  3. Starts with Delimiter
        //      Return Delimiter Token
        //  4. Starts with Regular
        //      Loop at p+1 until non regular char is reached (EOF, Delimiter, Whitespace),
        //      incrementing token if regular character.
        //      Return Regular token
        //

        if self.p >= self.buf.len() {
            println!("next_token p == len");
            return Err(PDFProcessingError);
        }

        let mut start = self.p;

        // Skip whitespace
        while start < self.buf.len() && Whitespace::is(&self.buf[start]) {
            start += 1;
        }

        if start == self.buf.len() {
            return Ok(Token {
                lexeme: &self.buf[self.p..start],
                offset: self.p,
                typ: TokenType::Whitespace(Whitespace::FormFeed),
            });
        }

        // Check for delimiter
        match Delimiter::try_from(&self.buf[start]) {
            Ok(delim) => {
                return Ok(Token {
                    lexeme: &self.buf[start..start + 1],
                    offset: start,
                    typ: TokenType::Delimiter(delim),
                });
            }
            Err(_) => (), // nop
        };

        // Regular, Increment end until EOF, Delimiter, or Whitespace.
        let mut end = start + 1;

        while end < self.buf.len() {
            if Whitespace::is(&self.buf[end]) || Delimiter::is(&self.buf[end]) {
                break;
            }
            end += 1;
        }
        return Ok(Token {
            lexeme: &self.buf[start..end],
            offset: start,
            typ: TokenType::Regular,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn creates_tokens() {
        let data = b"%PDF-1.7
<</DecodeParms<</Columns 5/Predictor 12>>/Filter/FlateDecode/ID[<2B551D2AFE52654494F9720283CFF1C4><564250FCF6F74BD99ACAC7DAC72EBAC4>]/Index[90856 1006]/Info 90855 0 R/Length 176/Prev 14751032/Root 90857 0 R/Size 91862/Type/XRef/W[1 3 1]>>";

        let mut lexer = Lexer { buf: data, p: 0 };

        while let Ok(tok) = lexer.next() {
            println!("lexer.next_token(): {:?}", tok);
        }
    }
}
