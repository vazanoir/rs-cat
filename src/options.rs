pub struct Option {
    pub value: bool,
    pub short: char,
    pub long: &'static str,
}

fn get_options() -> Vec<Option> {
    let options: Vec<Option> = vec![
        Option {
            value: false,
            short: 'A',
            long: "--show-all",
        },
        Option {
            value: false,
            short: 'E',
            long: "--show-ends",
        },
        Option {
            value: false,
            short: 'T',
            long: "--show-tabs",
        },
        Option {
            value: false,
            short: 'n',
            long: "--number",
        },
        Option {
            value: false,
            short: 'b',
            long: "--number-nonblank",
        },
        Option {
            value: false,
            short: 's',
            long: "--squeeze-blank",
        },
        Option {
            value: false,
            short: 'h',
            long: "--help",
        },
    ];

    return options;
}

pub fn set_options(args: &[String]) -> Vec<Option> {
    let mut options = get_options();

    for arg in args {
        let is_file = arg.chars().nth(0).unwrap() != '-';
        let is_standard_input = arg == "-";

        if is_file || is_standard_input {
            continue;
        }

        for mut option in options.iter_mut() {
            if option.long == arg {
                option.value = true;
            }

            if arg.chars().nth(1).unwrap() != '-' {
                for c in arg.chars() {
                    if option.short == c {
                        option.value = true;
                    }
                }
            }
        }
    }

    if options[0].value == true {
        options[1].value = true;
        options[2].value = true;
    }

    return options;
}
