use std::io::{self, Write};

#[derive(Debug, PartialEq)]
#[repr(i32)]
enum Token {
    Number(i32),
    Letter(char),
    Add = 43,
    Sub = 45,
    Multiply = 42,
    Divide = 47,
    Exponent = 94,
    LeftParenthesis = 40,
    RightParenthesis = 41,
}

// impl Token {
//     fn PartialEq(&self, other: &Token) -> bool {
//         self.
//     }
// }

// #[derive(Debug)]
// struct Node<'a> {
//     left: &'a Token,
//     right: &'a Token
// }

#[derive(Debug)]
struct Node<'a> {
    left: &'a Token,
    right: Option<&'a Token>
}

fn tokenTypeFromChar(char: char) -> Option<Token> {
    match char as u8 {
        43 => Some(Token::Add),
        45 => Some(Token::Sub),
        42 => Some(Token::Multiply),
        47 => Some(Token::Divide),
        94 => Some(Token::Exponent),
        40 => Some(Token::LeftParenthesis),
        41 => Some(Token::RightParenthesis),
        48..=57 => Some(Token::Number(char.to_digit(10).unwrap() as i32)),
        65..=90 => Some(Token::Letter(char)),
        97..=122 => Some(Token::Letter(char)),
        _ => None
    }
}

fn main() {
    loop {
        print!("Text: ");
        let _ = io::stdout().flush();
        
        let mut expression_string = String::new();
        let _ = io::stdin().read_line(&mut expression_string);

        let mut tokens: Vec<Token> = Vec::new();
        let characters: Vec<char> = expression_string.chars().collect();
        let mut index: usize = 0;
        let mut char: char;
        while index < expression_string.len() {
            char = characters[index];
            if char == '\n' {
                break;
            }
            if char == ' ' {
                index += 1;
                continue
            }
            let optional_token_type: Option<Token>;
            if char.is_digit(10) {
                let mut number_string = String::from(char);

                index += 1; // skip first number
                'inner_while_loop: while index < expression_string.len() {
                    let char2 = characters[index];
                    if char2.is_digit(10) {
                        number_string.push(char2);
                    } else {
                        index -= 1;
                        break 'inner_while_loop;
                    }

                    index += 1;
                }
                optional_token_type = Some(Token::Number(number_string.parse().unwrap()));
            } else {
                optional_token_type = tokenTypeFromChar(char);
            }
            match optional_token_type {
                Some(token_type) => {
                    tokens.push(token_type);
                },
                None => {
                    println!("Error: unexpected character '{}' at position {}\n{}\n{}^ invalid character here", char, index, &expression_string, " ".repeat((index - 1) as usize));
                    break;
                }
            }

            index += 1;
        }
        println!("Tokenized. Tokens: {:?}", tokens);

        // let mut left_token: Option<&Token> = None;
        // let mut right_token: Option<&Token> = None;

        // let mut tree: Vec<Node> = Vec::new();
        // let mut token_index: usize = 0;
        // while token_index < tokens.len() {
        //     let token = &tokens[token_index];
        //     println!("index: {}", token_index);
        //     match left_token {
        //         Some(left) => {
        //             match right_token {
        //                 Some(right) => {
        //                     tree.push(Node {left: &left, right: &right});
        //                     left_token = None;
        //                     right_token = None;
        //                 },
        //                 None => {
        //                     right_token = Some(token);
        //                 }
        //             }
        //         },
        //         None => {
        //             left_token = Some(token);
        //         }
        //     }
        //     token_index += 1;
        // }

        // let mut tree: Vec<Node> = Vec::new();

        // println!("Parsed. Tree: {:?}", tree);
    }
}
