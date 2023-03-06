mod format;
use format::options;

use std::env;
use std::fs;
use std::io;
use std::io::prelude::BufRead;

pub fn main() {
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
                Ok(_) => format::format_line(
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
                    Ok(line) => format::format_line(
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
