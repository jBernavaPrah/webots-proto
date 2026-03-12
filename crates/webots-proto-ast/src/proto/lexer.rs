use super::span::Span;
use serde::{Deserialize, Serialize};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    Identifier(String),
    // Keywords
    Def,
    Use,
    Proto,
    ExternProto,
    Field,
    VrmlField,
    HiddenField,
    DeprecatedField,
    Is,
    Null,
    True,
    False,
    // Literals
    Float(f64, String),
    Int(i64, String),
    Str(String),
    // Symbols
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Comma,
    // Trivia
    Whitespace(String),
    Comment(String),
    Newline(String),
    // Template
    Template {
        content: String,
        is_expression: bool,
        terminated: bool,
    },
    // Eof
    Eof,
    // Error
    Unknown(char),
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    byte_offset: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            byte_offset: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                let len = c.len_utf8();
                self.byte_offset += len;
                if c == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
                Some(c)
            }
            None => None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let start_offset = self.byte_offset;
        let start_line = self.line;
        let start_col = self.col;

        let c = match self.peek() {
            Some(c) => c,
            None => {
                return Token {
                    kind: TokenKind::Eof,
                    span: Span::new(
                        start_offset,
                        start_offset,
                        start_line,
                        start_col,
                        start_line,
                        start_col,
                    ),
                };
            }
        };

        let kind = if c.is_whitespace() {
            self.read_whitespace()
        } else if c == '#' {
            self.read_comment()
        } else if c == '%' {
            // Check for template
            self.advance(); // consume %
            if let Some('<') = self.peek() {
                self.read_template()
            } else {
                TokenKind::Unknown('%')
            }
        } else if c == '"' {
            self.read_string()
        } else if c == '{' {
            self.advance();
            TokenKind::OpenBrace
        } else if c == '}' {
            self.advance();
            TokenKind::CloseBrace
        } else if c == '[' {
            self.advance();
            TokenKind::OpenBracket
        } else if c == ']' {
            self.advance();
            TokenKind::CloseBracket
        } else if c == ',' {
            self.advance();
            TokenKind::Comma
        } else if c.is_ascii_digit() || c == '-' || c == '+' || c == '.' {
            self.read_number_or_symbol()
        } else if is_identifier_start(c) {
            self.read_identifier()
        } else {
            self.advance();
            TokenKind::Unknown(c)
        };

        Token {
            kind,
            span: Span::new(
                start_offset,
                self.byte_offset,
                start_line,
                start_col,
                self.line,
                self.col,
            ),
        }
    }

    fn read_whitespace(&mut self) -> TokenKind {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '\n' || c == '\r' {
                if s.is_empty() {
                    // Start of newline sequence
                    s.push(self.advance().unwrap());
                    // Check for \r\n
                    if c == '\r' && self.peek() == Some('\n') {
                        s.push(self.advance().unwrap());
                    }
                    return TokenKind::Newline(s);
                } else {
                    // We were reading spaces/tabs, now hit newline. Stop.
                    break;
                }
            } else if c.is_whitespace() {
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        TokenKind::Whitespace(s)
    }

    fn read_comment(&mut self) -> TokenKind {
        let mut s = String::new();
        // Consume #
        s.push(self.advance().unwrap());
        while let Some(c) = self.peek() {
            if c == '\n' || c == '\r' {
                break;
            }
            s.push(self.advance().unwrap());
        }
        TokenKind::Comment(s)
    }

    fn read_template(&mut self) -> TokenKind {
        // We already consumed '%'
        // Next should be '<'
        self.advance(); // Consume '<'

        let mut is_expr = false;
        if self.peek() == Some('=') {
            is_expr = true;
            self.advance(); // Consume '='
        }

        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '>' {
                self.advance();
                if self.peek() == Some('%') {
                    self.advance(); // Consume '%'
                    return TokenKind::Template {
                        content: s,
                        is_expression: is_expr,
                        terminated: true,
                    };
                } else {
                    s.push('>');
                }
            } else {
                s.push(self.advance().unwrap());
            }
        }

        // EOF inside template
        TokenKind::Template {
            content: s,
            is_expression: is_expr,
            terminated: false,
        }
    }

    fn read_string(&mut self) -> TokenKind {
        self.advance(); // "
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                return TokenKind::Str(s);
            } else if c == '\\' {
                self.advance();
                if let Some(esc) = self.peek() {
                    if esc == 'n' {
                        s.push('\n');
                    } else if esc == 'r' {
                        s.push('\r');
                    } else if esc == 't' {
                        s.push('\t');
                    } else if esc == '\\' {
                        s.push('\\');
                    } else if esc == '"' {
                        s.push('"');
                    } else {
                        s.push('\\');
                        s.push(esc);
                    }
                    self.advance();
                }
            } else {
                s.push(self.advance().unwrap());
            }
        }
        TokenKind::Unknown('"') // Unclosed string
    }

    fn read_identifier(&mut self) -> TokenKind {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if is_identifier_char(c) {
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        if s == "DEF" {
            TokenKind::Def
        } else if s == "USE" {
            TokenKind::Use
        } else if s == "PROTO" {
            TokenKind::Proto
        } else if s == "EXTERNPROTO" {
            TokenKind::ExternProto
        } else if s == "field" {
            TokenKind::Field
        } else if s == "vrmlField" {
            TokenKind::VrmlField
        } else if s == "hiddenField" {
            TokenKind::HiddenField
        } else if s == "deprecatedField" {
            TokenKind::DeprecatedField
        } else if s == "IS" {
            TokenKind::Is
        } else if s == "NULL" {
            TokenKind::Null
        } else if s == "TRUE" {
            TokenKind::True
        } else if s == "FALSE" {
            TokenKind::False
        } else {
            TokenKind::Identifier(s)
        }
    }

    fn read_number_or_symbol(&mut self) -> TokenKind {
        let start_char = self.peek().unwrap();
        let mut s = String::new();

        // Handle sign
        if let Some(c) = self.peek()
            && (c == '-' || c == '+')
        {
            s.push(self.advance().unwrap());
        }

        let mut has_dot = false;
        let mut has_digit = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                has_digit = true;
                s.push(self.advance().unwrap());
            } else if c == '.' {
                if has_dot {
                    break;
                }
                has_dot = true;
                s.push(self.advance().unwrap());
            } else if c == 'e' || c == 'E' {
                s.push(self.advance().unwrap());
                if let Some(sign) = self.peek()
                    && (sign == '+' || sign == '-')
                {
                    s.push(self.advance().unwrap());
                }
                while let Some(d) = self.peek() {
                    if d.is_ascii_digit() {
                        s.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
                break;
            } else {
                break;
            }
        }

        if !has_digit && !has_dot {
            return TokenKind::Unknown(start_char);
        }

        if has_dot || s.contains('e') || s.contains('E') {
            if let Ok(f) = s.parse::<f64>() {
                return TokenKind::Float(f, s);
            }
        } else {
            if let Ok(i) = s.parse::<i64>() {
                return TokenKind::Int(i, s);
            }
            if let Ok(f) = s.parse::<f64>() {
                return TokenKind::Float(f, s);
            }
        }

        TokenKind::Unknown(start_char)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '+' || c == '-'
}
