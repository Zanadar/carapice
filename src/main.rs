use std::env;
use std::io::{self, Read, Write};
use std::os::unix::process::CommandExt;
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

            if bin == "exec" {
                let bin = cmd.split_whitespace().nth(1).unwrap();
                let args: Vec<_> = cmd.split_whitespace().skip(2).collect();

                Command::new(bin).args(&args).exec();
            }
            // show output
            let args: Vec<_> = cmd.split_whitespace().skip(1).collect();
            let output = Command::new(&bin).args(&args).output().unwrap();
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

// Statement => x;x
// cmd args sep

enum Token {
    Word,
    Op,
}

//
