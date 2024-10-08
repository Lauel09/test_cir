use std::{collections::HashMap, fmt::Debug};


#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Lambda,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64)
}

#[derive(Clone)]
pub struct Token {
    pub t_type: TokenType,

    // lexeme is the String value of the Token as it is
    pub lexeme: Vec<u8>,
    
    // it may or may not be a literal
    pub literal: Option<Literal>,
    
    // line number
    pub line: usize, 

    // column number 
    pub col: usize 
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token {{ ty: {:?}, lexeme: \"{}\", literal: {:?}, line: {:?}, col: {:?}}}",
            self.t_type,
            String::from_utf8(self.lexeme.clone()).unwrap(),
            self.literal,
            self.line,
            self.col,
        )
    }
}


#[derive(Debug)]
pub struct Error {
    info: String,
    line: usize,
    col: usize,
    line_text: String,
}


#[derive(Debug)]
pub struct Scanner {
    
    // Source of tokens?
    pub source: Vec<u8>,

    // tokens in Token form
    tokens: Vec<Token>,


    // since Error string is only going to be used
    // for displaying the Error, we can borrow it with 
    // the same lifetime as the scanner borrows the line_text
    err: Option<Error>,

    // this is not the start position of the text
    // but rather the start position of the current
    // token we are looking at 
    start: usize,

    // current position
    current: usize, 

    // line number
    line: usize,

    // column number
    col:  usize,

    line_string: Vec<String>,

    keywords: HashMap<String, TokenType>,
}


impl Default for Scanner {
    fn default() -> Self {
        Self {
            source: Vec::with_capacity(100),
            tokens: Vec::with_capacity(100),
            err: None,
            start: 0,
            current: 0, 
            line: 1,
            col: 0,
            // Take the keywords and the TokenType
            // convert them into Rust HashMap
            keywords: vec![
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                 ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
                ("lambda", TokenType::Lambda)
            ]
            .into_iter()
            .map(|(x,y)| (x.to_string(), y))
            .collect(),
            line_string: Vec::with_capacity(100),
        }
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self, input: String) {
        self.source = input.as_bytes().to_vec();
        self.line_string = input
            .lines()
            .map(|x|x.to_string())
            .collect();
        
        while !self.done() {
            self.start = self.current;
            self.scan_token();
        }

        if let Some(err) = &self.err {
            eprintln!("[ERROR] - {} \n {} \n at {}:{}(line:col)", err.info,err.line_text, err.line, err.col);
        }
    }

