//Checks if process should be executed in the background and returns the (un)modified cmd + boolean as result
pub fn check_background_process<'a>(argv: &'a [&str]) -> (&'a [&'a str], bool) {
    let last_index = argv.len() - 1;
    if argv.len() > 0 && argv[last_index] == "&" {
        (&argv[0..last_index], true)
    } else {
        (argv, false)
    }
}

fn check_redirect<'a>(is_in: bool, argv: &'a [&str]) -> (Option<&'a str>, &'a [&'a str]) {
    let mut sym = ">";
    if is_in { sym = "<"; }

    let mut idx = 0;

    for i in 0..argv.len() {
        if argv[i] == sym {
            idx = i;
            break;
        }
    }

    if idx != 0 {
        (Some(argv[idx+1]), &argv[0..idx])
    } else {
        (None, argv)
    }
}


// Checks if process stdin source should be redirected
pub fn check_redirect_in<'a>(argv: &'a [&str]) -> (Option<&'a str>, &'a [&'a str]) {
    check_redirect(true, argv)
}

// Checks if process stdout source should be redirected
pub fn check_redirect_out<'a>(argv: &'a [&str]) -> (Option<&'a str>, &'a [&'a str]) {
    check_redirect(false, argv)
}
