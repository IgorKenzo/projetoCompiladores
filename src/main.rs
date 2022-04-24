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
    
    parser(content, true);
}

pub fn parser(code: String, debug: bool) {

    let mut lexer = Lexer::new(String::from(code));

    program(&mut lexer, debug);

    // debug_lexer(lexer);
    // debug_lexer_peek(lexer);
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

#[allow(dead_code)]
fn debug_lexer_peek(mut lexer: Lexer) {
    
    let mut tok = lexer.peek_token();
    println!("TOKEN: {}, TIPO: {:?}", tok.value, tok.t_type);

    tok = lexer.next_token();
    println!("TOKEN: {}, TIPO: {:?}", tok.value, tok.t_type);
    
}

pub fn read_token_type(lexer: &mut Lexer, token_type : TokenType) {
    let tok = lexer.next_token();
    if tok.t_type != token_type {
        eprintln!("Syntax error");
        exit(1);
    }
}


pub fn program(lexer: &mut Lexer, debug: bool) {
    if debug { println!("program"); }

    read_token_type(lexer, TokenType::ReservMain);
    block(lexer, debug);
    // read_token_type(lexer, TokenType::LCol);
    // num(lexer);
    // read_token_type(lexer, TokenType::RCol);

}

pub fn block(lexer: &mut Lexer, debug: bool) {
    if debug { println!("block"); }
    //block ::= "{" statement-list "}"
    read_token_type(lexer, TokenType::LCol);
    statement_list(lexer, debug);
    read_token_type(lexer, TokenType::RCol);
}

pub fn statement_list(lexer: &mut Lexer, debug: bool) {
    if debug { println!("statement_list"); }
    //statement-list ::= statement*
    while lexer.peek(1) != '}' {
        statement(lexer, debug);
    }
}

pub fn statement(lexer: &mut Lexer, debug: bool) {
    if debug { println!("statement"); }
    //statement ::= (assignment-statement ";"| structured-statement | var-declare-statement ";" | expression ";") 
    if lexer.peek_token().t_type == TokenType::ReservLet {
        var_declare_statement(lexer, debug);
        read_token_type(lexer, TokenType::SemiCol);
    }
    else if lexer.peek_token().t_type == TokenType::ReservWhile { // do while; for; depois
        structured_statement(lexer,0, debug);
    }
    else if lexer.peek_token().t_type == TokenType::ReservIf { //switch depois
        structured_statement(lexer, 1, debug);
    }
    else { //arrumar
        expression(lexer, debug);
    }   
}

pub fn assignment_statement(lexer: &mut Lexer, debug: bool) {
    if debug { println!("assignment_statement"); }
    //assignment-statement ::= variable assignment-operator expression 
    read_token_type(lexer, TokenType::SemiCol);    
}
pub fn structured_statement(lexer: &mut Lexer, i: u8, debug: bool) {
    if debug { println!("structured_statement"); }
    //structured-statement ::= loop-statement | conditional-statement
    match i {
        0 => loop_statement(lexer, debug),
        1 => conditional_statement(lexer, debug),
        _ => println!("STRUCTURED CODE NOT FOUND")
    }
}
pub fn var_declare_statement(lexer: &mut Lexer, debug: bool) {
    if debug { println!("var_declare_statement"); }
    //var-declare-statement ::= 'let' variable ':' type-specifier ( '=' expression )?

    read_token_type(lexer, TokenType::ReservLet);
    // ler identificador
    read_token_type(lexer, TokenType::Colon);
    // ler type

    if lexer.peek_token().t_type == TokenType::OpAssign {
        read_token_type(lexer, TokenType::OpAssign);
        expression(lexer, debug);
    }

}

pub fn loop_statement(lexer: &mut Lexer, debug: bool) {
    if debug { println!("loop_statement"); }
    //loop-statement ::= while-statement
    while_statement(lexer, debug);
}

pub fn conditional_statement(lexer: &mut Lexer, debug: bool) {
    if debug { println!("conditional_statetement"); }
    //conditional-statement ::= if-statement 
    if_statement(lexer, debug);
}

pub fn while_statement(lexer: &mut Lexer, debug: bool) {
    if debug { println!("while_statement"); }
    //while-statement ::= "while" expression block
    read_token_type(lexer, TokenType::ReservWhile);
    expression(lexer, debug);
    block(lexer, debug);
}

pub fn if_statement(lexer: &mut Lexer, debug: bool) {
    if debug { println!("if_statement"); }
    //if-statement ::= "if" expression block ('else' block)?
    read_token_type(lexer, TokenType::ReservIf);
    expression(lexer, debug);
    block(lexer, debug);
    if lexer.peek_token().t_type == TokenType::ReservElse { //criar outro peek de token
        read_token_type(lexer, TokenType::ReservElse);
        block(lexer, debug);
    }
}


pub fn expression(lexer: &mut Lexer, debug: bool) {
    if debug { println!("expression"); }
    // expression ::= simple-expression (relational-operator simple-expression)*
    simple_expression(lexer, debug);
    read_token_type(lexer, TokenType::SemiCol);    
}

pub fn simple_expression(lexer: &mut Lexer, debug: bool) {
    if debug { println!("simple_expression"); }
    //(sign)? term (addition-operator term)* | '"' [a-zA-Z]* '"' | "'" [a-zA-z] "'"
    //fazer sign
    term(lexer, debug);
    //fazer while
}

pub fn term(lexer: &mut Lexer, debug: bool) {
    if debug { println!("term"); }
    //term ::= factor (multiplication-operator factor)*
    factor(lexer, debug);
}
pub fn addition_operator(lexer: &mut Lexer, debug: bool) {
    if debug { println!("addition_operator"); }
    //addition-operator ::= "+" | "-" | "||" 
    
}

pub fn factor(lexer: &mut Lexer, debug: bool) {
    if debug { println!("factor"); }
    //factor ::= '!'* ( variable | number | string | '(' expression ')' )
    let tok = lexer.next_token();

    if tok.t_type == TokenType::Identificador || tok.t_type == TokenType::Literal || tok.t_type == TokenType::StringLiteral {
        println!("AEHO não sei oq fazer");
    }

    if tok.t_type == TokenType::RPar {
        expression(lexer, debug);
        read_token_type(lexer, TokenType::RPar);
    }

}
pub fn multiplication_operator(lexer: &mut Lexer, debug: bool) {
    if debug { println!("multiplication_operator"); }
    //multiplication-operator ::= "*" | "/" | div | mod | "&&" 

}

// pub fn identifier(lexer: &mut Lexer) {
//     //TokenType::identifier
// }

pub fn num(lexer: &mut Lexer, debug: bool) {
    let token = lexer.next_token();

    if token.t_type == TokenType::Literal {
        println!("{}",token.value);
    } else {
        exit(1);
    }
}
