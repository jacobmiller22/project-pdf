use std::convert::From;

pub mod lexer;
pub mod object;

pub struct PDFReader {
    data: Vec<u8>,
}

impl From<Vec<u8>> for PDFReader {
    fn from(bytes: Vec<u8>) -> Self {
        return PDFReader { data: bytes };
    }
}

// impl From<&[u8]> for PDFReader {
//     fn from(bytes: &[u8]) -> Self {
//         return PDFReader {
//             data: bytes.to_vec(),
//             lexer: lexer::Lexer::new(&bytes),
//         };
//     }
// }

impl PDFReader {
    pub fn new(data: Vec<u8>) -> PDFReader {
        return PDFReader { data };
    }

    pub fn parse(self) {
        let mut lexer = lexer::Lexer::new(&self.data);

        let mut c = 0;
        while let Ok(tok) = lexer.next() {
            println!("lexer.next_token(): {:?}", tok);
            c += 1;
            if c > 20 {
                break;
            }
        }
        println!("{} tokens!", c)
    }

    // fn parse(mut self) {
    //     let bp = self.parse_header();
    //     let tp = self.parse_body(bp);
    // }
    //
    // fn parse_header(self) -> Result<usize> {
    //     // def Character: A sequence of 8-bit bytes
    //
    //     // 7.5.2 PDF 2.0 Specification:
    //     // The PDF file begins with the 5 characters “%PDF–” and byte offsets shall be calculated from the
    //     // PERCENT SIGN (25h).
    //
    //     let mut p = 0;
    //
    //     // Move forward past initial bytes until we find the '%'
    //     while self.data.len() - 5 <= p || &self.data[p..p + 5] == b"%PDF-" {
    //         p += 1
    //     }
    //
    //     if &self.data[p..p + 5] != b"%PDF-" {
    //         return Err(PDFProcessingError);
    //     }
    //
    //     p += 5;
    //
    //     p += match &self.data[p + 1..p + 2] {
    //         b"1." => Ok(2),
    //         b"2." => Ok(2),
    //         _ => Err(PDFProcessingError),
    //     }?;
    //
    //     p += match &self.data[p + 1] {
    //         b'0'..=b'9' => Ok(1),
    //         _ => Err(PDFProcessingError),
    //     }?;
    //
    //     p += match Whitespace::try_from(&self.data[p + 1]).map_err(|()| PDFProcessingError)? {
    //         Whitespace::CarriageReturn => {
    //             match Whitespace::try_from(&self.data[p + 2]).map_err(|()| PDFProcessingError)? {
    //                 Whitespace::LineFeed => Ok(2),
    //                 _ => Ok(1),
    //             }
    //         }
    //         Whitespace::LineFeed => Ok(1),
    //         _ => Err(PDFProcessingError),
    //     }?;
    //
    //     return Ok(p);
    // }
    //
    // fn parse_body(mut self, bp: usize) -> Result<usize> {}
}

// struct PDFReaderBuilder<'a> {
//     data: Vec<u8>,
//     lexer: Option<lexer::Lexer<'a>>,
// }
//
// impl<'a> PDFReaderBuilder<'a> {
//     fn new(data: Vec<u8>) -> PDFReaderBuilder<'a> {
//         return PDFReaderBuilder { data, lexer: None };
//     }
//
//     fn data(mut self, data: Vec<u8>) -> PDFReaderBuilder<'a> {
//         self.data = data;
//         return self;
//     }
//
//     fn build(self) -> PDFReader<'a> {
//         return PDFReader {
//             data: self.data,
//             lexer: lexer::Lexer::new(&self.data),
//         };
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
