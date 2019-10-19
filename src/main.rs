use editline;
use std::io::{self, BufRead, BufReader};
use cattywampus::{parser, stack::Stack, value::Value};

fn main() {
    repl();
}

fn repl() {    
    let mut stack = Stack::new();
    
    loop {
        let line = match editline::readline("> ") {
            Some(l) => l,
            None => {
                println!("none!");
                "test"
            },
        };
        
        editline::add_history(line);
        
        let input = line.trim();
        
        if input == ":p" {
            println!("{:?}", stack);
        } else if input == ":q" {
            return;
        } 
        
        println!("{:?}", parser::parse_line(input).collect::<Vec<_>>());
    }
}
