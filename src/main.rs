use std::io::{stdout, Write, stdin};

fn main() {
    print!(">> ");
    stdout().flush().unwrap();
    let mut cmd = String::new();
    stdin().read_line(&mut cmd).unwrap();
    descript::run(cmd.as_str());
}