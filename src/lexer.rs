use crate::token::Token;
use crate::token::TokenType;

pub struct Lexer {
    pub cur_char: char,
    pub index: u32,
    pub code: String,
    pub code_len: u32
}

impl Lexer {
    pub fn new(code: String) -> Lexer {
        Lexer {
            index : 0,
            cur_char: code.chars().nth(0).unwrap(),
            code_len: code.chars().count().try_into().unwrap(),
            code: code,
        }
    }

    pub fn move_to_next(&mut self){
        println!("{}", self.index);
        if self.index < self.code_len &&  self.cur_char != '\0' {
            self.index += 1;
            self.cur_char = self.code.chars().nth(0).unwrap();
        }
    }

    pub fn parse_id(&mut self) -> Token {
        println!("PARSE ID");
        let mut val = String::new();

        while between(self.cur_char, 'a', 'z') {
            val.push(self.cur_char);
            self.move_to_next();
        }

        Token::new(val, TokenType::Identificador)
    }

    pub fn parse_number(&mut self) -> Token {
        println!("PARSE NUM");
        let mut val = String::new();

        while between(self.cur_char, '0', '9') {
            val.push(self.cur_char);
            self.move_to_next();
        }

        Token::new(val, TokenType::Literal)
    }

    pub fn next_token(&mut self) -> Token {
        while self.cur_char != '\0' {
            if between(self.cur_char, 'a', 'z') {
                let t = self.parse_id();
                return self.advance_with(t)
            }

            if between(self.cur_char, '0', '9') {
                let t = self.parse_number();
                return self.advance_with(t)
            }

            match self.cur_char {
                '='|'+'|'-'|'*'|'/' => return self.advance_with(Token::new(self.cur_char.to_string(), TokenType::Operador)),
                '>' => {
                    if self.peek(1) == '=' { 
                        return self.advance_with(Token::new(String::from(">="), TokenType::Operador))
                    } else { 
                        return self.advance_with(Token::new(String::from(">"), TokenType::Operador))
                    };
                },
                '<' => {
                    if self.peek(1) == '=' { 
                        return self.advance_with(Token::new(String::from("<="), TokenType::Operador)) 
                    } else { 
                        return self.advance_with(Token::new(String::from("<"), TokenType::Operador))
                    };
                },
                _ => return self.advance_with(Token::new(String::from("UNK"), TokenType::Operador))
            }
        }

        Token::new(String::from("EOF"), TokenType::EoF)
    }

   

    pub fn advance_with(&mut self, token: Token) -> Token{
        self.move_to_next();
        token
    }

    pub fn peek(&self, offset: u32) -> char {
        self.code.chars().nth(std::cmp::min(self.index + offset, self.code_len) as usize).unwrap()
    }
}

pub fn between(c:char, s:char, e: char) -> bool {
    if c >= s || c <= e {
        true
    } else{
        false
    }
}
//https://youtu.be/PRcMPwaWj1Y