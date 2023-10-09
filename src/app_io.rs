use std::process;

pub fn stderr(msg: &str) {
    eprintln!("{}", msg);
    process::exit(1);
}

pub fn stdout(msg: &str) {
    println!("{}", msg);
    process::exit(0);
}
