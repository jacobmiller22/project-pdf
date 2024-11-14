use std::fmt::{self, Display};
// type PDFProcessingError= ();
type Result<T> = std::result::Result<T, PDFProcessingError>;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub enum PDFProcessingError {
    EOF(String),
    General(String), // TODO: Remove this and replace with more concrete. For now, general error
}

impl fmt::Display for PDFProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PDFProcessingError::EOF(ref err) => write!(f, "Reached EOF: {}", err),
            PDFProcessingError::General(ref err) => write!(f, "PDFProcessingError: {}", err),
        }
    }
}

impl std::error::Error for PDFProcessingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            // Error cases involving std::error go here
            _ => None,
        }
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

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WhitespaceType {
    Null = 0,
    HorizontalTab = 9,
    LineFeed = 10,
    FormFeed = 12,
    CarriageReturn = 13,
    Space = 32,
}

impl Is<&u8> for WhitespaceType {
    fn is(v: &u8) -> bool {
        return WhitespaceType::try_from(v).is_ok();
    }
}

impl TryFrom<&u8> for WhitespaceType {
    type Error = ();

    fn try_from(value: &u8) -> std::result::Result<WhitespaceType, ()> {
        match value {
            0 => Ok(WhitespaceType::Null),
            9 => Ok(WhitespaceType::HorizontalTab),
            10 => Ok(WhitespaceType::LineFeed),
            12 => Ok(WhitespaceType::FormFeed),
            13 => Ok(WhitespaceType::CarriageReturn),
            32 => Ok(WhitespaceType::Space),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DelimiterType {
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

impl Is<&u8> for DelimiterType {
    fn is(v: &u8) -> bool {
        return DelimiterType::try_from(v).is_ok();
    }
}

impl TryFrom<&u8> for DelimiterType {
    type Error = ();

    fn try_from(value: &u8) -> std::result::Result<DelimiterType, ()> {
        match value {
            40 => Ok(DelimiterType::LeftParen),
            41 => Ok(DelimiterType::RightParen),
            60 => Ok(DelimiterType::LeftAngleBrack),
            62 => Ok(DelimiterType::RightAngleBrack),
            91 => Ok(DelimiterType::LeftSquareBrack),
            93 => Ok(DelimiterType::RightSquareBrack),
            123 => Ok(DelimiterType::LeftCurlyBrack),
            125 => Ok(DelimiterType::RightCurlyBrack),
            47 => Ok(DelimiterType::Solidus),
            37 => Ok(DelimiterType::PercentSign),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum KeywordType {
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

impl Is<&[u8]> for DelimiterType {
    fn is(v: &[u8]) -> bool {
        return KeywordType::try_from(v).is_ok();
    }
}

impl TryFrom<&[u8]> for KeywordType {
    type Error = ();
    fn try_from(value: &[u8]) -> std::result::Result<Self, ()> {
        match value {
            b"true" => Ok(KeywordType::True),
            b"false" => Ok(KeywordType::False),
            b"obj" => Ok(KeywordType::Obj),
            b"endobj" => Ok(KeywordType::Endobj),
            b"null" => Ok(KeywordType::Null),
            b"stream" => Ok(KeywordType::Stream),
            b"endstream" => Ok(KeywordType::Endstream),
            b"r" => Ok(KeywordType::R),
            b"xref" => Ok(KeywordType::Xref),
            b"trailer" => Ok(KeywordType::Trailer),
            b"n" => Ok(KeywordType::N),
            b"f" => Ok(KeywordType::F),
            b"startxref" => Ok(KeywordType::Startxref),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Delimiter(DelimiterType),
    Whitespace(WhitespaceType),
    Keyword(KeywordType),
    Regular,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub lexeme: &'a [u8],
    pub offset: usize,
    pub typ: TokenType,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert lexeme (which is a byte slice) to a string for display
        let lexeme_str = match std::str::from_utf8(self.lexeme) {
            Ok(v) => v,
            Err(_) => "<invalid UTF-8>",
        };

        write!(
            f,
            "Token {{ lexeme: '{}', offset: {}, type: {:?} }}",
            lexeme_str, self.offset, self.typ
        )
    }
}

pub struct Lexer {
    p: usize,
}

impl<'a> Lexer {
    pub fn new() -> Lexer {
        return Lexer { p: 0 };
    }

    /// Move the position of the lexer
    pub fn setp(&mut self, p: usize) {
        self.p = p
    }

    /// Get the position of the lexer
    pub fn getp(&self) -> usize {
        return self.p;
    }

    /// Grabs the next token and moves pointer p forward
    pub fn next(&mut self, buf: &'a [u8]) -> Result<Token<'a>> {
        let tok = self.peek(buf)?;
        self.p = tok.offset + tok.lexeme.len();
        return Ok(tok);
    }

    /// Grabs the next token based on the pointer p
    ///
    pub fn peek(&self, buf: &'a [u8]) -> Result<Token<'a>> {
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
        //  5. Token is a  keyword
        //      return a keyword token
        //  6.
        //      Return Regular token

        if self.p >= buf.len() {
            return Err(PDFProcessingError::EOF("".to_string()));
        }

        let mut start = self.p;

        // Skip literal whitespace
        while start < buf.len()
            // && WhitespaceType::is(&buf[start])
            && buf[start] == (WhitespaceType::Space as u8)
        {
            start += 1;
        }

        if start == buf.len() {
            return Ok(Token {
                lexeme: &buf[self.p..start],
                offset: self.p,
                typ: TokenType::Whitespace(WhitespaceType::FormFeed),
            });
        }

        // Check for WhitespaceType
        match WhitespaceType::try_from(&buf[start]) {
            Ok(whitespace) => {
                return Ok(Token {
                    lexeme: &buf[start..start + 1],
                    offset: start,
                    typ: TokenType::Whitespace(whitespace),
                });
            }
            Err(_) => (), // nop
        };

        // Check for delimiter
        match DelimiterType::try_from(&buf[start]) {
            Ok(delim) => {
                return Ok(Token {
                    lexeme: &buf[start..start + 1],
                    offset: start,
                    typ: TokenType::Delimiter(delim),
                });
            }
            Err(_) => (), // nop
        };

        // Increment end until EOF, Delimiter, or Whitespace.
        let mut end = start + 1;

        while end < buf.len() {
            if WhitespaceType::is(&buf[end]) || DelimiterType::is(&buf[end]) {
                break;
            }
            end += 1;
        }

        // If keyword
        if let Ok(kw) = KeywordType::try_from(&buf[start..end]) {
            return Ok(Token {
                lexeme: &buf[start..end],
                offset: start,
                typ: TokenType::Keyword(kw),
            });
        }
        // If keyword
        return Ok(Token {
            lexeme: &buf[start..end],
            offset: start,
            typ: TokenType::Regular,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_delimiters() {
        let data = b"()<>[]{}/%";

        let mut lexer = Lexer { p: 0 };
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(token.typ, TokenType::Delimiter(DelimiterType::LeftParen));
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(token.typ, TokenType::Delimiter(DelimiterType::RightParen));
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(
            token.typ,
            TokenType::Delimiter(DelimiterType::LeftAngleBrack)
        );
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(
            token.typ,
            TokenType::Delimiter(DelimiterType::RightAngleBrack)
        );
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(
            token.typ,
            TokenType::Delimiter(DelimiterType::LeftSquareBrack)
        );
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(
            token.typ,
            TokenType::Delimiter(DelimiterType::RightSquareBrack)
        );
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(
            token.typ,
            TokenType::Delimiter(DelimiterType::LeftCurlyBrack)
        );
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(
            token.typ,
            TokenType::Delimiter(DelimiterType::RightCurlyBrack)
        );
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(token.typ, TokenType::Delimiter(DelimiterType::Solidus));
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(token.typ, TokenType::Delimiter(DelimiterType::PercentSign));
        let token = lexer.next(data);
        assert!(token.is_err())
    }

    #[test]
    fn has_correct_offsets_with_multichars() {
        let data = b"\\ \n \t \r";
        let mut lexer = Lexer { p: 0 };
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(token.offset, 0);
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(token.offset, 2);
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(token.offset, 4);
        let token = lexer.next(data).expect("Token was not ok");
        assert_eq!(token.offset, 6);
        let token = lexer.next(data);
        assert!(token.is_err())
    }
}
