pub struct Option {
    pub value: bool,
    pub short: char,
    pub long: String,
}

pub fn get_options() -> Vec<Option> {
    let options: Vec<Option> = vec![
        Option {
            value: false,
            short: 'E',
            long: "--show-ends".to_string(),
        },
        Option {
            value: false,
            short: 'T',
            long: "--show-tabs".to_string(),
        },
        Option {
            value: false,
            short: 'n',
            long: "--number".to_string(),
        },
        Option {
            value: false,
            short: 'b',
            long: "--number-nonblank".to_string(),
        },
        Option {
            value: false,
            short: 's',
            long: "--squeeze-blank".to_string(),
        },
        Option {
            value: false,
            short: 'h',
            long: "--help".to_string(),
        },
    ];

    return options;
}

pub fn set_options(args: &[String], mut options: Vec<Option>) -> Vec<Option> {
    for arg in args {
        // ignore files
        if arg.chars().nth(0).unwrap() != '-' {
            continue;
        }

        for mut option in options.iter_mut() {
            if &option.long == arg {
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

    return options;
}
