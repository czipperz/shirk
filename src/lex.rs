use diagnostics;
use file_position::FilePosition;
use std::vec::Vec;
use std::io::Bytes;
use std::io::Read;
use std::fmt;

#[derive(Debug)]
pub enum TokenData {
    CloseCurly,
    CloseParen,
    Colon,
    Fun,
    Namespace,
    OpenCurly,
    OpenParen,
    Return,
    RightArrow,
    Semicolon,
    Struct,
    Word(String),

    Comma,
    Assign,
    Minus,
    Plus,
}

impl fmt::Display for TokenData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self {
                   &TokenData::CloseCurly => "}",
                   &TokenData::CloseParen => ")",
                   &TokenData::Colon => ":",
                   &TokenData::Fun => "fn",
                   &TokenData::Namespace => "::",
                   &TokenData::OpenCurly => "{",
                   &TokenData::OpenParen => "(",
                   &TokenData::Return => "return",
                   &TokenData::RightArrow => "->",
                   &TokenData::Semicolon => ";",
                   &TokenData::Struct => "struct",
                   &TokenData::Word(ref s) => s.as_ref(),
                   &TokenData::Comma => ",",
                   &TokenData::Assign => "=",
                   &TokenData::Minus => "-",
                   &TokenData::Plus => "+",
               })
    }
}

pub struct Token {
    pub data: TokenData,
    pub fpos: FilePosition,
}

pub struct FileIterator<T> {
    pub bytes: Bytes<T>,
    pub fpos: FilePosition,
}

impl<T: Read> FileIterator<T> {
    pub fn next(&mut self) -> Option<char> {
        return self.bytes.next()
            .map(|x| x.unwrap() as char)
            .map(|x| if x == '\n' {
                self.fpos.line += 1;
                self.fpos.column = 0;
                '\n'
            } else {
                self.fpos.column += 1;
                x
            }
            );
    }
}

pub fn lex<T: Read>(mut fiter: FileIterator<T>) -> (Vec<Token>, bool) {
    // force drop through to end of the loop.
    let mut c: char = ' ';
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        let fpos = fiter.fpos.clone();
        if c.is_whitespace() {
        } else if c.is_alphabetic() {
            let mut s = String::new();
            s.push(c);
            loop {
                match fiter.next() {
                    Some(ch) => {
                        if ch.is_alphanumeric() || ch == '_' {
                            s.push(ch);
                        } else {
                            c = ch;
                            break;
                        }
                    },
                    None => break,
                }
            }
            tokens.push(Token { fpos: fpos,
                                data:
                                if s == "fun" {
                                    TokenData::Fun
                                } else if s == "struct" {
                                    TokenData::Struct
                                } else if s == "return" {
                                    TokenData::Return
                                } else {
                                    TokenData::Word(s)
                                }
            });
            continue;
        } else if c == '-' {
            match fiter.next() {
                Some('>') => {
                    tokens.push(Token { fpos: fpos,
                                        data: TokenData::RightArrow });
                },
                Some(ch) => {
                    tokens.push(Token { fpos: fpos,
                                        data: TokenData::Minus });
                    c = ch;
                    continue;
                },
                None => {
                    tokens.push(Token { fpos: fpos,
                                        data: TokenData::Minus });
                    break;
                },
            }
        } else if c == ':' {
            match fiter.next() {
                Some(':') => {
                    tokens.push(Token { fpos: fpos,
                                        data: TokenData::Namespace });
                },
                Some(ch) => {
                    tokens.push(Token { fpos: fpos,
                                        data: TokenData::Colon });
                    c = ch;
                    continue;
                },
                None => {
                    tokens.push(Token { fpos: fpos,
                                        data: TokenData::Colon });
                    break;
                },
            }
        } else {
            let data =
                match c {
                    '{' => TokenData::OpenCurly,
                    '}' => TokenData::CloseCurly,
                    '(' => TokenData::OpenParen,
                    ')' => TokenData::CloseParen,
                    ';' => TokenData::Semicolon,
                    '+' => TokenData::Plus,
                    '=' => TokenData::Assign,
                    ',' => TokenData::Comma,
                    _ => {
                        diagnostics::print_error_pos(
                            format_args!("Lexing error on seeing {}", c),
                            &fpos);
                        return (tokens, false);
                    }
                };
            tokens.push(Token { fpos: fpos,
                                data: data });
        }

        match fiter.next() {
            Some(ch) => c = ch,
            None => break,
        }
    }

    (tokens, true)
}
