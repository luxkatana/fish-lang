mod generation;
mod parser;
mod tokenizer;
use generation::CodeGeneration;
use parser::Parser;
use std::fs::{read_to_string as read_file, File};
use std::io::Write;
use std::process::exit;
use std::{env::args, process::Command};
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
            eprintln!("TOKENIZER: '{errormsg}'");
            exit(1);
        }
    };
    let mut parser = Parser::new(tokens);
    let nodes = match parser.create_nodes() {
        Ok(e) => e,
        Err(error) => {
            eprintln!("Parser: '{error}'");
            exit(1);
        }
    };
    let mut codegen = CodeGeneration::new(nodes);
    let generated_assembly = match codegen.generate_asm() {
        Ok(e) => e,
        Err(error) => {
            eprintln!("CODEGENERATION: '{error}'");
            exit(1);
        }
    };
    println!("{generated_assembly}");
    clean_bin();
    match std::env::set_current_dir("bin/") {
        Err(_) => {
            println!("BIN FOLDER DOES NOT EXIST, THEREFORE IT'LL BE CREATED");
            let _ = std::fs::create_dir("bin/");
            let _ = std::env::set_tcurrent_dir("bin/");
        },
        _ => {}
    }
    File::create("dump.asm")
        .expect("Could not create dump file")
        .write(generated_assembly.as_bytes())
        .expect("Could not write data to dump file './dump.asm'");

    Command::new("nasm")
        .arg("-felf64")
        .arg("dump.asm")
        .status()
        .expect("Could not create object file");
    Command::new("ld")
        .arg("-o")
        .arg("out.exe")
        .arg("dump.o")
        .status()
        .expect("Could not link to binary");
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
