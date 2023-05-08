use std::{env, fs};

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Letter(char),
    Number(usize),
    Alphabet,
    Rule,
    Equals,
    CurlyOpen,
    CurlyClose,
    Open,
    Close,
    Comma,
}

#[derive(PartialEq, Eq, Debug)]
enum Node {
    Alphabet(Vec<char>),
    Rule((usize, char), (usize, char)),
}

fn lex(machine: String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut number = String::new();
    for c in machine.chars() {
        if c.is_ascii_digit() {
            number.push(c);
            continue;
        }
        if number != "".to_string() {
            result.push(Token::Number(number.parse::<usize>().unwrap()));
            number = "".to_string();
        }
        match c {
            'T' => {result.push(Token::Alphabet)},
            'D' => {result.push(Token::Rule)},
            '=' => {result.push(Token::Equals)},
            '{' => {result.push(Token::CurlyOpen)},
            '}' => {result.push(Token::CurlyClose)},
            '(' => {result.push(Token::Open)},
            ')' => {result.push(Token::Close)},
            ',' => {result.push(Token::Comma)},
            ' ' | '\n' => {},
            'a'..='z' => {result.push(Token::Letter(c))}
            _ => {panic!("Unexpected character")}
        }
    }

    result
}

/**
 * wtf is this
 */
fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut index = 0;
    let len = tokens.len();
    let mut result: Vec<Node> = Vec::new();
    while index < len {
        match tokens[index] {
            Token::Alphabet => {
                index = expect_token(&tokens, index, Token::Equals);
                index = expect_token(&tokens, index, Token::CurlyOpen);
                let mut alphabet: Vec<char> = Vec::new();
                let mut can_have_next = true;
                index += 1;
                while tokens[index] != Token::CurlyClose {
                    if !can_have_next && tokens[index] != Token::Comma {
                        panic!("missing comma");
                    }
                    match tokens[index] {
                        Token::Letter(c) => { 
                            alphabet.push(c); 
                            can_have_next = false;
                        },
                        Token::Number(n) => {
                            if n / 10 != 0 {
                                panic!("unexpected token! alpabet can contain only digits, not numbers");
                            }
                            alphabet.push(n.to_string().chars().next().unwrap());
                            can_have_next = false;
                        },
                        Token::Comma => {
                            can_have_next = true;
                        },
                        _ => { panic!("unexpected token!") }
                    }
                    
                    index += 1;
                }
                result.push(Node::Alphabet(alphabet));
                index += 1;
            },
            Token::Rule => {
                index = expect_token(&tokens, index, Token::Open);   
                let init_state;
                index += 1;
                match tokens[index] {
                    Token::Number(n) => {
                        init_state = n;
                    },
                    _ => panic!("unexpected token")
                }
                index = expect_token(&tokens, index, Token::Comma);

                index += 1;
                let init_value;
                match tokens[index] {
                    Token::Letter(c) => {
                        init_value = c;
                    },
                    Token::Number(n) => {
                        if n / 10 != 0 {
                            panic!("unexpected token! alpabet can contain only digits, not numbers");
                        }
                        init_value = n.to_string().chars().next().unwrap();
                    },
                    _ => panic!("unexpected token")
                }
                index = expect_token(&tokens, index, Token::Close);
                index = expect_token(&tokens, index, Token::Equals);

                index = expect_token(&tokens, index, Token::Open);   
                let new_state;
                index += 1;
                match tokens[index] {
                    Token::Number(n) => {
                        new_state = n;
                    },
                    _ => panic!("unexpected token")
                }
                index = expect_token(&tokens, index, Token::Comma);

                index += 1;
                let new_value;
                match tokens[index] {
                    Token::Letter(c) => {
                        new_value = c;
                    },
                    Token::Number(n) => {
                        if n / 10 != 0 {
                            panic!("unexpected token! alpabet can contain only digits, not numbers");
                        }
                        new_value = n.to_string().chars().next().unwrap();
                    },
                    _ => panic!("unexpected token")
                }
                index = expect_token(&tokens, index, Token::Close);

                result.push(Node::Rule((init_state, init_value), (new_state, new_value)));
                index += 1;
            },
            _ => { panic!("unexpected token") },

        }
    }

    result
}

fn expect_token(tokens: &Vec<Token>, at: usize, token: Token) -> usize{
    if tokens[at + 1] != token {
        panic!("unexpected token");
    }

    at + 1
}

fn interpret(nodes: Vec<Node>, start: usize, end: usize, tape: &Vec<char>) -> Vec<char>{
    let mut alphabet: &Vec<char> = &Vec::new();
    let mut index = 0;
    let len = nodes.len();
    while index < len {
        match &nodes[index] {
            Node::Alphabet(a) => {
                if index != 0 {
                    panic!("alphabet must be defined only once at the begininning");
                }
                
                alphabet = a;
            },
            Node::Rule(_, _) => {
                if index == 0 {
                    panic!("working without alphabet!");
                }
            }
        }
        index += 1;
    }

    let mut new_tape = tape.clone();
    let rules = &nodes[1..];
    let mut pos = start;
    let mut state: char;
    let mut rules_index: usize = 0;
    let rules_len = rules.len();
    while pos != end {
        state = tape[pos]; 
        if !alphabet.contains(&state) {
            panic!("unexpected letter {}", state);
        }

        loop {
            if rules_index == rules_len {
                panic!("missing rule for {} {}", pos, tape[pos]);
            }
            match rules[rules_index] {
                Node::Rule(init, new) => {
                    if init.0 == pos && init.1 == state {
                        new_tape[pos] = new.1;
                        pos = new.0;
                        break;
                    }
                },
                _ => { panic!("wtf") }
            }
            rules_index += 1;
        }
    }

    new_tape
}

fn parse_tape(tape: String) -> (usize, usize, Vec<char>) {
    let mut parts = tape.split(';');

    let start = parts.next().expect("tape error").parse::<usize>().expect("tape error");
    let end = parts.next().expect("tape error").parse::<usize>().expect("tape error");
    let tape_itself = parts.next().expect("tape error").chars().collect();
    
    (start, end, tape_itself)
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).expect("no file provided");

    let code = fs::read_to_string(filename)
        .expect("File is not readable");

    let tape_filename = args.nth(0).expect("no file provided");

    let tape = fs::read_to_string(tape_filename)
        .expect("File is not readable");

    let (start, end, tape_itself) = parse_tape(tape);

    println!("{}", String::from_iter(
            interpret(parse(lex(code)), start, end, &tape_itself))
    );
}
