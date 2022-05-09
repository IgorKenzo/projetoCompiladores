#[derive(Debug, PartialEq)]
pub enum TokenType{
    Identificador,

    // Operadores
    OpSum,
    OpMinus,
    OpMult,
    OpPower,
    OpDiv,
    OpMod,
    
        //Assing
        OpAssign,
    
        //Rel
        OpRelLt,
        OpRelLe,
        OpRelGt,
        OpRelGe,
        OpRelEq,
        OpRelNe,

        //Logic
        OpLogAnd,
        OpLogOr,

        //Literal Types
        IntLiteral,
        DoubleLiteral,
        StringLiteral,
        True,
        False,

    LCol,
    RCol,
    LPar,
    RPar,
    SemiCol,
    Colon,
    Comma,

    //Palavras Reservadas
    ReservMain,
    ReservLet,
    ReservIf,
    ReservElse,
    ReservWhile,
    ReservPrint,
    // ReservVoid,
    // ReservChar,
    // ReservInt,
    // ReservDouble,
    // ReservString,
    DataType(VarType),

    Error,
    ErUNK,
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
            "print" => { return TokenType::ReservPrint },
            "void"  => { return TokenType::DataType(VarType::Void) },
            "char"  => { return TokenType::DataType(VarType::Char) },
            "int"  => { return TokenType::DataType(VarType::Int) },
            "double"  => { return TokenType::DataType(VarType::Double) },
            "string" => { return TokenType::DataType(VarType::String) },
            "bool" => { return TokenType::DataType(VarType::Bool) },
            "true" => { return TokenType::True },
            "false" => { return TokenType::False },
            _ => { return TokenType::Error; }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VarType {
    Void,
    Char,
    Int,
    Double,
    String,
    Bool
}
#[derive(Debug, Clone)]
pub struct VarStruct {
    pub value : String,
    pub v_type: VarType
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