enum TokenType{
    Identificador,
    Operador,
    Literal
}

struct Token {
    value: String,
    t_type: TokenType
}

impl Token {
    fn new(v: String, t: TokenType) -> Token {
        Token{
            value: v,
            t_type: t
        }
    }
}