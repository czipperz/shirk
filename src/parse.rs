use diagnostics;
use file_position::FilePosition;
use lex;
use std::vec::Vec;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Name(String),
    Comma(BinaryExpression),
    Assign(BinaryExpression),
    Minus(BinaryExpression),
    Plus(BinaryExpression),
}

#[derive(Debug)]
pub enum TypeExpression {
    Pointer(Box<TypeExpression>),
    Name(String),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<VariableDeclaration>,
    pub return_type: TypeExpression,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum DefiningTypeExpression {
    TypeExpression(TypeExpression),
    Function(Function),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: DefiningTypeExpression,
}

#[derive(Debug)]
pub enum Statement {
    If(Box<Expression>, Vec<Statement>, Vec<Statement>),
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
}

pub type ParseResult<T> = Result<T, ()>;

struct Peekable<I: Iterator> {
    iter: I,
    peeked: Option<I::Item>,
}

impl<I: Iterator> Peekable<I> {
    fn peek(&mut self) -> &mut Option<I::Item> {
        if self.peeked.is_none() {
            self.peeked = self.iter.next();
        }
        &mut self.peeked
    }
}

impl<I: Iterator> Iterator for Peekable<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.peeked.take() {
            Some(v) => Some(v),
            None => self.iter.next(),
        }
    }
}

type TokenIter = Peekable<IntoIter<lex::Token>>;

fn parse_namespaced_word(tokens: &mut TokenIter)
                         -> ParseResult<(String, FilePosition)> {
    let mut word: String;
    let fpos: FilePosition;
    match tokens.next() {
        Some(lex::Token { data: lex::TokenData::Word(w), 
                          fpos: f }) => {
            word = w;
            fpos = f;
        },
        Some(lex::Token { data: _,
                          fpos: fpos }) => {
            diagnostics::print_error_pos(
                format_args!("Did not find a word"),
                &fpos);
            return Err(());
        },
        None => {
            diagnostics::print_error(
                format_args!("EOF when expected a word"));
            return Err(());
        },
    }
    loop {
        let nsfpos;
        match tokens.peek() {
            &mut Some(lex::Token { data: lex::TokenData::Namespace,
                                   fpos: _ }) => {
                nsfpos = tokens.next().unwrap().fpos;
                word.push_str("::");
            },
            _ => return Ok((word, fpos)),
        }
        match tokens.next() {
            Some(lex::Token { data: lex::TokenData::Word(ref w),
                              fpos: _ }) => {
                word.push_str(w.as_ref());
            },
            Some(lex::Token { data: _,
                              fpos: fpos }) => {
                diagnostics::print_error_pos(
                    format_args!("Did not find a word after the namespace"),
                    &fpos);
                return Err(());
            },
            None => {
                diagnostics::print_error_pos(
                    format_args!(""));
                return Err(());
            }
        }
    }
}

pub fn parse(mut tokens: IntoIter<lex::Token>)
             -> ParseResult<Vec<VariableDeclaration>> {
    let mut tokens: TokenIter = Peekable { iter: tokens,
                                           peeked: None };
    let mut variables = Vec::new();
    loop {
        let (name, fpos) = try!(parse_namespaced_word(&mut tokens));
        diagnostics::print_fpos(&fpos);
        println!("{}", name);
        if let &mut Some(lex::Token { data: lex::TokenData::Colon,
                                      fpos: _ }) = tokens.peek() {
            
        }
    }
    Ok(variables)
}
