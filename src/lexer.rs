mod token

struct Lexer {
    cur_char: char,
    index: u32,
    code: String
}

impl Lexer {
    fn new(code: String) -> Lexer {
        Lexer {
            index : 0,
            cur_char: code.chars().nth(0).unwrap(),
            code: code
        }
    }

    fn next(mut self) {
        if self.index < self.code.len() &&  self.cur_char != '\0' {
            self.index += 1;
            self.cur_char = self.code.chars().nth(0).unwrap();
        }
    }
}

