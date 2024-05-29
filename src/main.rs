mod tokenizer;
use colored::Colorize;
use std::fs::read_to_string as read_file;
use std::process::exit;
use std::env::args;
use tokenizer::Tokenizer;
fn main() {
    let arguments: Vec<String> = args().collect();
    if arguments.len() != 2 {
        eprintln!("Syntax: {} <FILENAME>", arguments[0]);
        exit(1);
    }
    let filedata = read_file(&arguments[1])
        .expect(format!("Could not read file from {}", arguments[1]).as_str());
    if filedata.len() == 0 {
        exit(0);
    }

    let mut tokenizer = Tokenizer::new(filedata);
    let tokens = match tokenizer.tokenize() {
        Ok(e) => e,
        Err(errormsg) => {
            panic_msg(format!("Tokenizer: '{errormsg}'"));
            exit(1);
        }
    };
    for token in tokens {
        dbg!(token);
    }
}

fn clean_bin() {
    if let Ok(files) = std::fs::read_dir("bin/") {
        for file in files {
            if let Ok(f) = file {
                let _ = std::fs::remove_file(f.path());
            }
        }
    }
}

fn panic_msg(msg: String) {
    eprintln!("{}", msg.red());
}