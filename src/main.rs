mod token;
mod lexer;
mod parser;
use lexer::Lexer;
use token::Token;
use token::TokenType;
use token::VarStruct;
use token::VarType;


use std::env;


use std::fs;
use std::process::exit;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Arquivo não encontrado");
    
    parser(content, false);
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
    let mut simbolos : HashMap<String, VarStruct> = HashMap::new();
    read_token_type(lexer, TokenType::LCol);
    statement_list(lexer, debug, &mut simbolos);
    read_token_type(lexer, TokenType::RCol);
}

pub fn statement_list(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
    if debug { println!("statement_list"); }
    //statement-list ::= statement*
    while lexer.peek_token().t_type != TokenType::RCol {
        statement(lexer, debug, simbolos);
    }
}

pub fn statement(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
    if debug { println!("statement"); }
    //statement ::= (assignment-statement ";"| structured-statement | var-declare-statement ";" | expression ";") 
    if lexer.peek_token().t_type == TokenType::ReservLet {
        var_declare_statement(lexer, debug, simbolos);
        read_token_type(lexer, TokenType::SemiCol);
    }
    else if lexer.peek_token().t_type == TokenType::ReservWhile { // do while; for; depois
        structured_statement(lexer,0, debug, simbolos);
    }
    else if lexer.peek_token().t_type == TokenType::ReservIf { //switch depois
        structured_statement(lexer, 1, debug, simbolos);
    }
    else if lexer.peek_token().t_type == TokenType::Identificador && lexer.peek_nth_token(2).t_type == TokenType::OpAssign {
        assignment_statement(lexer, debug, simbolos);
        read_token_type(lexer, TokenType::SemiCol);    
    }
    else { //arrumar
        expression(lexer, debug, simbolos);
        read_token_type(lexer, TokenType::SemiCol);
    }   
}

pub fn assignment_statement(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
    if debug { println!("assignment_statement"); }
    //assignment-statement ::= variable assignment-operator expression 
    let iden = lexer.next_token();

    read_token_type(lexer, TokenType::OpAssign);

    let val = expression(lexer, debug, simbolos);

    simbolos.get_mut(&iden.value).unwrap().value = val.to_string();
    println!("{:?}", simbolos);
}

pub fn structured_statement(lexer: &mut Lexer, i: u8, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
    if debug { println!("structured_statement"); }
    //structured-statement ::= loop-statement | conditional-statement
    match i {
        0 => loop_statement(lexer, debug, simbolos),
        1 => conditional_statement(lexer, debug, simbolos),
        _ => println!("STRUCTURED CODE NOT FOUND")
    }
}

pub fn var_declare_statement(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
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
    
    
    match tipo.t_type {
        TokenType::DataType(t) => {
            simbolos.insert(String::from(&iden.value) , VarStruct { value: String::from(""), v_type: t });
        }
        _ => {}
    }
    

    if lexer.peek_token().t_type == TokenType::OpAssign {
        read_token_type(lexer, TokenType::OpAssign);
        let val = expression(lexer, debug, simbolos);
        simbolos.get_mut(&iden.value).unwrap().value = val.to_string();
    }

    println!("{:?}", simbolos);
}

pub fn loop_statement(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
    if debug { println!("loop_statement"); }
    //loop-statement ::= while-statement
    while_statement(lexer, debug, simbolos);
}

pub fn conditional_statement(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
    if debug { println!("conditional_statetement"); }
    //conditional-statement ::= if-statement 
    if_statement(lexer, debug, simbolos);
}

pub fn while_statement(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
    if debug { println!("while_statement"); }
    //while-statement ::= "while" expression block
    read_token_type(lexer, TokenType::ReservWhile);
    expression(lexer, debug, simbolos);
    block(lexer, debug);
}

