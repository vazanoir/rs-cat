pub mod options;

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

pub fn format_line(
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
