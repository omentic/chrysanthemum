use std::io::{Write, stdout, stdin};

use chrysanthemum::*;
use chrysanthemum::ast::*;

fn main() {
    println!("chrysanthemum");
    let mut input = String::new();
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
                simple::infer(Context::new(), parser::parse(&input));
            },
            "c" | "t" | "check" | "typecheck" => {
                println!("enter fully annotated expression to typecheck");
                print!("\x1b[1m====> \x1b[22m");
                stdout().flush().unwrap();

                input.clear();
                stdin().read_line(&mut input).unwrap();
                simple::check(Context::new(), parser::parse(&input));
            },
            "e" | "r" | "execute" | "run" => {
                println!("enter expression to execute");
                print!("\x1b[1m====> \x1b[22m");
                stdout().flush().unwrap();

                input.clear();
                stdin().read_line(&mut input).unwrap();
                println!("{:?}", simple::execute(Context::new(), parser::parse(&input)));
            },
            _ => println!("invalid option {}. please try again.", input.trim())
        }
    }
}
