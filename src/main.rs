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

#[allow(dead_code)]
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
    //block ::= "{" statement-list "}"
    read_token_type(lexer, TokenType::LCol);
    statement_list(lexer);
    read_token_type(lexer, TokenType::RCol);
}

pub fn statement_list(lexer: &mut Lexer) {
    //statement-list ::= statement*
    while lexer.peek(1) != '}' {
        statement(lexer);
    }
}

pub fn statement(lexer: &mut Lexer) {
    //statement ::= (assignment-statement ";"|structured-statement|var-declare-statement ";" | expression ";") 
    expression(lexer);
}

pub fn assignment_statement(lexer: &mut Lexer) {
    //assignment-statement ::= variable assignment-operator expression 
    read_token_type(lexer, TokenType::SemiCol);    
}
pub fn structured_statement(lexer: &mut Lexer) {
    //structured-statement ::= loop-statement | conditional-statement
    read_token_type(lexer, TokenType::SemiCol);
}
pub fn var_declare_statement(lexer: &mut Lexer) {
    //var-declare-statement ::= 'let' variable ':' type-specifier ( '=' expression )?
    read_token_type(lexer, TokenType::SemiCol);
}

pub fn loop_statement(lexer: &mut Lexer) {
    //loop-statement ::= while-statement
    while_statement(lexer);
}

pub fn conditional_statement(lexer: &mut Lexer) {
    //conditional-statement ::= if-statement 
    if_statement(lexer);
}

pub fn while_statement(lexer: &mut Lexer) {
    //while-statement ::= "while" expression block
    read_token_type(lexer, TokenType::ReservWhile);
    expression(lexer);
    block(lexer);
}

pub fn if_statement(lexer: &mut Lexer) {
    //if-statement ::= "if" expression block ('else' block)?
    read_token_type(lexer, TokenType::ReservIf);
    expression(lexer);
    block(lexer);
    // if lexer.peek(1) == TokenType::ReservElse { //criar outro peek de token
    //     read_token_type(lexer, TokenType::ReservElse);
    //     block(lexer);
    // }
}


pub fn expression(lexer: &mut Lexer) {
    // expression ::= simple-expression (relational-operator simple-expression)*
    simple_expression(lexer);
    read_token_type(lexer, TokenType::SemiCol);    
}

pub fn simple_expression(lexer: &mut Lexer) {
    //(sign)? term (addition-operator term)* | '"' [a-zA-Z]* '"' | "'" [a-zA-z] "'"
    num(lexer);
}

pub fn term(lexer: &mut Lexer) {
    //term ::= factor (multiplication-operator factor)*
    
}
pub fn addition_operator(lexer: &mut Lexer) {
    //addition-operator ::= "+" | "-" | "||" 
}

pub fn factor(lexer: &mut Lexer) {
    //factor ::= '!'* ( variable | number | string | '(' expression ')' )


}
pub fn multiplication_operator(lexer: &mut Lexer) {
    //multiplication-operator ::= "*" | "/" | div | mod | "&&" 

}

pub fn identifier(lexer: &mut Lexer) {
    //TokenType::identifier
}

pub fn num(lexer: &mut Lexer) {
    let token = lexer.next_token();

    if token.t_type == TokenType::Literal {
        println!("{}",token.value);
    } else {
        exit(1);
    }
}
