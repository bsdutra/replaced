mod arguments;
mod replace;

use crate::arguments::arguments::parse_arguments;
use crate::replace::replace::replace_data;
use std::fs;
use std::process::exit;

fn main() {
    let args = parse_arguments();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to read data from file '{}': '{:?}'", args.filename, e);
            exit(1);
        }
    };

    let replaced_data = match replace_data(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to replace text: {:?}", e);
            exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to write to file '{}': '{:?}'", args.filename, e);
            exit(1);
        }
    }
}
