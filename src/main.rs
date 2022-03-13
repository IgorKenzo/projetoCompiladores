// mod operadores;
// mod maqest;
mod token;
mod lexer;
use lexer::Lexer;
use token::Token;
use token::TokenType;

use std::env;


use std::fs;

fn main() {
    // operadores::find("= + - * /");
    // maqest::run("./operators.txt", "- ");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let content = fs::read_to_string(filename)
        .expect("Arquivo não encontrado");
    
    let mut lexer = Lexer::new(String::from(content));
    let mut tok : Token;
    loop {
        tok = lexer.next_token();
        if matches!(tok.t_type,TokenType::EoF) {
            break;
        }

        println!("TOKEN: {}, TIPO: {:?}", tok.value, tok.t_type);
    }
}
