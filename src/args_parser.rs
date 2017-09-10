//Checks if process should be executed in the background and returns the (un)modified cmd + boolean as result
pub fn check_background_process<'a>(argv: &'a [&str]) -> (&'a [&'a str], bool) {
    let last_index = argv.len() - 1;
    if argv.len() > 0 && argv[last_index] == "&" {
        (&argv[0..last_index], true)
    } else {
        (argv, false)
    }
}

pub fn check_redirect<'a>(argv: &'a [&str]) -> (Option<(&'a str, bool)>, &'a [&'a str]) {
    let mut idx = 0;

    for i in 0..argv.len() {
        if argv[i] == ">" || argv[i] == "<" {
            idx = i;
            break;
        }
    }

    if idx != 0 {
        if argv[idx] == ">" { 
            (Some((argv[idx+1], false)), &argv[0..idx])
        } else {
            (Some((argv[idx+1], true)), &argv[0..idx])
        }
    } else {
        (None, argv)
    }
}
