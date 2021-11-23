#[path= "./basics/datatypes.rs"] mod datatypes;
#[path= "./basics/printing.rs"] mod printing;

use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    datatypes::run();
    printing::run();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}