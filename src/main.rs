use std::env;
use std::fs;
use std::io;
use std::io::prelude::BufRead;

const LINE_NUMBER_MAX_SIZE: usize = 6;

fn fmt_line_number(number: i32) -> String {
    let char_to_fill = LINE_NUMBER_MAX_SIZE - number.to_string().len();
    let mut formated_line_number = number.to_string();

    for _ in 0..char_to_fill {
        formated_line_number = " ".to_string() + &formated_line_number;
    }

    return formated_line_number;
}

fn print_help() {
    println!(
        "Usage: cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.
  -A, --show-all            equivalent to -ET
  -E, --show-ends           display $ at end of each line
  -T, --show-tabs           display TAB characters as ^I
  -n, --number              number all output lines
  -b, --number-nonblank     number nonempty output lines, overrides -n
  -s, --squeeze-blank       suppress repeated empty output lines
  -h, --help                display this help and exit"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    // Arguments
    let mut help = false;
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
                match short_arg {
                    'h' => help = true,
                    'E' => show_ends = true,
                    'n' => number = true,
                    'b' => number_nonblank = true,
                    'T' => show_tabs = true,
                    's' => squeeze_blank = true,
                    'A' => {
                        show_ends = true;
                        show_tabs = true
                    }
                    _short_arg => (),
                }
            }
        }

        // Long arguments
        match arg.as_str() {
            "--help" => help = true,
            "--show-ends" => show_ends = true,
            "--number" => number = true,
            "--number-nonblank" => number_nonblank = true,
            "--show-tabs" => show_tabs = true,
            "--squeeze-blank" => squeeze_blank = true,
            "--show-all" => {
                show_ends = true;
                show_tabs = true
            }
            _arg => (),
        }
    }

    if help {
        print_help();
        return;
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
                        let empty_line = line.len() == 0;

                        let b_decrement = number_nonblank && empty_line;
                        let s_decrement =
                            !number_nonblank && squeeze_blank && blank_line_counter > 1;

                        line_number += 1;
                        if b_decrement || s_decrement {
                            line_number -= 1;
                        }

                        if empty_line {
                            blank_line_counter += 1;
                        } else {
                            blank_line_counter = 0;
                        }

                        if show_tabs {
                            line = line.replace("\t", "^I");
                        }

                        if number || number_nonblank {
                            if number_nonblank && empty_line {
                                line = "".to_string();
                            } else {
                                line = fmt_line_number(line_number) + "\t" + &line;
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

    let output = match output.strip_suffix("\n") {
        Some(output) => output,
        None => &output,
    };

    println!("{output}");
}
