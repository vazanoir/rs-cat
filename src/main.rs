use std::env;
use std::fs;
use std::io;
use std::io::prelude::BufRead;

mod options;

const LINE_NUMBER_MAX_LENGTH_SIZE: usize = 6;

fn fmt_line_number(number: usize) -> String {
    let char_to_fill = LINE_NUMBER_MAX_LENGTH_SIZE - number.to_string().len();
    let mut formated_line_number = number.to_string();

    for _ in 0..char_to_fill {
        formated_line_number = " ".to_string() + &formated_line_number;
    }

    return formated_line_number;
}

fn format_line(
    options: &Vec<options::Option>,
    mut line: String,
    mut output: String,
    mut line_number: usize,
    mut blank_line_counter: usize,
) -> (String, usize, usize) {
    let show_all = options[0].value;
    let show_ends = options[1].value;
    let show_tabs = options[2].value;
    let number = options[3].value;
    let number_nonblank = options[4].value;
    let squeeze_blank = options[5].value;

    let empty_line = line.trim_end_matches("\n").len() == 0;

    let b_decrement = number_nonblank && empty_line;
    let s_decrement = !number_nonblank && squeeze_blank && blank_line_counter > 1;

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

    return (output, line_number, blank_line_counter);
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

    let mut standard_input = false;
    args.iter()
        .filter(|arg| arg.as_str() == "-")
        .for_each(|_| standard_input = true);

    let mut output = String::from("");
    let mut line_number: usize = 0;
    let mut blank_line_counter: usize = 0;

    if standard_input {
        loop {
            let mut buffer = String::new();
            let result = io::stdin().read_line(&mut buffer);

            let result = match result {
                Ok(_) => format_line(
                    &options,
                    buffer,
                    output.clone(),
                    line_number,
                    blank_line_counter,
                ),
                Err(err) => panic!("{}", err),
            };

            let line = result.0.trim_end_matches("\n").to_string();
            line_number = result.1;
            blank_line_counter = result.2;

            println!("{line}");
        }
    }

    if !standard_input {
        for arg in &args {
            let is_file = arg.chars().nth(0).unwrap() != '-';

            if is_file {
                let file = fs::File::open(arg).expect("Please enter a valid file");
                let buf_reader = io::BufReader::new(file);

                for line in buf_reader.lines() {
                    let result = match line {
                        Ok(line) => format_line(
                            &options,
                            line,
                            output.clone(),
                            line_number,
                            blank_line_counter,
                        ),
                        Err(e) => (e.to_string(), 0, 0),
                    };

                    let line = result.0;
                    line_number = result.1;
                    blank_line_counter = result.2;

                    output = line;
                }
            }
        }

        let output = match output.strip_suffix("\n") {
            Some(output) => output,
            None => &output,
        };

        println!("{output}");
    }
}
