#[derive(Debug)]
pub enum TokenType{
    Identificador,
    Operador,
    Literal,
    EoF
}

pub struct Token {
    pub value: String,
    pub t_type: TokenType
}

impl Token {
    pub fn new(v: String, t: TokenType) -> Token {
        Token{
            value: v,
            t_type: t
        }
    }
}