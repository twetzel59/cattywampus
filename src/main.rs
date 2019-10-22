use cattywampus::{
    parser::{self, ParsedToken},
    stack::Stack,
    typecheck,
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
        } else if input == ":r" {
            stack.clear();

            println!("Stack cleared.");
            continue;
        } else if input == ":q" {
            return;
        }

        let parsed_tokens = parser::parse_line(input).collect::<Vec<_>>();

        // Just for debugging.
        println!(
            "{:?}",
            parsed_tokens
                .iter()
                .map(|(_, parsed_tok)| parsed_tok)
                .collect::<Vec<_>>()
        );

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
                    ParsedToken::Intrinsic(fun) => {
                        if let Err(typ_err) = typecheck::checked_apply(fun, &mut stack) {
                            println!("Error - {:?}: {:?}", typ_err, fun);
                        }
                    }
                    ParsedToken::BadToken => unreachable!(), // Bad tokens are handled above.
                }
            }

            print_stack(&stack);
        }
    }
}

fn print_stack(stack: &Stack) {
    let mut types = String::new();
    let mut values = String::new();
    
    for i in stack {
        let type_repr = i.type_str();
        let val_repr = i.to_string();
        
        let max_length = type_repr.len().max(val_repr.len()) + 1;
        
        make_entry(&mut types, type_repr, max_length);
        make_entry(&mut values, &val_repr, max_length);
    }

    println!("{}\n{}", types, values);
}

fn make_entry(result: &mut String, repr: &str, max_length: usize) {
    const SPACE: &str = " ";
    const CELL: &str = "|";
    
    *result += SPACE;
    *result += repr;
    for _ in 0..(max_length - repr.len()) {
        *result += SPACE;
    }
    *result += CELL;
}
