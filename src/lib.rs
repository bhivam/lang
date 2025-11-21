use std::{iter::Peekable, mem, str::Chars};

#[derive(Debug)]
pub enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Or,

    String(String),
    Number(i32),
    True,
    False,
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub column: u32,
}

struct Lexer<'s> {
    tokens: Vec<Token>,
    current_lexeme: String,
    chars: Peekable<Chars<'s>>,
    line: u32,
    column: u32,
}

impl<'s> Lexer<'s> {
    fn new_error(&self, message: String) -> LexError {
        return LexError {
            message,
            line: self.line,
            column: self.column - (self.current_lexeme.len() as u32) + 1,
        };
    }

    fn consume_if(&mut self, ch: char) -> bool {
        match self.chars.peek() {
            Some(c) if *c == ch => {
                self.consume();
                true
            }
            _ => false,
        }
    }

    fn track_line_column(&mut self, character: char) {
        match character {
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            '\t' => {
                self.column += 4;
            }
            _ => {
                self.column += 1;
            }
        };
    }

    fn consume(&mut self) -> Option<char> {
        let character = match self.chars.next() {
            Some(character) => character,
            None => return None,
        };

        self.track_line_column(character);
        self.current_lexeme.push(character);

        Some(character)
    }

    fn next_token(&mut self) -> Result<bool, LexError> {
        self.current_lexeme.clear();

        let character = match self.consume() {
            Some(character) => character,
            None => return Ok(false),
        };

        match character {
            // we need to consider what counts as a delimiter for a word
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),
            '!' => {
                if self.consume_if('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '>' => {
                if self.consume_if('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.consume_if('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {}
            '"' => {
                let mut is_string = false;

                while let Some(character) = self.consume() {
                    if character == '"' {
                        is_string = true;
                        break;
                    }
                }

                if !is_string {
                    return Err(self.new_error("Invalid String".to_string()));
                }

                self.add_token(TokenType::String(
                    self.current_lexeme[1..self.current_lexeme.len() - 1].to_string(),
                ));
            }
            character => {
                if character.is_digit(10) {
                    // if we detect a digit, keep consuming digits until we hit a non digit
                    while let Some(character) = self.chars.peek() {
                        if character.is_digit(10) {
                            self.consume();
                        } else {
                            break;
                        };
                    }

                    self.add_token(TokenType::Number(
                        self.current_lexeme.parse::<i32>().unwrap(),
                    ));
                } else {
                    return Err(self.new_error(format!("Unexpected Token: {character}")));
                }
            }
        }

        Ok(true)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = mem::take(&mut self.current_lexeme);
        let lexeme_len = lexeme.len();

        self.tokens.push(Token {
            token_type,
            lexeme,
            column: self.column - (lexeme_len as u32) + 1,
            line: self.line,
        });
    }
}

pub fn scan_source(source_code: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer {
        chars: source_code.chars().peekable(),
        tokens: Vec::new(),
        current_lexeme: String::new(),
        line: 1,
        column: 0,
    };

    loop {
        match lexer.next_token() {
            Ok(true) => continue,
            Ok(false) => break,
            Err(error) => return Err(error),
        }
    }

    Ok(lexer.tokens)
}
