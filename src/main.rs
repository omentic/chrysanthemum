use std::io::{Write, stdout, stdin};

use chrysanthemum::*;
use chrysanthemum::ast::*;

fn main() {
    println!("chrysanthemum");
    let mut input = String::new();
    let empty_context = Context::new();
    loop {
        println!("infer, check, or execute? (i/c/e)");
        print!("\x1b[1m==> \x1b[22m");
        stdout().flush().unwrap();

        input.clear();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "i" | "g" | "infer" => {
                println!("enter partially annotated expression to fully infer");
                print!("\x1b[1m====> \x1b[22m");
                stdout().flush().unwrap();

                input.clear();
                stdin().read_line(&mut input).unwrap();
                match empty_context.infer(parser::parse_lambda(&input).unwrap()) {
                    Ok(kind) => println!("infers! {}", kind),
                    Err(e) => println!("{:?}", e),
                }
            },
            "c" | "t" | "check" => {
                println!("enter fully annotated expression to typecheck");
                print!("\x1b[1m====> \x1b[22m");
                stdout().flush().unwrap();

                input.clear();
                stdin().read_line(&mut input).unwrap();
                let kind = empty_context.infer(parser::parse_lambda(&input).unwrap());
                match kind {
                    Ok(kind) => {
                        match empty_context.check(parser::parse_lambda(&input).unwrap(), &kind) {
                            Ok(_) => println!("checks!"),
                            Err(e) => println!("{:?}", e),
                        }
                    },
                    Err(_) => println!("failed to infer high-level type!")
                }
            },
            "e" | "r" | "execute" | "run" => {
                println!("enter expression to execute");
                print!("\x1b[1m====> \x1b[22m");
                stdout().flush().unwrap();

                input.clear();
                stdin().read_line(&mut input).unwrap();
                match empty_context.execute(parser::parse_lambda(&input).unwrap()) {
                    Ok(term) => println!("{}", term),
                    Err(e) => println!("{:?}", e)
                }
            },
            _ => println!("invalid option {}. please try again.", input.trim())
        }
    }
}
