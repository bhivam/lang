use std::{iter::Peekable, mem, str::Chars};

use crate::{SyntaxError, Token, TokenType};

struct Lexer<'s> {
    tokens: Vec<Token>,
    current_lexeme: String,
    chars: Peekable<Chars<'s>>,
    line: u32,
    column: u32,
    token_start_line: u32,
    token_start_column: u32,
}

impl<'s> Lexer<'s> {
    fn new_error(&self, message: String) -> SyntaxError {
        SyntaxError::new(
            message,
            self.token_start_line,
            self.token_start_column,
        )
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

    fn next_token(&mut self) -> Result<bool, SyntaxError> {
        self.current_lexeme.clear();
        self.token_start_line = self.line;
        self.token_start_column = self.column + 1;

        let character = match self.consume() {
            Some(character) => character,
            None => return Ok(false),
        };

        match character {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
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
                } else if character.is_ascii_alphabetic() || character == '_' {
                    while let Some(character) = self.chars.peek() {
                        if character.is_ascii_alphanumeric() || *character == '_' {
                            self.consume();
                        } else {
                            break;
                        };
                    }

                    match self.current_lexeme.as_str() {
                        "and" => self.add_token(TokenType::And),
                        "or" => self.add_token(TokenType::Or),
                        _ => {}
                    };
                } else {
                    return Err(self.new_error(format!("Unexpected Token: {character}")));
                }
            }
        }

        Ok(true)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = mem::take(&mut self.current_lexeme);

        self.tokens.push(Token {
            token_type,
            lexeme,
            column: self.token_start_column,
            line: self.token_start_line,
        });
    }
}

pub fn scan(source_code: &str) -> Result<Vec<Token>, SyntaxError> {
    let mut lexer = Lexer {
        chars: source_code.chars().peekable(),
        tokens: Vec::new(),
        current_lexeme: String::new(),
        line: 1,
        column: 0,
        token_start_line: 1,
        token_start_column: 1,
    };

    loop {
        match lexer.next_token() {
            Ok(true) => continue,
            Ok(false) => break,
            Err(error) => return Err(error),
        }
    }

    lexer.tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: String::new(),
        line: lexer.line,
        column: lexer.column,
    });

    Ok(lexer.tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_types(input: &str) -> Vec<TokenType> {
        scan(input)
            .unwrap()
            .into_iter()
            .map(|t| t.token_type)
            .collect()
    }

    #[test]
    fn test_numbers() {
        assert_eq!(
            token_types("42"),
            vec![TokenType::Number(42), TokenType::Eof]
        );
        assert_eq!(
            token_types("123 456"),
            vec![
                TokenType::Number(123),
                TokenType::Number(456),
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_strings() {
        let tokens = scan("\"hello\"").unwrap();
        assert_eq!(tokens[0].token_type, TokenType::String("hello".to_string()));
    }

    #[test]
    fn test_arithmetic_operators() {
        assert_eq!(
            token_types("+ - * /"),
            vec![
                TokenType::Plus,
                TokenType::Minus,
                TokenType::Star,
                TokenType::Slash,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_comparison_operators() {
        assert_eq!(
            token_types("> >= < <= !="),
            vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
                TokenType::BangEqual,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_logical_operators() {
        assert_eq!(
            token_types("! and or"),
            vec![
                TokenType::Bang,
                TokenType::And,
                TokenType::Or,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(
            token_types("(1 + 2)"),
            vec![
                TokenType::LeftParen,
                TokenType::Number(1),
                TokenType::Plus,
                TokenType::Number(2),
                TokenType::RightParen,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_whitespace_ignored() {
        assert_eq!(
            token_types("1\n+\t2  *   3"),
            vec![
                TokenType::Number(1),
                TokenType::Plus,
                TokenType::Number(2),
                TokenType::Star,
                TokenType::Number(3),
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_line_column_tracking() {
        let tokens = scan("1\n2").unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[1].column, 1);
    }

    #[test]
    fn test_unexpected_character() {
        let result = scan("@");
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Unexpected Token"));
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(
            token_types("(1 + 2) * 3 >= 9 and !false"),
            vec![
                TokenType::LeftParen,
                TokenType::Number(1),
                TokenType::Plus,
                TokenType::Number(2),
                TokenType::RightParen,
                TokenType::Star,
                TokenType::Number(3),
                TokenType::GreaterEqual,
                TokenType::Number(9),
                TokenType::And,
                TokenType::Bang,
                TokenType::Eof
            ]
        );
    }

    #[test]
    fn test_multiline_string_position() {
        let tokens = scan("\"hello\nworld\"").unwrap();
        assert_eq!(tokens[0].token_type, TokenType::String("hello\nworld".to_string()));
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
    }

    #[test]
    fn test_multiple_tokens_per_line() {
        let tokens = scan("1 + 2").unwrap();
        assert_eq!(tokens[0].column, 1); // 1
        assert_eq!(tokens[1].column, 3); // +
        assert_eq!(tokens[2].column, 5); // 2
    }

    #[test]
    fn test_tab_column_tracking() {
        let tokens = scan("\t1").unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 5); // tab advances by 4, so 1 is at column 5
    }

    #[test]
    fn test_error_position_multiline() {
        let result = scan("\"unterminated\nstring");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 1);
    }
}
