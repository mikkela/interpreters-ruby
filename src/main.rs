mod lexer;
mod token;
mod monkey;
mod basic;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    // This line creates an Editor with the default configuration options.
    let mut repl = DefaultEditor::new()?;
    // This if statement loads a file with the history of commands
    // If the file does not exists, it creates one.
    #[cfg(feature = "with-file-history")]
    if repl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    // This is our infinite loop. We will be here until the user terminates the program.
    loop {
        // This line asks the user to input a command. You can add whatever you want in here as a prefix.
        let readline = repl.readline(">> ");

        // The readline method returns an Result. Which we now use a match statement to filter the result.
        match readline {
            Ok(line) => {
                repl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // Here we are saving the commands into the file. Until now they are stored in memory.
    #[cfg(feature = "with-file-history")]
    repl.save_history("history.txt").unwrap();
    Ok(())
}
