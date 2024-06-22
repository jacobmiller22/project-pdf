use project_pdf;
use std::{
    env,
    fs::File,
    io::{self, Read},
};
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments!");
    }
    if args.len() > 2 {
        panic!("Too many arguments. Expected 2.");
    }

    let path = args.get(1).expect("Arg length checks insufficient!");

    let mut fd = File::open(path)?;

    let mut buf: Vec<u8> = Vec::new();

    fd.read_to_end(&mut buf)?;

    let pdf_reader = project_pdf::PDFReader::from(buf);

    pdf_reader.parse();

    return Ok(());
}
