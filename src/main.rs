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

fn char_from_u32_to_string(i: u32) -> String {
    return char::from_u32(i).unwrap().to_string();
}

fn replace_nonprinting(
    mut line: String,
    before_code: u32,
    shift: u32,
    shift_up: bool,
    after_notation: &str,
) -> String {
    let before = char_from_u32_to_string(before_code);
    let mut after = String::from(after_notation);

    match shift_up {
        true => after.push_str(&char_from_u32_to_string(before_code + shift)),
        false => after.push_str(&char_from_u32_to_string(before_code - shift)),
    }

    line = line.replace(&before, &after);
    return line;
}

fn format_line(
    options: &Vec<options::Option>,
    mut line: String,
    mut output: String,
    mut line_number: usize,
    mut blank_line_counter: usize,
) -> (String, usize, usize) {
    let show_all = options[0].value;
    let e = options[1].value;
    let t = options[2].value;
    let show_nonprinting = options[3].value;
    let show_ends = options[4].value;
    let show_tabs = options[5].value;
    let number = options[6].value;
    let number_nonblank = options[7].value;
    let squeeze_blank = options[8].value;

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

    if show_tabs || show_all || t {
        line = line.replace("\t", "^I");
    }

    if show_nonprinting || show_all || e || t {
        const TAB: u32 = 9;
        const LFD: u32 = 10;

        let control_chars_codes = 0..32;
        for dec_code in control_chars_codes {
            if let TAB | LFD = dec_code {
                continue;
            }

            line = replace_nonprinting(line, dec_code, 64, true, "^");
        }

        // DEL is an exception
        line = replace_nonprinting(line, 127, 64, false, "^");

        let first_extended_ascii_codes = 128..160;
        for dec_code in first_extended_ascii_codes {
            line = replace_nonprinting(line, dec_code, 64, false, "M-^")
        }

        let second_extended_ascii_codes = 160..255;
        for dec_code in second_extended_ascii_codes {
            line = replace_nonprinting(line, dec_code, 128, false, "M-")
        }

        // Another exception
        line = replace_nonprinting(line, 255, 192, false, "M-^")
    }

    if number || number_nonblank {
        if number_nonblank && empty_line {
            line = "".to_string();
        } else {
            line = fmt_line_number(line_number) + "\t" + &line;
        }
    }

    if show_ends || show_all || e {
        line = line + "$";
    }

    if !squeeze_blank || (squeeze_blank && blank_line_counter <= 1) {
        output += &(line + "\n");
    }

    return (output, line_number, blank_line_counter);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = options::set_options(&args[1..]);

    let options = match options {
        Ok(ok) => ok,
        Err(err) => {
            println!("ERROR: {}\n", err);
            options::print_help();
            return;
        }
    };

    if options[9].value {
        options::print_help();
        return;
    }

    let mut standard_input = false;
    let mut file_count = 0;

    for arg in args[1..].iter() {
        let is_standard_input = arg.as_str() == "-";
        if is_standard_input {
            standard_input = true;
        }

        let is_arg = arg.chars().nth(0).unwrap() == '-';
        if !is_arg {
            file_count += 1;
        }
    }

    if file_count <= 0 {
        standard_input = true;
    }

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
                    buffer.trim_end().to_string(),
                    output.clone(),
                    line_number,
                    blank_line_counter,
                ),
                Err(err) => panic!("{}", err),
            };

            let line = result.0.to_string();
            line_number = result.1;
            blank_line_counter = result.2;

            print!("{line}");
        }
    }

    if !standard_input {
        for arg in &args[1..] {
            let is_arg = arg.chars().nth(0).unwrap() == '-';

            if is_arg {
                continue;
            }

            let metadata = fs::metadata(arg);
            let metadata = match metadata {
                Ok(ok) => ok,
                Err(_) => {
                    output += &(args[0].to_owned() + ": " + arg + ": No such file or directory\n");
                    continue;
                }
            };

            if metadata.is_dir() {
                output += &(args[0].to_owned() + ": " + arg + ": Is a folder\n");
                continue;
            }

            let file = fs::File::open(arg);
            let file = match file {
                Ok(ok) => ok,
                Err(err) => {
                    println!("{}: {}: {}\n", args[0].to_owned(), arg, err);
                    options::print_help();
                    return;
                }
            };

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

                let new_output = result.0;
                line_number = result.1;
                blank_line_counter = result.2;

                output = new_output;
            }
        }

        print!("{output}");
    }
}
