use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter a path");
        return;
    }

    for arg in &args[1.. ] {
        if arg.chars().nth(0).unwrap() != '-' {
            let contents = fs::read_to_string(arg)
                .expect("Please use valid path");

            println!("{contents}");
        }
    }
}
