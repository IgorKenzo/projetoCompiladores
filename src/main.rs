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
        eprintln!("Syntax error: {:?} era esperado", token_type);
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

    let mut v = simbolos.get_mut(&iden.value).unwrap();

    if v.v_type != val.v_type {
        eprintln!("Atriubuição de tipo {:?} em variável de tipo {:?} não da certo", val.v_type, v.v_type); 
        exit(1);
    }
    
    v.value = val.value;

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

    //:
    read_token_type(lexer, TokenType::Colon);

    // ler type
    let tipo = lexer.next_token();
    
    match simbolos.get(&iden.value) {
        Some(_) => { 
            eprintln!("Variável {} já declarada", iden.value);
            exit(1);
        },
        None => {}
    }
    
    match tipo.t_type {
        TokenType::DataType(t) => {
            simbolos.insert(String::from(&iden.value) , VarStruct { value: String::from(""), v_type: t });
        }
        _ => {}
    }
    

    if lexer.peek_token().t_type == TokenType::OpAssign {
        read_token_type(lexer, TokenType::OpAssign);
        let val = expression(lexer, debug, simbolos);

        let mut v = simbolos.get_mut(&iden.value).unwrap();

        if v.v_type != val.v_type {
            eprintln!("Atriubuição de tipo {:?} em variável de tipo {:?} não da certo", val.v_type, v.v_type); 
            exit(1);
        }
        
        v.value = val.value;
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


pub fn expression(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> VarStruct {
    if debug { println!("expression"); }
    // expression ::= simple-expression (relational-operator simple-expression)*
    let mut se = simple_expression(lexer, debug, simbolos);

    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpRelLt|| temp == TokenType::OpRelLe|| temp == TokenType::OpRelGt|| temp == TokenType::OpRelGe|| temp == TokenType::OpRelEq|| temp == TokenType::OpRelNe {
        let op = lexer.next_token().t_type; // consome token de operador relacional
        let se2 = simple_expression(lexer, debug, simbolos);

        if se.v_type != se2.v_type { eprintln!("Tipos incompatíveis {:?} com {:?}", se.v_type, se2.v_type); exit(1); }

        if op == TokenType::OpRelLt {
            if se.v_type == VarType::Int {
                let v1 = se.value.parse::<i32>().unwrap();
                let v2 = se2.value.parse::<i32>().unwrap();
                let res = v1 < v2;
    
                se.value = res.to_string();
            }
            else if se.v_type == VarType::Double {
                let v1 = se.value.parse::<f32>().unwrap();
                let v2 = se2.value.parse::<f32>().unwrap();
                let res = v1 < v2;

                se.value = res.to_string();
            } 
        }
        else if op == TokenType::OpRelLe {
            if se.v_type == VarType::Int {
                let v1 = se.value.parse::<i32>().unwrap();
                let v2 = se2.value.parse::<i32>().unwrap();
                let res = v1 <= v2;
    
                se.value = res.to_string();
            }
            else if se.v_type == VarType::Double {
                let v1 = se.value.parse::<f32>().unwrap();
                let v2 = se2.value.parse::<f32>().unwrap();
                let res = v1 <= v2;

                se.value = res.to_string();
            } 
        }
        else if op == TokenType::OpRelGt {
            if se.v_type == VarType::Int {
                let v1 = se.value.parse::<i32>().unwrap();
                let v2 = se2.value.parse::<i32>().unwrap();
                let res = v1 > v2;
    
                se.value = res.to_string();
            }
            else if se.v_type == VarType::Double {
                let v1 = se.value.parse::<f32>().unwrap();
                let v2 = se2.value.parse::<f32>().unwrap();
                let res = v1 > v2;

                se.value = res.to_string();
            } 
        }
        else if op == TokenType::OpRelGe {
            if se.v_type == VarType::Int {
                let v1 = se.value.parse::<i32>().unwrap();
                let v2 = se2.value.parse::<i32>().unwrap();
                let res = v1 >= v2;
    
                se.value = res.to_string();
            }
            else if se.v_type == VarType::Double {
                let v1 = se.value.parse::<f32>().unwrap();
                let v2 = se2.value.parse::<f32>().unwrap();
                let res = v1 >= v2;

                se.value = res.to_string();
            } 
        }
        if op == TokenType::OpRelEq {
            if se.v_type == VarType::Int {
                let v1 = se.value.parse::<i32>().unwrap();
                let v2 = se2.value.parse::<i32>().unwrap();
                let res = v1 == v2;
    
                se.value = res.to_string();
            }
            else if se.v_type == VarType::Double {
                let v1 = se.value.parse::<f32>().unwrap();
                let v2 = se2.value.parse::<f32>().unwrap();
                let res = v1 == v2;

                se.value = res.to_string();
            }
            else if se.v_type == VarType::Bool {
                let v1 = se.value.parse::<bool>().unwrap();
                let v2 = se2.value.parse::<bool>().unwrap();
                let res = v1 == v2;

                se.value = res.to_string();
            }
        }
        if op == TokenType::OpRelNe {
            if se.v_type == VarType::Int {
                let v1 = se.value.parse::<i32>().unwrap();
                let v2 = se2.value.parse::<i32>().unwrap();
                let res = v1 != v2;
    
                se.value = res.to_string();
            }
            else if se.v_type == VarType::Double {
                let v1 = se.value.parse::<f32>().unwrap();
                let v2 = se2.value.parse::<f32>().unwrap();
                let res = v1 != v2;

                se.value = res.to_string();
            }
            else if se.v_type == VarType::Bool {
                let v1 = se.value.parse::<bool>().unwrap();
                let v2 = se2.value.parse::<bool>().unwrap();
                let res = v1 != v2;

                se.value = res.to_string();
            }
        }

        se.v_type = VarType::Bool;
        temp = lexer.peek_token().t_type;
    }
    println!("Expression: {:?}",se);
    se
}

pub fn simple_expression(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> VarStruct {
    if debug { println!("simple_expression"); }
    //(sign)? term (addition-operator term)* 
    //fazer sign
    let mut t = term(lexer, debug, simbolos);
    //fazer while
    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpSum || temp == TokenType::OpMinus || temp == TokenType::OpLogOr {
        let op = addition_operator(lexer, debug);
        let t2 = term(lexer, debug, simbolos);

        if t.v_type != t2.v_type { eprintln!("Tipos incompatíveis {:?} com {:?}", t.v_type, t2.v_type); exit(1); }

        if op == TokenType::OpSum {
            if t.v_type == VarType::Int {
                let mut v1 = t.value.parse::<i32>().unwrap();
                let v2 = t2.value.parse::<i32>().unwrap();
                v1 += v2;
    
                t.value = v1.to_string();
            }
            else if t.v_type == VarType::Double {
                let mut v1 = t.value.parse::<f32>().unwrap();
                let v2 = t2.value.parse::<f32>().unwrap();
                v1 += v2;

                t.value = v1.to_string();
            }
        }
        else if op == TokenType::OpMinus {
            if t.v_type == VarType::Int {
                let mut v1 = t.value.parse::<i32>().unwrap();
                let v2 = t2.value.parse::<i32>().unwrap();
                v1 -= v2;
                
                t.value = v1.to_string();
            }
            else if t.v_type == VarType::Double {
                let mut v1 = t.value.parse::<f32>().unwrap();
                let v2 = t2.value.parse::<f32>().unwrap();
                v1 -= v2;

                t.value = v1.to_string();
            }
        }
        else if op == TokenType::OpLogOr {
            if t.v_type == VarType::Bool {
                let v1 = t.value.parse::<bool>().unwrap();
                let v2 = t2.value.parse::<bool>().unwrap();
                let res = v1 || v2;

                t.value = res.to_string();
                t.v_type = VarType::Bool;
            } else {
                eprintln!("O operador ' || ' só aceita operações com booleanos");
                exit(1);
            }
        }

        temp = lexer.peek_token().t_type;
    }

    t
}

pub fn term(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> VarStruct {
    if debug { println!("term"); }
    //term ::= power (multiplication-operator power)*

    let mut t = power(lexer, debug, simbolos);
    //fazer while
    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpMult || temp == TokenType::OpDiv || temp == TokenType::OpMod || temp == TokenType::OpLogAnd {
        let op = multiplication_operator(lexer, debug);
        let t2 = power(lexer, debug, simbolos);

        if t.v_type != t2.v_type { eprintln!("Tipos incompatíveis {:?} com {:?} no operador {:?}", t.v_type, t2.v_type, op); exit(1); }

        if t.v_type == VarType::String {  eprintln!("Tipos incompatíveis: {:?} não é compatível com '*', '/' ", t.v_type); exit(1);  }
        if t2.v_type == VarType::String {  eprintln!("Tipos incompatíveis: {:?} não é compatível com '*', '/' ", t2.v_type); exit(1);  }

        if op == TokenType::OpMult {
            if t.v_type == VarType::Int {
                let mut v1 = t.value.parse::<i32>().unwrap();
                let v2 = t2.value.parse::<i32>().unwrap();
                v1 *= v2;
                
                t.value = v1.to_string();
            }
            else if t.v_type == VarType::Double {
                let mut v1 = t.value.parse::<f32>().unwrap();
                let v2 = t2.value.parse::<f32>().unwrap();
                v1 *= v2;

                t.value = v1.to_string();
            }
            
        }
        else if op == TokenType::OpDiv {
            if t.v_type == VarType::Int {
                let mut v1 = t.value.parse::<i32>().unwrap();
                let v2 = t2.value.parse::<i32>().unwrap();
                v1 /= v2;
                
                t.value = v1.to_string();
            }
            else if t.v_type == VarType::Double {
                let mut v1 = t.value.parse::<f32>().unwrap();
                let v2 = t2.value.parse::<f32>().unwrap();
                v1 /= v2;

                t.value = v1.to_string();
            }
        }
        else if op == TokenType::OpMod {
            if t.v_type == VarType::Int {
                let mut v1 = t.value.parse::<i32>().unwrap();
                let v2 = t2.value.parse::<i32>().unwrap();
                v1 %= v2;
                
                t.value = v1.to_string();
            }
            else if t.v_type == VarType::Double {
                let mut v1 = t.value.parse::<f32>().unwrap();
                let v2 = t2.value.parse::<f32>().unwrap();
                v1 %= v2;

                t.value = v1.to_string();
            }
        }
        else if op == TokenType::OpLogAnd {
            if t.v_type == VarType::Bool {
                let v1 = t.value.parse::<bool>().unwrap();
                let v2 = t2.value.parse::<bool>().unwrap();
                let res = v1 && v2;

                t.value = res.to_string();
            } else {
                eprintln!("O operador ' && ' só aceita operações com booleanos");
                exit(1);
            }
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
    //power-operator ::= "**"
    let tok = lexer.next_token();
    tok.t_type
}
// -------

pub fn power(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> VarStruct {
    //power ::= factor (power-operator factor)*
    let mut t = factor(lexer, debug, simbolos);
    //fazer while
    let mut temp = lexer.peek_token().t_type;

    while temp == TokenType::OpPower {
        let op = power_operator(lexer, debug);
        let t2 = factor(lexer, debug, simbolos);

        if t.v_type != t2.v_type { eprintln!("Tipos incompatíveis {:?} com {:?}", t.v_type, t2.v_type); exit(1); }

        if t.v_type == VarType::String || t.v_type == VarType::Bool {  eprintln!("Tipos incompatíveis: {:?} não é compatível com **", t.v_type); exit(1);  }
        if t2.v_type == VarType::String || t2.v_type == VarType::Bool {  eprintln!("Tipos incompatíveis: ** não aceita {:?} como lado direito", t2.v_type); exit(1);  }

        if op == TokenType::OpPower {

            if t.v_type == VarType::Int {
                t.value = t.value.parse::<i32>().unwrap().pow(t2.value.parse::<i32>().unwrap() as u32).to_string();
            }
            
        }
       
        temp = lexer.peek_token().t_type;
    }

    t
}

pub fn factor(lexer: &mut Lexer, debug: bool, simbolos : &mut HashMap<String, VarStruct>) -> VarStruct {
    if debug { println!("factor"); }
    //factor ::= '!'* ( variable | number | string | '(' expression ')' )
    let tok = lexer.next_token();
    // println!("TOK {:?}; PEEK {:?}", tok.value, lexer.peek_token().value);

    if tok.t_type == TokenType::Identificador {
        let v_stru = simbolos.get_mut(&tok.value).unwrap();
        match v_stru.v_type {
            _ => { return VarStruct { value: v_stru.value.clone(), v_type: v_stru.v_type } }
            //_ => {return VarStruct{value: "Error".to_owned(), v_type: VarType::Void} }
        }
    }
    else if tok.t_type == TokenType::IntLiteral { 
        return VarStruct{ value: tok.value, v_type: VarType::Int };//tok.value.parse::<i32>().unwrap();
    }
    else if tok.t_type == TokenType::DoubleLiteral { 
        return VarStruct{ value: tok.value, v_type: VarType::Double };//tok.value.parse::<i32>().unwrap();
    }
    else if tok.t_type == TokenType::StringLiteral {
        // println!("AEHO não sei oq fazer");
        return VarStruct{ value: tok.value, v_type: VarType::String };//tok.value.parse::<i32>().unwrap();
    }
    else if tok.t_type == TokenType::True || tok.t_type == TokenType::False {
        // println!("AEHO não sei oq fazer");
        return VarStruct{ value: tok.value, v_type: VarType::Bool };//tok.value.parse::<i32>().unwrap();
    }

    
    if tok.t_type == TokenType::LPar {
        
        let t = expression(lexer, debug, simbolos);
        
        read_token_type(lexer, TokenType::RPar);
        return t;
    }


    eprintln!("Não foi possível achar nenhum valor {:?}", tok.t_type);

    VarStruct{value: "Error".to_owned(), v_type: VarType::Void}
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
