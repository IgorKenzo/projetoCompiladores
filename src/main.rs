// mod operadores;
mod maqest;
mod token;
// use std::io;


fn main() {
    // let mut line = String::new();
    // io::stdin().read_line(&mut line).unwrap();
    // operadores::find("= + - * /");
    maqest::run("./operators.txt", "- ");
}
