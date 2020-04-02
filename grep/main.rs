use std::{env, fs, process};

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("---");
        println!("usage: {} {} {}", args[0], "file", "pattern_string");
        println!("---");

        process::exit(1);
    }

    let pattern = &args[2];
    let file = fs::read_to_string(&args[1]).expect("File not Found");

    let mut line_counter : u32 = 0;
    let mut nothing = true;

    for line in file.split("\n") {
        for word in line.split(" ") {
            if word.contains(pattern) {
                nothing = false;
                println!(" - {}: \t {}", line_counter, word);
            }
        }
        line_counter += 1;
    }

    if nothing {
        println!("Nothing Found");
    }
}
