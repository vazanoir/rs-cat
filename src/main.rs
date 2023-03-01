use std::env;
use std::fs;
use std::io;
use std::io::prelude::BufRead;

mod options;

const LINE_NUMBER_MAX_LENGTH_SIZE: usize = 6;

fn fmt_line_number(number: i32) -> String {
    let char_to_fill = LINE_NUMBER_MAX_LENGTH_SIZE - number.to_string().len();
    let mut formated_line_number = number.to_string();

    for _ in 0..char_to_fill {
        formated_line_number = " ".to_string() + &formated_line_number;
    }

    return formated_line_number;
}


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let options = options::set_options(&args);

    let options = match options {
        Ok(ok) => ok,
        Err(err) => {
            println!("ERROR: {}\n", err);
            options::print_help();
            return;
        }
    };

    if options[6].value {
        options::print_help();
        return;
    }

    let mut output = String::from("");
    let mut line_number: i32 = 0;
    let mut blank_line_counter: i32 = 0;

    for file in &args {
        if file.chars().nth(0).unwrap() != '-' {
            let file = fs::File::open(file).expect("Please enter a valid file");
            let buf_reader = io::BufReader::new(file);

            let show_all = options[0].value;
            let show_ends = options[1].value;
            let show_tabs = options[2].value;
            let number = options[3].value;
            let number_nonblank = options[4].value;
            let squeeze_blank = options[5].value;

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

                        if show_tabs || show_all {
                            line = line.replace("\t", "^I");
                        }

                        if number || number_nonblank {
                            if number_nonblank && empty_line {
                                line = "".to_string();
                            } else {
                                line = fmt_line_number(line_number) + "\t" + &line;
                            }
                        }

                        if show_ends || show_all {
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
