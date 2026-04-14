use rustyline::{DefaultEditor, Result, error::ReadlineError};

use crate::{lexer::Lexer, parser::Parser};

pub fn start_repl() -> Result<()> {
    println!("Monke smart! Make read, Monke do!");
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let l = Lexer::new(line.to_string());
                let mut p = Parser::new(l);
                let mut program = p.parse_program();
                if !p.errors.is_empty() {
                    print_parser_errors(&p);
                } else {
                    println!("{}", program.string());
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}

fn print_parser_errors(parser: &Parser) {
    for msg in parser.errors.iter() {
        eprintln!("\t {}", msg);
    }
}