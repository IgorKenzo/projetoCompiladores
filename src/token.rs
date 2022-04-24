#[derive(Debug, PartialEq)]
pub enum TokenType{
    Identificador,
    Operador,
    Literal,
    StringLiteral,
    LCol,
    RCol,
    LPar,
    RPar,
    SemiCol,

    //Palavras Reservadas
    ReservMain,
    ReservLet,
    ReservIf,
    ReservElse,
    ReservWhile,
    ReservVoid,
    ReservChar,
    ReservInt,
    ReservDouble,
    ReservString,

    Error,
    EoF
}
 
impl TokenType {
    pub fn str_to_tk(val : &str) -> TokenType {
        let val_slice: &str = &*val;
        match val_slice {
            "main" => { return TokenType::ReservMain; },
            "let" => { return TokenType::ReservLet },
            "if" => { return TokenType::ReservIf },
            "else" => { return TokenType::ReservElse },
            "while" => { return TokenType::ReservWhile },
            "void"  => { return TokenType::ReservVoid },
            "char"  => { return TokenType::ReservChar },
            "int"  => { return TokenType::ReservInt },
            "double"  => { return TokenType::ReservDouble },
            "string" => { return TokenType::ReservString },
            _ => { return TokenType::Error; }
        }
    }
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