Rust Shell
==========

A simple shell for Unix-based systems.

To run, type ```cargo build && /target/debug/main```.

Running with ```cargo run``` will cause the signal handling within the shell to break due to the fact that Cargo sets up its own handlers that interfere with ctrl+c (reference [one](https://github.com/Detegr/rust-ctrlc/issues/15) and [two](https://www.reddit.com/r/rust/comments/6lsead/problems_with_ctrlc_handling_under_rust_in_windows/)). 

Features:
* cd
* history (last 10 commands)
* signal handling for ctrl+c
* background processes
* i/o redirection

In progress:
* pipes

## Credits
* [David Evans' Rust Class](http://www.rust-class.org/pages/ps2.html)
