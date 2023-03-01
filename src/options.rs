pub struct Option {
    pub value: bool,
    short: char,
    long: &'static str,
    help: &'static str,
}

fn get_options() -> Vec<Option> {
    let options: Vec<Option> = vec![
        Option {
            value: false,
            short: 'A',
            long: "--show-all",
            help: "equivalent to -vET",
        },
        Option {
            value: false,
            short: 'e',
            long: "",
            help: "equivalent to -vE",
        },
        Option {
            value: false,
            short: 't',
            long: "",
            help: "equivalent to -vT",
        },
        Option {
            value: false,
            short: 'v',
            long: "--show-nonprinting",
            help: "use ^ and M- notation, except for LFD and TAB",
        },
        Option {
            value: false,
            short: 'E',
            long: "--show-ends",
            help: "display $ at end of each line",
        },
        Option {
            value: false,
            short: 'T',
            long: "--show-tabs",
            help: "display TAB characters as ^I",
        },
        Option {
            value: false,
            short: 'n',
            long: "--number",
            help: "number all output lines",
        },
        Option {
            value: false,
            short: 'b',
            long: "--number-nonblank",
            help: "number nonempty output lines, overrides -n",
        },
        Option {
            value: false,
            short: 's',
            long: "--squeeze-blank",
            help: "suppress repeated empty output lines",
        },
        Option {
            value: false,
            short: 'h',
            long: "--help",
            help: "display this help and exit",
        },
    ];

    return options;
}

pub fn set_options(args: &[String]) -> Result<Vec<Option>, String> {
    let mut options = get_options();

    for arg in args {
        let is_file = arg.chars().nth(0).unwrap() != '-';
        let is_standard_input = arg == "-";

        if is_file || is_standard_input {
            continue;
        }

        let is_long = arg.chars().nth(1).unwrap() == '-';

        if is_long {
            let mut long_not_matching = true;

            for option in options.iter_mut() {
                if option.long == arg {
                    option.value = true;
                    long_not_matching = false;
                }
            }

            if long_not_matching {
                return Err(arg.to_string() + " doesn't exist");
            }
        }

        if !is_long {
            for c in arg.chars() {
                let mut short_not_matching = true;

                for option in options.iter_mut() {
                    if option.short == c {
                        option.value = true;
                        short_not_matching = false;
                    }
                }

                if short_not_matching && c != '-' {
                    return Err("-".to_string() + c.to_string().as_str() + " doesn't exist");
                }
            }
        }
    }

    return Ok(options);
}

pub fn print_help() {
    let options = get_options();
    let mut help = "Usage: rs-cat [OPTION]... [FILE]...
Concatenate FILE(s) to standard output.

With no FILE, or when FILE is -, read standard input.\n\n"
        .to_string();

    for option in options {
        let line = format_help_line(option);
        help.push_str(&line);
        help.push_str("\n");
    }

    print!("{}", help);
}

fn format_help_line(option: Option) -> String {
    let mut line = String::from("");

    line.push_str("  -");
    line.push_str(option.short.to_string().as_str());
    line.push_str(", ");
    line.push_str(option.long);

    line.push_str("\t");
    for tab_multiple in [8, 16, 24] {
        if line.len() <= tab_multiple {
            line.push_str("\t");
        }
    }

    line.push_str(option.help);

    return line;
}
