use std::process;

fn main() {
    if let Err(e) = bulls_and_cows::run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}
