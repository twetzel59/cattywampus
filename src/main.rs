use std::io::{self, BufRead, BufReader};
use cattywampus::{parser, stack::Stack, value::Value};

fn main() {
    repl();
}

fn repl() {
    let mut stack = Stack::new();
    
    let mut buf = String::new();
    let mut reader = BufReader::new(io::stdin());
    
    loop {
        reader.read_line(&mut buf).unwrap();
        
        //print!("{}", buf);
        
        let input = buf.trim();
        
        if input == ":p" {
            println!("{:?}", stack);
        } else if input == ":q" {
            return;
        } 
        
        println!("{:?}", parser::parse_line(&input).collect::<Vec<_>>());
        
        buf.clear();
    }
}
