use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter a path");
        return;
    }

    let mut show_ends = false;

    for arg in &args[1.. ] {
        if arg == "-E" {
            show_ends = true;
        }
    }

    for arg in &args[1.. ] {
        if arg.chars().nth(0).unwrap() != '-' {
                let file = fs::File::open(arg).expect("Please enter a valid file");
                let buf_reader = io::BufReader::new(file);

                for line in buf_reader.lines() {
                    match line {
                        Ok(mut line) => {
                            if show_ends == true {
                                line = line + "$";
                            }

                            println!("{line}")
                        },
                        Err(e) => println!("{e}"),
                    }
                }
        }
    }
}
