// Starting code credited to:
// University of Virginia - cs4414 Spring 2014
// Weilin Xu, David Evans
// Version 0.4

extern crate shell;
extern crate nix;
extern crate rand;

use nix::libc::c_int;
use nix::sys::signal::{kill, Signal, SigHandler};
use nix::sys::signal;
use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult, execvp, Pid, setpgid, dup2, close};
use rand::Rng;
use shell::args_parser;
use shell::circular_buffer::CircularBuffer;
use shell::spells::get_spell;
use shell::magic;
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::{self, Write};
use std::os::unix::io::{RawFd, IntoRawFd};
use std::path::Path;
use std::process::Command;

static mut FG_PID: Option<Pid> = None;

const HISTORY_SIZE: usize = 10;
const STDIN_FILENO: RawFd = 0;
const STDOUT_FILENO: RawFd = 1;

struct Shell<'a> {
    cmd_prompt: &'a str, 
    history: CircularBuffer<String>,
    error_msgs: Vec<&'static str>
}

#[allow(unused_must_use)]
impl <'a>Shell<'a> {
    fn new(prompt_str: &'a str) -> Shell<'a> {
        Shell { 
            cmd_prompt: prompt_str, 
            history: CircularBuffer::new(HISTORY_SIZE),
            error_msgs: magic::get_error_msgs()
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
            
            line = line.trim().to_lowercase(); 

            if line == "i solemnly swear that i am up to no good" { break; }
        }

        let welcome_msg = "
        ****************************************************
        Messrs Moony, Wormtail, Padfoot and Prongs \n
        Purveyors of Aids to Magical Mischief-Makers \n
        are proud to present...\n
        ****************************************************
        \n"; 

        stdout.write(welcome_msg.as_bytes()).unwrap();
        stdout.flush().unwrap();

        loop {
            stdout.write(self.cmd_prompt.as_bytes()).unwrap();
            stdout.flush().unwrap();

            let mut line = String::new();
            stdin.read_line(&mut line).unwrap(); 
            let cmd_line = line.trim();
            self.history.write(String::from(cmd_line));
            
            if cmd_line.to_lowercase() == "mischief managed" { return; }
            if cmd_line.to_lowercase() == "prior incantato" { 
                self.print_history();
                continue;
            }

            let program = cmd_line.splitn(1, ' ').nth(0).expect("no program");
            match program {
                ""      =>  { continue; }
                _       =>  { self.run_cmdline(cmd_line); }
            }
        }
    }

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
            None => (),
            Some(&program) => {
                match program {
                    "apparate" => {
                        if argv.len() == 1 {
                            println!("No directory specified.");
                        } else {
                            self.run_cd(&argv[1]);
                        }
                    },
                    _ => self.run_cmd(program, &argv),
                }
            }
        };
    }

    /* Change directory */
    fn run_cd(&self, new_dir: &str) {
        let path = Path::new(new_dir);
        if path.exists() {
            if env::set_current_dir(&path).is_err() {
                println!("Unable to apparate. Are you sure you got your licence?");
            }
        } else {
            println!("Destination does not exist. You just got splinched.");
        }
    }

    fn run_cmd(&self, program: &str, argv: &[&str]) {
        let spell = get_spell(program);

        match spell {
            Some(p) => {
                if !self.cmd_exists(p) { return; };
                let (argv, is_bg_process) = args_parser::check_background_process(argv);
                if is_bg_process {
                    match fork() {
                        Ok(ForkResult::Parent { .. }) => {}, // Parent does not wait for bg process
                        Ok(ForkResult::Child) => {
                            setpgid(Pid::from_raw(0), Pid::from_raw(0)); // Put the child into a different process group
                            self.run_cmd_child(p, argv);
                        },
                        Err(_) => { println!("Fork failed"); }
                    }
                } else {
                    match fork() {
                        Ok(ForkResult::Parent { child, .. }) => {
                            // Record child process as the currently-running foreground process
                            unsafe { FG_PID = Some(child); }
                            waitpid(child, None);
                            // child process has completed, so clear FG_PID
                            unsafe { FG_PID = None;
                            } },
                        Ok(ForkResult::Child) => { self.run_cmd_child(p, argv); },
                        Err(_) => { println!("Fork failed"); }
                    }
                }
            },
            None => {
                println!("{}: spell failed!", program);
                println!("{}", rand::thread_rng().choose(&self.error_msgs).unwrap());
            }
        } 
    }

    fn run_cmd_child(&self, program: &str, argv: &[&str]) {
        let (redirect, argv) = args_parser::check_redirect(argv);

        match redirect {
            None => {},
            Some((src, is_redirect_in)) => {
                let fd = if is_redirect_in {
                    File::open(src).unwrap().into_raw_fd() 
                } else {
                    File::create(src).unwrap().into_raw_fd()
                };
                
                let dest = if is_redirect_in { STDIN_FILENO } else { STDOUT_FILENO };

                match dup2(fd, dest) { 
                    Ok(_) => {},
                    Err(e) => println!("Error occurred during redirection. {:?}", e)
                }
                close(fd); 
            }
        };

        let argv_cstring: Vec<CString> = argv.into_iter().map( |slice| CString::new(*slice).unwrap()).collect();
        execvp(&CString::new(program).unwrap(), &argv_cstring);
    }

    fn cmd_exists(&self, cmd_path: &str) -> bool {
        Command::new("which").arg(cmd_path).status().unwrap().success()
    }
}

#[allow(unused_must_use)]
extern "C" fn handle_sigint(signal_num: c_int) {
    unsafe {
        match FG_PID {
            None => {}, 
            Some(pid) => { 
                kill(pid, Signal::from_c_int(signal_num).ok()); 
            }
        };
    }
}

#[allow(unused_must_use)]
fn main() {
    let sig_action = signal::SigAction::new(SigHandler::Handler(handle_sigint),
                                          signal::SaFlags::empty(),
                                          signal::SigSet::empty());
    unsafe {
        signal::sigaction(signal::SIGINT, &sig_action);
    }

    Shell::new("swish > ").run();
}
