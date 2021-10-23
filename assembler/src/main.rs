use std::io::{Read, Write};

use assembler::pre::parse_string;
use assembler::Result;

fn main() -> std::io::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let file = match args.len() {
        1 => None,
        2 => Some(args[1].clone()),
        _ => {
            eprintln!("usage: {} <filename>?", args[0]);
            std::process::exit(1);
        }
    };
    match do_assembler(file) {
        Ok(assembled) => {
            let mut out = std::io::stdout();
            out.write_all(&assembled)?;
            out.flush()?;
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(-1);
        }
    }

    Ok(())
}

fn do_assembler(file: Option<String>) -> Result<Vec<u8>> {
    let program = match file {
        None => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
        Some(path) => std::fs::read_to_string(path)?,
    };

    let program = parse_string(&program)?;
    let program = program.unlabel();
    Ok(program.assemble())
}
