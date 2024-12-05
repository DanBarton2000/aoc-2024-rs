use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs;
use std::io::{self, prelude::*};

#[derive(PartialEq, Eq, Copy, Clone)]
enum TokenType {
    Mul,
    Do,
    Don,
    Apostrophe,
    T,
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
    start: usize,
    key_words: HashMap<String, TokenType>
}

impl Lexer {
    fn new(source: Vec<char>) -> Lexer {

        let mut map = HashMap::new();
        map.insert("mul".to_string(), TokenType::Mul);
        map.insert("don".to_string(), TokenType::Don);
        map.insert("do".to_string(), TokenType::Do);

        Lexer {
            source,
            tokens: vec![],
            current: 0,
            start: 0,
            key_words: map
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
            't' => { self.add_token(TokenType::T); }
            '\'' => { self.add_token(TokenType::Apostrophe); }
            _ => {
                if c.is_numeric() {
                    self.number();
                } else if c.is_alphabetic() {
                    self.word(&c);
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

    fn word(&mut self, first_char: &char) {
        let mut key_word = String::new();
        key_word.push(*first_char);

        while !self.at_end() {
            let mut starts_with = false;

            if let Some(token) = self.key_words.get(&key_word) {
                self.add_token(*token);
                return;
            }

            for key in self.key_words.keys() {
                if key.starts_with(&key_word) {
                    starts_with = true;
                }
            }

            if starts_with {
                key_word.push(self.next_character());
            } else {
                break;
            }
        }

        self.add_token(TokenType::Other);
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