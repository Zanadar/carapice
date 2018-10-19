use std::env;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;
use std::process::Command;
extern crate regex;

use regex::Regex;

fn main() -> io::Result<()> {
    loop {
        io::stdout().write_all(b"> ")?;
        io::stdout().flush()?;
        // read line from standard input
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        // "parse" line into executable and args
        let re = regex::Regex::new(r";|&&|\|\|").unwrap();

        for cmd in re.split(&buf) {
            let bin: String = cmd.split_whitespace().take(1).collect();

            // run in seperate process
            if bin == "cd" {
                let dest: String = cmd.split_whitespace().skip(1).collect();
                let dest = Path::new(&dest);
                env::set_current_dir(&dest).ok();
                continue;
            }

            if bin == "exit" {
                let exit: i32 = match cmd.split_whitespace().nth(1) {
                    Some(code) => code.parse().unwrap(),
                    None => 0,
                };

                process::exit(exit);
            }

            let args: Vec<_> = cmd.split_whitespace().skip(1).collect();
            let output = Command::new(bin).args(&args).output()?;
            // show output
            println!("{}", String::from_utf8_lossy(&output.stdout))
        }
    }

    Ok(())
}

struct Cmd {}

impl Cmd {
    fn new(stdin: String) -> Cmd {
        Cmd {}
    }
}

enum Token<'a> {
    Cmd(String),
    Args(Vec<&'a str>),
    Semi,
    And,
    Or,
}
