use std::cmp::PartialEq;
use std::fs;
use std::io::{self, prelude::*};

#[derive(PartialEq, Eq)]
enum TokenType {
    Mul,
    LeftBracket,
    Number,
    Comma,
    RightBracket,
    Other
}

struct Token {
    token_type: TokenType,
    original_text: String,
}

impl Token {
    fn new(token_type: TokenType, original_text: String) -> Token {
        Token {
            token_type,
            original_text,
        }
    }
}

struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
    start: usize
}

impl Lexer {
    fn new(source: Vec<char>) -> Lexer {
        Lexer {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn next_character(&mut self) -> char {
        let character = self.source[self.current];
        self.current += 1;
        character
    }

    fn peek(&self) -> Option<char> {
        if self.at_end() { return None; }
        Some(self.source[self.current])
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, self.source[self.start..self.current].iter().collect()))
    }

    fn scan_token(&mut self) {
        let c = self.next_character();
        
        match c {
            '(' => { self.add_token(TokenType::LeftBracket); }
            ')' => { self.add_token(TokenType::RightBracket); }
            ',' => { self.add_token(TokenType::Comma); }
            'm' => {
                if let Some(u) = self.peek() {
                    if u == 'u' {
                        self.next_character();

                        if let Some(l) = self.peek() {
                            if l == 'l' {
                                self.next_character();
                                self.add_token(TokenType::Mul);
                            }
                        }
                    }
                }
            }
            _ => {
                if c.is_numeric() {
                    self.number();
                } else {
                    self.add_token(TokenType::Other);
                }
            }
        }
    }

    fn number(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_numeric() {
                self.next_character();
            } else {
                break;
            }
        }

        self.add_token(TokenType::Number);
    }

    fn scan_tokens(&mut self) {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
    }

    fn token(&self, i: usize) -> Option<&Token> {
        if i >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[i])
        }
    }
}

fn part_one() -> io::Result<()> {
    let mut file = fs::File::open(".\\files\\day_three.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let char_vec: Vec<char> = contents.chars().collect();

    let mut lexer: Lexer = Lexer::new(char_vec);
    lexer.scan_tokens();

    let mut index = 0;
    let mut result: u32 = 0;

    while let Some(token) = lexer.token(index) {
        index += 1;

        if token.token_type != TokenType::Mul { continue; }

        let Some(left) = lexer.token(index) else { continue; };
        if left.token_type != TokenType::LeftBracket { continue; }
        index += 1;

        let Some(number_one) = lexer.token(index) else { continue; };
        if number_one.token_type != TokenType::Number { continue; }
        index += 1;

        let Some(comma) = lexer.token(index) else { continue; };
        if comma.token_type != TokenType::Comma { continue; }
        index += 1;

        let Some(number_two) = lexer.token(index) else { continue; };
        if number_two.token_type != TokenType::Number { continue; }
        index += 1;

        let Some(right) = lexer.token(index) else { continue; };
        if right.token_type != TokenType::RightBracket { continue; }

        result += number_one.original_text.parse::<u32>().unwrap() * number_two.original_text.parse::<u32>().unwrap();
    }

    println!("{result}");

    // mul(
    // number
    // ,
    // number
    // )

    Ok(())
}

fn main() -> io::Result<()> {
    part_one()
}