mod token;
mod lexer;
mod parser;
use lexer::Lexer;
use token::Token;
use token::TokenType;


use std::env;


use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Arquivo não encontrado");
    
    parser(content);
}

pub fn parser(code: String) {

    let mut lexer = Lexer::new(String::from(code));

    program(&mut lexer);

    // debug_lexer(lexer);
}

fn debug_lexer(mut lexer: Lexer) {
    let mut tok : Token;
    loop {
        tok = lexer.next_token();
        if matches!(tok.t_type,TokenType::EoF) {
            break;
        }

        println!("TOKEN: {}, TIPO: {:?}", tok.value, tok.t_type);
    }
}

pub fn read_token_type(lexer: &mut Lexer, token_type : TokenType) {
    let tok = lexer.next_token();
    if tok.t_type != token_type {
        eprintln!("Syntax error");
        exit(1);
    }
}


pub fn program(lexer: &mut Lexer) {

    read_token_type(lexer, TokenType::ReservMain);
    block(lexer);
    // read_token_type(lexer, TokenType::LCol);
    // num(lexer);
    // read_token_type(lexer, TokenType::RCol);

}

pub fn block(lexer: &mut Lexer) {
    read_token_type(lexer, TokenType::LCol);
    statement_list(lexer);
    read_token_type(lexer, TokenType::RCol);
}

pub fn statement_list(lexer: &mut Lexer) {
    while lexer.peek(1) != '}' {
        statement(lexer);
    }
}

pub fn statement(lexer: &mut Lexer) {
    num(lexer);
}

pub fn assignment_statement(lexer: &mut Lexer) {
    read_token_type(lexer, TokenType::SemiCol);    
}
pub fn structured_statement(lexer: &mut Lexer) {
    read_token_type(lexer, TokenType::SemiCol);
}
pub fn var_declare_statement(lexer: &mut Lexer) {
    read_token_type(lexer, TokenType::SemiCol);
}
pub fn expression(lexer: &mut Lexer) {
    // expression ::= simple-expression (relational-operator simple-expression)*
    
    read_token_type(lexer, TokenType::SemiCol);    
}

pub fn num(lexer: &mut Lexer) {
    let token = lexer.next_token();

    if token.t_type == TokenType::Literal {
        println!("{}",token.value);
    } else {
        exit(1);
    }
}
