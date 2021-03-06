extern crate cargo_binutils as cbu;

use std::process;

use crate::cbu::Tool;

fn main() {
    match cbu::run(Tool::Profdata, None) {
        Err(e) => eprintln!("error: {}", e),
        Ok(ec) => process::exit(ec),
    }
}
