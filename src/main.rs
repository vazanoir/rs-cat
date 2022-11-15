use std::env;
use std::fs;
use std::io;
use std::io::prelude::BufRead;

const LINENUMBER_MAX_SIZE: usize = 6;

fn format_linenumber(number: i32) -> String {
    let char_to_fill = LINENUMBER_MAX_SIZE - number.to_string().len();
    let mut formated_linenumber = number.to_string();

    for _ in 0..char_to_fill {
        formated_linenumber = " ".to_string() + &formated_linenumber;
    }

    return formated_linenumber;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter a path");
        return;
    }

    // Arguments
    let mut show_ends = false;
    let mut number = false;
    let mut number_nonblank = false;
    let mut show_tabs = false;
    let mut squeeze_blank = false;

    for arg in &args[1..] {
        // Handle short and combined arguments
        let first_char = arg.chars().nth(0).unwrap();
        let second_char = arg.chars().nth(1).unwrap();

        if first_char == '-' && second_char != '-' {
            for short_arg in arg.chars() {
                if short_arg == 'E' {
                    show_ends = true;
                }

                if short_arg == 'n' {
                    number = true;
                }

                if short_arg == 'b' {
                    number_nonblank = true;
                }

                if short_arg == 'T' {
                    show_tabs = true;
                }

                if short_arg == 's' {
                    squeeze_blank = true;
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

        if arg == "--number-nonblank" {
            number_nonblank = true;
        }

        if arg == "--show-tabs" {
            show_tabs = true;
        }

        if arg == "--squeeze_blank" {
            squeeze_blank = true;
        }
    }

    let mut output = String::from("");
    let mut line_number: i32 = 0;
    let mut blank_line_counter: i32 = 0;

    for arg in &args[1..] {
        if arg.chars().nth(0).unwrap() != '-' {
            let file = fs::File::open(arg).expect("Please enter a valid file");
            let buf_reader = io::BufReader::new(file);

            for line in buf_reader.lines() {
                match line {
                    Ok(mut line) => {
                        line_number += 1;
                        if number_nonblank && line.len() == 0 {
                            line_number -= 1;
                        }

                        if line.len() == 0 {
                            blank_line_counter += 1;
                        } else {
                            blank_line_counter = 0;
                        }

                        if show_tabs {
                            line = line.replace("\t", "^I");
                        }

                        if number || number_nonblank {
                            if number_nonblank && line.len() == 0 {
                                line = "".to_string();
                            } else {
                                line = format_linenumber(line_number) + "\t" + &line;
                            }
                        }

                        if show_ends {
                            line = line + "$";
                        }

                        if !squeeze_blank || (squeeze_blank && blank_line_counter <= 1) {
                            output += &(line + "\n");
                        }
                    }
                    Err(e) => println!("{e}"),
                }
            }
        }
    }

    println!("{output}");
}
