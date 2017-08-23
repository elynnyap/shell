//Checks if process should be executed in the background and returns the (un)modified cmd + boolean as result
pub fn check_background_process<'a>(argv: &'a [&str]) -> (&'a [&'a str], bool) {
    if argv.len() == 0 {
        (argv, false) 
    } else {
        let last_idx = argv.len() - 1;
        if argv[last_idx] == "&" {
            (&argv[0..last_idx], true)
        } else {
            (argv, false)
        }
    }
}

