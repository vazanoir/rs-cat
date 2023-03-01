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

    if show_nonprinting || e || t {
        let control_chars = 0..=31;
        const TAB: u32 = 9;
        const LFD: u32 = 10;

        for c in control_chars {
            if let TAB | LFD = c {
                continue;
            }

            let before = char_from_u32_to_string(c);
            let mut after = String::from("^");
            after.push_str(&char_from_u32_to_string(c + 64));

            line = line.replace(&before, &after);
        }

        // DEL is an exception
        let before = char_from_u32_to_string(127);
        let after = char_from_u32_to_string(63);
        line = line.replace(&before, &after);
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

    args[1..]
        .iter()
        .filter(|arg| arg.as_str() == "-")
        .for_each(|_| standard_input = true);

    let file_count = args[1..]
        .iter()
        .filter(|arg| arg.chars().nth(0).unwrap() != '-')
        .count();

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
                    println!("ERROR: {}: {}\n", err, arg);
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

                let line = result.0;
                line_number = result.1;
                blank_line_counter = result.2;

                output = line;
            }
        }

        print!("{output}");
    }
}
