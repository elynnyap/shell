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

fn check_redirect<'a>(is_in: bool, argv: &'a [&str]) -> (Option<&'a str>, &'a [&'a str]) {
    let mut sym = ">";
    if is_in {
        sym = "<";
    }

    let mut found_redirect = false;
    let mut idx = 0;

    for i in 0..argv.len() {
        if argv[i] == sym {
            found_redirect = true;
            idx = i;
            break;
        }
    }

    if found_redirect {
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
