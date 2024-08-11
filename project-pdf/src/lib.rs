use lexer::{KeywordType, PDFProcessingError};
pub mod lexer;
pub mod object;

struct PDF {
    lexer: lexer::Lexer,
}

#[allow(dead_code)]
impl PDF {
    fn header_offset<'a>(&mut self, buf: &'a [u8]) -> Result<usize, PDFProcessingError> {
        // let HPREFIX = b"%PDF-";
        // let offset = buf.windows(HPREFIX.len()).position(|w| w == HPREFIX)?
        return Ok(3);
    }

    fn parse_dictionary(
        &mut self,
        buf: &[u8],
        offset: usize,
    ) -> Result<object::Object, PDFProcessingError> {
        return Ok(object::Object::new(
            object::ObjectType::Dictionary,
            offset,
            offset,
        ));
    }

    fn parse_literalstring(
        &mut self,
        buf: &[u8],
        offset: usize,
    ) -> Result<object::Object, PDFProcessingError> {
        return Ok(object::Object::new(
            object::ObjectType::String(object::StringObjectType::Literal),
            offset,
            offset,
        ));
    }

    fn parse_hexstring(
        &mut self,
        buf: &[u8],
        offset: usize,
    ) -> Result<object::Object, PDFProcessingError> {
        return Ok(object::Object::new(
            object::ObjectType::String(object::StringObjectType::Hexadecimal),
            offset,
            offset,
        ));
    }

    fn parse_nameobject(
        &mut self,
        buf: &[u8],
        offset: usize,
    ) -> Result<object::Object, PDFProcessingError> {
        return Ok(object::Object::new(
            object::ObjectType::Name,
            offset,
            offset,
        ));
    }

    fn parse_array(
        &mut self,
        buf: &[u8],
        offset: usize,
    ) -> Result<object::Object, PDFProcessingError> {
        return Ok(object::Object::new(
            object::ObjectType::Array,
            offset,
            offset,
        ));
    }

    fn parse_numericobject(
        &mut self,
        buf: &[u8],
        offset: usize,
    ) -> Result<object::Object, PDFProcessingError> {
        return Ok(object::Object::new(
            object::ObjectType::Numeric(object::NumericObjectType::Integer),
            offset,
            offset,
        ));
    }

    fn parse_object(
        &mut self,
        buf: &[u8],
        offset: usize,
    ) -> Result<object::Object, PDFProcessingError> {
        // Literal Strings are delimited by parenthesis: (value)
        // Hex Strings are delimited by single angle brackets: <value>
        // Name Objects are introduced with a Solidus with no delimiter between solidus and encoded
        //  name: \MyNameObject
        // Arrays are delimited by square brackets: [blahblahblah]
        // Dictionaries are delimited by double angle brackets: <<blahblah>>

        let next_token = self.lexer.peek(buf)?;

        return match next_token.typ {
            lexer::TokenType::Delimiter(lexer::DelimiterType::LeftParen) => {
                self.parse_literalstring(buf, offset)
            }
            lexer::TokenType::Delimiter(lexer::DelimiterType::LeftAngleBrack) => {
                // Check for dictionary start

                // Move lexer to after the discovered '<'
                self.lexer.setp(next_token.offset + next_token.lexeme.len());
                if self.lexer.next(buf)?.typ
                    == lexer::TokenType::Delimiter(lexer::DelimiterType::LeftAngleBrack)
                {
                    // Found a dictionary
                    return self.parse_dictionary(buf, offset);
                }
                // No dictionary found, reset the lexer pointer
                self.lexer.setp(offset);
                return self.parse_hexstring(buf, offset);
            }
            lexer::TokenType::Delimiter(lexer::DelimiterType::Solidus) => {
                self.parse_nameobject(buf, offset)
            }
            lexer::TokenType::Delimiter(lexer::DelimiterType::LeftSquareBrack) => {
                self.parse_array(buf, offset)
            }
            lexer::TokenType::Keyword(lexer::KeywordType::True) => Ok(object::Object::new(
                object::ObjectType::Boolean(true),
                offset,
                offset + 4,
            )),
            lexer::TokenType::Keyword(lexer::KeywordType::False) => Ok(object::Object::new(
                object::ObjectType::Boolean(false),
                offset,
                offset + 5,
            )),
            lexer::TokenType::Regular => self.parse_numericobject(buf, offset),
            _ => Err(PDFProcessingError::General(
                "Missing xref offset at EOF".to_string(),
            )),
        };
    }

    pub fn parse_xref_offset(&mut self, buf: &[u8]) -> Result<usize, PDFProcessingError> {
        let p = buf.len() - 27;
        self.lexer.setp(p);
        // Move pointer until `startxref`
        loop {
            match self.lexer.next(buf) {
                Ok(token) => {
                    if token.typ == lexer::TokenType::Keyword(lexer::KeywordType::Startxref) {
                        break;
                    }
                }
                Err(e) => return Err(e),
            };
        }

        self.lexer.next(buf)?; // TODO: Ensure this is a newline delimiter token

        let offset_token = self.lexer.next(buf)?;

        if offset_token.typ != lexer::TokenType::Regular {
            return Err(PDFProcessingError::General(
                "Missing xref offset at EOF".to_string(),
            ));
        }

        let mut offset: usize = 0;
        for &byte in offset_token.lexeme {
            if byte < b'0' || byte > b'9' {
                return Err(PDFProcessingError::General(
                    "Offset string is invalid ascii".to_string(),
                ));
            }
            let digit = (byte - b'0') as usize;
            offset = offset
                .checked_mul(10)
                .and_then(|v| v.checked_add(digit))
                .ok_or_else(|| {
                    PDFProcessingError::General("Offset of xref table exceeds a usize".to_string())
                })?;
        }

        return Ok(offset);
    }

    fn parse_xref_table<'a>(
        &mut self,
        buf: &'a [u8],
        offset: usize,
    ) -> Result<Vec<lexer::Token<'a>>, PDFProcessingError> {
        let mut tokens = vec![];
        self.lexer.setp(offset);
        loop {
            let tok = self.lexer.next(buf)?;
            println!("TOKEN: {}", tok);
            if tok.typ == lexer::TokenType::Keyword(KeywordType::Trailer) {
                break;
            }
            tokens.push(tok)
        }
        println!("XREF TABLE TOKENS: {}", tokens.len());
        return Ok(tokens);
    }

    fn parse_xref_stream<'a>(
        &mut self,
        buf: &'a [u8],
        offset: usize,
    ) -> Result<Vec<lexer::Token<'a>>, PDFProcessingError> {
        let mut tokens = vec![];
        self.lexer.setp(offset);
        loop {
            let tok = self.lexer.next(buf)?;
            println!("TOKEN: {}", tok);
            if tok.typ == lexer::TokenType::Keyword(KeywordType::Endstream) {
                break;
            }
            tokens.push(tok)
        }
        return Ok(vec![]);
    }

    pub fn parse_xref<'a>(
        &mut self,
        buf: &'a [u8],
        offset: usize,
    ) -> Result<Vec<lexer::Token<'a>>, PDFProcessingError> {
        self.lexer.setp(offset);
        let tok = self.lexer.next(buf)?;
        // 7.5.4 Cross-reference table
        if tok.typ == lexer::TokenType::Keyword(KeywordType::Xref) {
            return self.parse_xref_table(buf, offset);
        }
        // 7.5.8 Cross-reference streams
        return self.parse_xref_stream(buf, offset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use base64::{engine::general_purpose, Engine as _};

    static  TAIL_PDF_B64: &'static str= "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";

    #[test]
    fn test_xref_offset() {
        let tail_pdf = general_purpose::STANDARD
            .decode(TAIL_PDF_B64)
            .expect("Failed to decode b64 string");

        let lxr = lexer::Lexer::new();
        let mut pdf = PDF { lexer: lxr };

        let offset = pdf
            .parse_xref_offset(&tail_pdf)
            .expect("Failed to find xref offset");
        assert_eq!(offset, 15563027)
    }

    #[test]
    fn test_parse_xref() {
        // let tail_pdf = general_purpose::STANDARD
        //     .decode(TAIL_PDF_B64)
        //     .expect("Failed to decode b64 string");

        // Replace "path_to_your_file.bin" with the path to your binary file
        let file_path = "data/example2-pdf";

        // Read the file contents into a Vec<u8>
        let content = fs::read(file_path).expect("Failed to read file");
        let slice = content.as_slice();

        let lxr = lexer::Lexer::new();
        let mut pdf = PDF { lexer: lxr };

        let xref_offset = pdf
            .parse_xref_offset(&slice)
            .expect("Failed to find xref offset");

        println!("CALLING PARSE_XREF!\n\n");
        let tokens = pdf
            .parse_xref(&slice, xref_offset)
            .expect("Failed to parse xref_section");
        for tok in tokens {
            println!("XRef Token: {}", tok)
        }
    }
}
