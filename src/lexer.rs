use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(i32),
    Identifier(String),
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator,
    SemiColon,
    Let,
    For,
    Eof
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType
}

impl Token {
    pub fn new(value: String, token_type: TokenType) -> Token {
        Token { 
            value: value,
            token_type: token_type
         }
    }
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub src: Vec<char>,
    pub ch: char,
    pub position: usize,
    pub identifier: String
}

impl Lexer {
    pub fn new(src: Vec<char>) -> Self {
        Self {
            tokens: Vec::new(),
            src: src,
            ch: ' ',
            position: 0,
            identifier: String::new()
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while self.position < self.src.len(){
            if let Some(token) = self.next_token(){//check if number/identifier before adding bcuz sometimes x=10 not x = 10
                if !self.identifier.is_empty() {
                    println!("Identifier is not empty, adding as a token");
                    if let Ok(i) = self.identifier.parse::<i32>(){
                        self.add_token(Token::new(self.identifier.clone(), 
                            TokenType::Number(i)));    
                    }
                    else {
                        self.add_token(Token::new(self.identifier.clone(), 
                            TokenType::Identifier(self.identifier.clone())));    
                    }
                    self.identifier.clear();
                }
                self.add_token(token);
            }
            std::thread::sleep(Duration::from_millis(10));    
        }
        let t = Token::new(String::from("EndOfFile"), TokenType::Eof);
        self.add_token(t);
        self.tokens.clone()
    } 

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn next_token(&mut self) -> Option<Token> {
        let c = self.src[self.position];
        self.position += 1;
        match c {
            '(' => {
                return Some(Token::new(c.to_string(), TokenType::OpenParen));
            }
            ')' => {
                return Some(Token::new(c.to_string(), TokenType::CloseParen));
            }
            '+' | '-' | '*' | '/' => {
                return Some(Token::new(c.to_string(), TokenType::BinaryOperator));
            }
            '=' => {
                return Some(Token::new(c.to_string(), TokenType::Equals));
            }
            ';' => {
                return Some(Token::new(c.to_string(), TokenType::SemiColon));
            }
            _ => {
                //println!("Unknown char is: {}", c);
    
                if self.handle_whitespace() {
                    //println!("White Space Token.");
                    if !self.identifier.is_empty(){
                        self.flush_identifier();
                    }
                    return None
                }
                if c.is_alphabetic() || c == '_' {
                    self.identifier.push(c);
                    //println!("Adding to identifier: {}", self.identifier);
                    return None;
                }
                if c.is_numeric() {
                    self.identifier.push(c);
                    //println!("Adding to identifier: {}", self.identifier);
                    return None;
                }
            }
        };
        None
    }

    fn flush_identifier(&mut self) {
        //println!("Flushing Identifier: {}", self.identifier);    
        
        if self.identifier == String::from("let") {
            self.add_token(Token::new(self.identifier.clone(),
            TokenType::Let));       
        }
        else if self.identifier == String::from("for") {
            self.add_token(Token::new(self.identifier.clone(),
            TokenType::For));       
        }
        else if let Ok(i) = self.identifier.parse::<i32>() {
            self.add_token(Token::new(self.identifier.clone(), 
                TokenType::Number(i)));    
        }
        else {
            self.add_token(Token::new(self.identifier.clone(), 
                TokenType::Identifier(self.identifier.clone())));    
        }
        self.identifier.clear();
    }
    
    fn handle_whitespace(&mut self) -> bool {
        let c = self.src[self.position - 1]; 
        if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            return true
        }
        return false 
    }

}