pub fn if_statement(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) {
    if debug { println!("if_statement"); }
    //if-statement ::= "if" expression block ('else' block)?
    read_token_type(lexer, TokenType::ReservIf);
    expression(lexer, debug, simbolos);
    block(lexer, debug);
    if lexer.peek_token().t_type == TokenType::ReservElse { //criar outro peek de token
        read_token_type(lexer, TokenType::ReservElse);
        block(lexer, debug);
    }
}


pub fn expression(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> i32 {
    if debug { println!("expression"); }
    // expression ::= simple-expression (relational-operator simple-expression)*
    let v = simple_expression(lexer, debug, simbolos);
    
    println!("Expression: {}",v);
    v
}

pub fn simple_expression(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> i32 {
    if debug { println!("simple_expression"); }
    //(sign)? term (addition-operator term)* | '"' [a-zA-Z]* '"' | "'" [a-zA-z] "'"
    //fazer sign
    let mut t = term(lexer, debug, simbolos);
    //fazer while
    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpSum || temp == TokenType::OpMinus  {
        let op = addition_operator(lexer, debug);
        let t2 = term(lexer, debug, simbolos);

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

pub fn term(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> i32 {
    if debug { println!("term"); }
    //term ::= factor (multiplication-operator factor)*

    let mut t = power(lexer, debug, simbolos);
    //fazer while
    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpMult || temp == TokenType::OpDiv || temp == TokenType::OpMod  {
        let op = multiplication_operator(lexer, debug);
        let t2 = power(lexer, debug, simbolos);

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

// ------- ARRUMAR
pub fn addition_operator(lexer: &mut Lexer, debug: bool) -> TokenType {
    if debug { println!("addition_operator"); }
    //addition-operator ::= "+" | "-" | "||" 
    let tok = lexer.next_token();
    tok.t_type
}

pub fn multiplication_operator(lexer: &mut Lexer, debug: bool) -> TokenType {
    if debug { println!("multiplication_operator"); }
    //multiplication-operator ::= "*" | "/" | % | "&&" 
    let tok = lexer.next_token();
    tok.t_type
}

pub fn power_operator(lexer: &mut Lexer, debug: bool) -> TokenType {
    if debug { println!("power_operator"); }
    //multiplication-operator ::= "*" | "/" | % | "&&" 
    let tok = lexer.next_token();
    tok.t_type
}
// -------

pub fn power(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> i32 {

    let mut t = factor(lexer, debug, simbolos);
    //fazer while
    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpPower {
        let op = power_operator(lexer, debug);
        let t2 = factor(lexer, debug, simbolos);

        if op == TokenType::OpPower {
            t = t.pow(t2 as u32);
        }
       
        temp = lexer.peek_token().t_type;
    }

    t
}

pub fn factor(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> i32 {
    if debug { println!("factor"); }
    //factor ::= '!'* ( variable | number | string | '(' expression ')' )
    let tok = lexer.next_token();
    // println!("TOK {:?}; PEEK {:?}", tok.value, lexer.peek_token().value);

    if tok.t_type == TokenType::Identificador {
        let v_stru = simbolos.get_mut(&tok.value).unwrap();
        match v_stru.v_type {
            VarType::Int => { return v_stru.value.parse::<i32>().unwrap(); }
            _ => {return i32::MIN;}
        }
    }

    if  tok.t_type == TokenType::Literal { //|| tok.t_type == TokenType::StringLiteral
        // println!("AEHO não sei oq fazer");
        return tok.value.parse::<i32>().unwrap();
    }
    
    if tok.t_type == TokenType::LPar {
        
        let t = expression(lexer, debug, simbolos);
        
        read_token_type(lexer, TokenType::RPar);
        return t;
    }

    -1
}

// pub fn identifier(lexer: &mut Lexer) {
//     //TokenType::identifier
// }

// pub fn num(lexer: &mut Lexer, debug: bool) {
//     let token = lexer.next_token();

//     if token.t_type == TokenType::Literal {
//         println!("{}",token.value);
//     } else {
//         exit(1);
//     }
// }
