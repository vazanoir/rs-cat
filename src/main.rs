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

    // Arguments
    let mut show_ends = false;
    let mut number = false;

    for arg in &args[1..] {
        // Handle short and combined arguments
        if arg.chars().nth(1).unwrap() != '-' {
            for short_arg in arg.chars() {
                if short_arg == 'E' {
                    show_ends = true;
                }

                if short_arg == 'n' {
                    number = true;
                }
            }
        }

        // Long arguments
        if arg == "--show-ends" {
            show_ends = true;
        }

        if arg == "--number" {
            number = true;
        }
    }

    for arg in &args[1..] {
        if arg.chars().nth(0).unwrap() != '-' {
            let file = fs::File::open(arg).expect("Please enter a valid file");
            let buf_reader = io::BufReader::new(file);

            let mut line_number: i32 = 0;
            for line in buf_reader.lines() {
                match line {
                    Ok(mut line) => {
                        if show_ends == true {
                            line = line + "$";
                        }

                        if number == true {
                            line_number += 1;
                            line = line_number.to_string() + "\t" + &line;
                        }

                        println!("{line}")
                    }
                    Err(e) => println!("{e}"),
                }
            }
        }
    }
}
