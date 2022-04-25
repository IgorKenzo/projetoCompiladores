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

        
    

    //
    Literal,
    StringLiteral,
    LCol,
    RCol,
    LPar,
    RPar,
    SemiCol,
    Colon,

    //Palavras Reservadas
    ReservMain,
    ReservLet,
    ReservIf,
    ReservElse,
    ReservWhile,
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
            "void"  => { return TokenType::DataType(VarType::Void) },
            "char"  => { return TokenType::DataType(VarType::Char) },
            "int"  => { return TokenType::DataType(VarType::Int) },
            "double"  => { return TokenType::DataType(VarType::Double) },
            "string" => { return TokenType::DataType(VarType::String) },
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
}
#[derive(Debug, Clone, Copy)]
pub struct VarStruct {
    pub value : &'static str,
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