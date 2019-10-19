use cattywampus::{
    parser::{self, ParsedToken},
    stack::Stack,
    value::Value,
};
use editline;

fn main() {
    repl();
}

fn repl() {
    let mut stack = Stack::new();

    loop {
        let line = editline::readline("> ").unwrap();

        editline::add_history(line);

        let input = line.trim();

        if input == ":p" {
            println!("{:?}", stack);
            continue;
        } else if input == ":q" {
            return;
        }

        let parsed_tokens = parser::parse_line(input).collect::<Vec<_>>();
        
        println!("{:?}", parsed_tokens.iter().map(|(_, parsed_tok)| parsed_tok).collect::<Vec<_>>());

        // First, verify the input.
        let mut invalid = false;
        for (tok, parsed_tok) in &parsed_tokens {
            if *parsed_tok == ParsedToken::BadToken {
                println!("Error - Invalid token: {}", tok);
                invalid = true;
            }
        }

        if !invalid {
            for (_, parsed_tok) in parsed_tokens {
                match parsed_tok {
                    ParsedToken::Literal(val) => stack.push(val),
                    ParsedToken::Intrinsic(fun) => println!("{}", fun.name),
                    ParsedToken::BadToken => unreachable!(), // Bad tokens are handled above.
                }
            }

            print_stack(&stack);
        }
    }
}

fn print_stack(stack: &Stack) {
    for i in stack {
        print!("{} ", i);
    }

    println!("");
}
