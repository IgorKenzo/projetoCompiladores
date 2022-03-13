// mod operadores;
// use std::io;
// mod maqest;
mod token;
mod lexer;
use lexer::Lexer;
use token::Token;
use token::TokenType;
fn main() {
    // let mut line = String::new();
    // io::stdin().read_line(&mut line).unwrap();
    // operadores::find("= + - * /");
    // maqest::run("./operators.txt", "- ");

    let mut lexer = Lexer::new(String::from("a = 12 + 3"));
    let mut tok : Token;
    loop {
        tok = lexer.next_token();
        if matches!(tok.t_type,TokenType::EoF) {
            break;
        }

        println!("TOKEN: {}, TIPO: {:?}", tok.value, tok.t_type);
    }
}
