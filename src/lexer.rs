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

    pub fn move_whitespace(&mut self) {
        while self.cur_char == '\n' || self.cur_char == '\r' || self.cur_char == ' ' || self.cur_char == '\t' {
            self.move_to_next()
        }
    }

    pub fn move_to_next(&mut self){
        if self.index < self.code_len && self.cur_char != '\0' {
            self.index += 1;
            if self.index != self.code_len {
                self.cur_char = self.code.chars().nth(self.index as usize).unwrap();
            } else {
                self.cur_char = '\0';
            }
        }
    }

    pub fn parse_id(&mut self) -> Token {
        // println!("PARSE ID");
        let mut val = String::new();

        while self.cur_char.is_alphanumeric() {
            val.push(self.cur_char);
            self.move_to_next();
        }

        // Fazer aqui o teste de palavre reservada?
        // .
        let reserv_words = ["main", "let", "if", "else", "while", "void" ,"char" ,"int" ,"double" ,"string"];

        for rw in reserv_words.iter() {
            if val == rw.to_string() {
                return Token::new(val, TokenType::str_to_tk(rw));
            }
        }
        

        Token::new(val, TokenType::Identificador)
    }

    pub fn parse_number(&mut self) -> Token {
        // println!("PARSE NUM");
        let mut val = String::new();

        while self.cur_char.is_numeric() {
            val.push(self.cur_char);
            self.move_to_next();
        }
        
        if self.cur_char == '.' {
            val.push(self.cur_char);
            self.move_to_next();
        }
        
        while self.cur_char.is_numeric() {
            val.push(self.cur_char);
            self.move_to_next();
        }

        if self.cur_char == 'e' {
            val.push(self.cur_char);
            self.move_to_next();
        }

        if self.cur_char == '-' {
            val.push(self.cur_char);
            self.move_to_next();
        }

        while self.cur_char.is_numeric() {
            val.push(self.cur_char);
            self.move_to_next();
        }


        Token::new(val, TokenType::Literal)
    }

    pub fn next_token(&mut self) -> Token {
        while self.cur_char != '\0' {

            //ANDA espaço em branco
            self.move_whitespace();

            if self.cur_char.is_numeric() {
                let t = self.parse_number();
                return t;//self.advance_with(t)
            }

            if self.cur_char.is_alphabetic()  {
                let t = self.parse_id();
                return t;//self.advance_with(t)
            }

            match self.cur_char {
                '='|'+'|'-'|'/' => return self.advance_with(Token::new(self.cur_char.to_string(), TokenType::Operador)),
                '*' => {
                    if self.peek(1) == '*' { 
                        self.move_to_next();
                        return self.advance_with(Token::new(String::from("**"), TokenType::Operador))
                    } else { 
                        return self.advance_with(Token::new(String::from("*"), TokenType::Operador))
                    };
                },
                '>' => {
                    if self.peek(1) == '=' { 
                        self.move_to_next();
                        return self.advance_with(Token::new(String::from(">="), TokenType::Operador))
                    } else { 
                        return self.advance_with(Token::new(String::from(">"), TokenType::Operador))
                    };
                },
                '<' => {
                    if self.peek(1) == '=' { 
                        self.move_to_next();
                        return self.advance_with(Token::new(String::from("<="), TokenType::Operador)) 
                    } else { 
                        return self.advance_with(Token::new(String::from("<"), TokenType::Operador))
                    };
                },
                '('|')' => return self.advance_with(Token::new(self.cur_char.to_string(), TokenType::Agrupador)),
                '{' => return self.advance_with(Token::new(self.cur_char.to_string(), TokenType::LCol)),
                '}' => return self.advance_with(Token::new(self.cur_char.to_string(), TokenType::RCol)),
                ';' => return self.advance_with(Token::new(self.cur_char.to_string(), TokenType::SemiCol)),
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