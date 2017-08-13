pub fn is_background_process<'a>(argv: &'a [&str]) -> bool {
    if argv.len() == 0 {
        false 
    } else {
        argv[argv.len() - 1] == "&"
    }
}

