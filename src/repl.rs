use rustyline::{DefaultEditor, Result, error::ReadlineError};

use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

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
                let mut l = Lexer::new(line.to_string());
                let mut next_token: Token;
                loop {
                    next_token = l.next_token();

                    if next_token.t_type == TokenType::Eof {
                        break;
                    }
                    println!("{:?}", next_token);
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