    fn scan_token(&mut self) {
        
        use TokenType::*;
        
        let c = self.advance();

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let match_result = self.matches('=');

                if match_result {
                    self.add_token(BangEqual)
                }
                else {
                    self.add_token(Bang)
                }
            },
            '=' => {
                let match_result = self.matches('=');

                if match_result {
                    self.add_token(EqualEqual);
                }
                else {
                    self.add_token(Equal)
                }
            },
            '<' => {
                let match_result = self.matches('=');

                if match_result {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less);
                }
            },
            '>' => {
                let match_result = self.matches('=');

                if match_result {
                    self.add_token(GreaterEqual);
                }
                else {
                    self.add_token(Greater);
                }
            },
            
            // '/' is for comments
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                            // Till you encounter a new line
                            // and we haven't reached till the end
                            self.advance();
                    }

                } else {
                    // Then it's a character either in a string or division
                    self.add_token(Slash);
                }
            },

            ' ' | '\r' | '\t' => {},
            '\n' => {
                self.line += 1;
                self.col =0;
            },
            
            '"' => {
                // Handle the case of a String
                self.string();
            }
            _ => {
                if Scanner::is_decimal_digit(c) {
                    // it's a decimal number
                    self.number();
                } else if Scanner::is_alpha(c) {
                    // alphabetic -> might be a reserved keyword
                    // or a variable
                    self.identifier();
                }
                else {
                    self.set_error(format!("Scanner can't handle: {}",c));
                }
            },
        }
    }

    /// ths handlesk keywords and variables
    fn identifier(&mut self) {

        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let token_string = String::from_utf8(
            self.source[self.start .. self.current].to_vec()
        ).unwrap();

        let token_type = match self.keywords.get(&token_string) {
            Some(token_type)     =>  *token_type,
            None    =>  TokenType::Identifier,
        };

        match token_type {
            TokenType::Identifier => self.add_token_literal(
                TokenType::Identifier, Some(
                    Literal::Identifier(token_string)
                )
            ),

            _   => self.add_token(token_type),
        }
    }
    /// Checks if the character is alphanumeric
    fn is_alphanumeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_alpha(c)
    }
   
    /// Handle parsing of the number here
    fn number(&mut self) {
        // Whole idea is that our first character `c` has been found 
        // to be a decimal digit. So the numbers ahead can be either a float
        // or a long decimal number

        // Our start position is kept in self.start so don't worry about that
        while Scanner::is_decimal_digit(self.peek()) {
            self.advance(); // keep advancing
        }

        // next character is a point maybe we are looking at a float ?
        // So look at the character +2 ahead than the current
        // say if ex: 22.30 then you found the '.' to be next, if you look ahead
        // than that, it would be '3' at self.peek_next()
        if self.peek() == '.' && Scanner::is_decimal_digit(self.peek_next()) {
            self.advance();
        }

        // After our one decimal we are sure it's a float, and even if it's a 
        // decimal (based on our first while) then we are considering it float 
        // as well since Lox keeps numbers as float(always)

        while Scanner::is_decimal_digit(self.peek()) {
            self.advance();
        }

        if !Scanner::is_decimal_digit(self.peek()) {
            // if it is not a decimal digit
            if self.peek() != '\0' {
                self.set_error(format!("Invalid string at the end of the number: `{}`", self.peek()));
                return;
            }
        }

        let val = match String::from_utf8(
            self.source[self.start .. self.current].to_vec()
        ) {
            Ok(str)   => {
                match str.parse::<f64>() {
                    Ok(float) => float,
                    Err(float_e)   => {
                        self.set_error(float_e.to_string());
                        return;
                    } 
                }
            },
            Err(e) => {
                // utf8 conversion error
                // Error to return
                self.set_error(e.to_string());
                return;
            },
        };

        self.add_token_literal(TokenType::Number, Some(Literal::Number(val)))

    }

    /// Take error string, get current line, and set the error
    fn set_error(&mut self, error_string: String) {
          
        let current_line_text = match self.get_current_line() {
            Some(current_line) => current_line.to_string(),
            None    => "".to_string(),
        };

        let error = Error {
            info: error_string,
            line: self.line,
            col: self.col,
            line_text: current_line_text,
        };

        self.err = Some(error);

    }

    /// Scan for a string and store as a Token
    fn string(&mut self){

        // until you find the ending of the string and till the source code is ended
        // keep iterating
        // This code allows multiline strings
        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            // Get the text of the current line

            let line_text = match self.get_current_line() {
                Some(str) => str.to_string(),
                None    => "".to_string(),
            };
            self.err = Some(
                Error {
                    info: "Unterminated string found".to_string(),
                    line: self.line,
                    col: self.col,
                    line_text,
                }
            );
            return;
        }

        // can we remove this assert?
        // Ans: we sure can, but it is for the edge cases that we might ignore
        // the termination double quote
        assert!(self.peek() == '"');

        // Why this advance here exactly?
        // Ans: To also increment the '"' token
        self.advance();

        
        self.add_token_literal(
            
            // this TokenType is String
            TokenType::String,
            Some(Literal::Str(

                // Create the String from the raw u8 bytes
                String::from_utf8(
                    self.source[self.start + 1 .. self.current - 1].to_vec()
                ).unwrap()
            ))
        );
    }


    /// This function gets the current line in the form of a String
    fn get_current_line(&self) -> Option<&String> {
        self.line_string.get(self.line - 1)
    }

    /// Peek the next character without increasing the count or incrementing the tokenizer
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            char::from(self.source[self.current])
        }
    }

    /// If at the end return true, if next char doesn't match the `c`
    /// then return false, else increase the counts and return true
    fn matches(&mut self, c: char) -> bool {

        if self.is_at_end() {
            return false
        } else if self.peek_next() != c {
            return false
        }
        
        self.col += 1;
        self.current += 1;
        true
    }

    fn add_token(&mut self, token_type : TokenType) {
        self.add_token_literal(token_type, None)
    }

    /// To get next character without incrementing any counts
    /// or consuming any value
    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        }
        else {
            char::from(self.source[self.current + 1])
        }
    }

    fn add_token_literal(&mut self, token_type : TokenType, literal: Option<Literal>) {

        // text of Token in Vec<u8>
        let text = self.source[self.start .. self.current].to_vec();

        self.tokens.push(
            Token {
                t_type: token_type,
                lexeme: text,
                literal, 
                line: self.line,
                col:self.col
            }
        )

    }
    fn is_alpha(c: char) -> bool {
        c.is_alphabetic()
    }

    fn is_decimal_digit(c: char) -> bool {
        c.is_ascii_digit()
    }


    /// Get the current char
    pub fn cur_char(&self)  -> Option<char> {
        if self.done() {
            return None;
        }
        else {
            return Some(char::from(self.source[self.current - 1]));
        }
    }

    /// Advance to the next character and increment the counters
    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.col += 1;
        
        // The whole reason we did a +1 before and -1 later
        // was the analogy in our head that self.current is 0
        // in the program but in our head it is self.current = 1
        char::from(self.source[self.current - 1])

    }

    /// If we got an error or are at the end
    /// then we are done
    pub fn done(&self) -> bool {
        self.err.is_some() || self.is_at_end()
    }

    /// if current pointer is greater than len of the source of text
    /// then we are at the end
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}