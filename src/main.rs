mod token;
mod lexer;
mod parser;
use lexer::Lexer;
use token::Token;
use token::TokenType;


use std::env;


use std::fs;
use std::process::exit;
// use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Arquivo não encontrado");
    
    parser(content, false);
}

pub fn parser(code: String, debug: bool) {

    let mut lexer = Lexer::new(String::from(code));

    // let mut simbolos : HashMap<&str, bool> = HashMap::new();

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

pub fn cmp_token_type(t1 : TokenType, t2 : TokenType) {
    
    if t1 != t2 {
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
        read_token_type(lexer, TokenType::SemiCol);
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

    //let
    read_token_type(lexer, TokenType::ReservLet);

    // ler identificador
    let iden = lexer.next_token();

    assert_eq!(iden.t_type, TokenType::Identificador);

    //;
    read_token_type(lexer, TokenType::Colon);

    // ler type
    let tipo = lexer.next_token();
    

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


pub fn expression(lexer: &mut Lexer, debug: bool) -> i32 {
    if debug { println!("expression"); }
    // expression ::= simple-expression (relational-operator simple-expression)*
    let v = simple_expression(lexer, debug);
    
    println!("Expression: {}",v);
    v
}

pub fn simple_expression(lexer: &mut Lexer, debug: bool) -> i32 {
    if debug { println!("simple_expression"); }
    //(sign)? term (addition-operator term)* | '"' [a-zA-Z]* '"' | "'" [a-zA-z] "'"
    //fazer sign
    let mut t = term(lexer, debug);
    //fazer while
    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpSum || temp == TokenType::OpMinus  {
        let op = addition_operator(lexer, debug);
        let t2 = term(lexer, debug);

        if op == TokenType::OpSum {
            t += t2;
        }
        else if op == TokenType::OpMinus {
            t -= t2;
        }

        temp = lexer.peek_token().t_type;
    }

    t
}

pub fn term(lexer: &mut Lexer, debug: bool) -> i32 {
    if debug { println!("term"); }
    //term ::= factor (multiplication-operator factor)*

    let mut t = factor(lexer, debug);
    //fazer while
    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpMult || temp == TokenType::OpDiv || temp == TokenType::OpMod  {
        let op = multiplication_operator(lexer, debug);
        let t2 = term(lexer, debug);

        if op == TokenType::OpMult {
            t *= t2;
        }
        else if op == TokenType::OpDiv {
            t /= t2;
        }
        else if op == TokenType::OpMod {
            t %= t2;
        }

        temp = lexer.peek_token().t_type;
    }

    t
}

pub fn addition_operator(lexer: &mut Lexer, debug: bool) -> TokenType {
    if debug { println!("addition_operator"); }
    //addition-operator ::= "+" | "-" | "||" 
    let tok = lexer.next_token();
    tok.t_type
}

pub fn factor(lexer: &mut Lexer, debug: bool) -> i32 {
    if debug { println!("factor"); }
    //factor ::= '!'* ( variable | number | string | '(' expression ')' )
    let tok = lexer.next_token();
    // println!("TOK {:?}; PEEK {:?}", tok.value, lexer.peek_token().value);
    if  tok.t_type == TokenType::Literal { //tok.t_type == TokenType::Identificador || || tok.t_type == TokenType::StringLiteral
        // println!("AEHO não sei oq fazer");
        return tok.value.parse::<i32>().unwrap();
    }
    
    if tok.t_type == TokenType::LPar {
        
        let t = expression(lexer, debug);
        
        read_token_type(lexer, TokenType::RPar);
        return t;
    }

    -1
}
pub fn multiplication_operator(lexer: &mut Lexer, debug: bool) -> TokenType {
    if debug { println!("multiplication_operator"); }
    //multiplication-operator ::= "*" | "/" | % | "&&" 
    let tok = lexer.next_token();
    tok.t_type
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
