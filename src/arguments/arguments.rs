use std::env;
use std::process::exit;

#[derive(Debug)]
pub struct Arguments {
    pub target: String,
    pub replacement: String,
    pub filename: String,
    pub output: String,
}

pub fn parse_arguments() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    // Debug -> println!("{:?}", args);

    if args.len() != 4 {
        eprintln!("Number of arguments: expected 4 got {}", args.len());
        exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}

