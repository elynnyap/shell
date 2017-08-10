//
// gash.rs
//
// Starting code for PS2
// Running on Rust 1+
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4
//

extern crate shell;
extern crate getopts; // Rust library for parsing CLI options
// https://doc.rust-lang.org/getopts/getopts/index.html

use shell::CircularBuffer;
use getopts::Options;
use std::env;
use std::io::{self, Write};
use std::process::Command;

struct Shell<'a> {
    cmd_prompt: &'a str,
    history: CircularBuffer,
}

impl <'a>Shell<'a> {
    fn new(prompt_str: &'a str) -> Shell<'a> {
        Shell { 
            cmd_prompt: prompt_str, 
            history: CircularBuffer::new()
        }
    }

    fn run(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            stdout.write(self.cmd_prompt.as_bytes()).unwrap();
            stdout.flush().unwrap();

            let mut line = String::new();

            stdin.read_line(&mut line).unwrap();
            let cmd_line = line.trim();
            let program = cmd_line.splitn(1, ' ').nth(0).expect("no program");

            // record the newly-entered command in the history
            self.history.write(String::from(cmd_line));

            match program {
                ""      =>  { continue; }
                "exit"  =>  { return; }
                "history" => { self.print_history(); }
                _       =>  { self.run_cmdline(cmd_line); }
            }
        }
    }

    // Iterate through the 10 most recently entered commands and print them out
    fn print_history(&self) {
       self.history.print_all(); 
    }
    

    fn run_cmdline(&self, cmd_line: &str) {
        let argv: Vec<&str> = cmd_line.split(' ').filter_map(|x| {
            if x == "" {
                None
            } else {
                Some(x)
            }
        }).collect();

        match argv.first() {
            Some(&program) => self.run_cmd(program, &argv[1..]),
            None => (),
        };
    }

    fn run_cmd(&self, program: &str, argv: &[&str]) {
        if self.cmd_exists(program) {
            io::stdout().write(&Command::new(program).args(argv).output().unwrap().stdout).unwrap();
        } else {
            println!("{}: command not found", program);
        }
    }

    fn cmd_exists(&self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }
}

fn get_cmdline_from_args() -> Option<String> {
    /* Begin processing program arguments and initiate the parameters. */
    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("c", "", "", "");

    opts.parse(&args[1..]).unwrap().opt_str("c")
}

fn main() {
    let opt_cmd_line = get_cmdline_from_args();

    match opt_cmd_line {
        Some(cmd_line) => Shell::new("").run_cmdline(&cmd_line),
        None           => Shell::new("gash > ").run(),
    }
}
