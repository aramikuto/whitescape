#[derive(Debug)]
pub enum Token {
    Func,
    Return,
    Const,
    Int,
    String,
    While,
    CurlyL,
    CurlyR,
    Identifier(String),
    Assign,
    Integer(i32),
    Semicolon,
    Print,
    /// {}
    LParen,
    /// )
    RParen,
    /// [
    LSquare,
    /// ]
    RSquare,
    Comma,

    Plus,
    Minus,
    Star,
    /// /
    Slash,
    Percent,

    /// ==
    Equals,
    /// <
    Less,
    /// <=
    LessOrEqual,
    Literal(String),

    /// proc
    Procedure,

    Exit,
    EOF,
}

#[derive(Debug)]
pub struct SourceToken {
    pub token: Token,
    pub position: (usize, usize),
}

impl SourceToken {
    pub fn get(token: Token, position: (usize, usize)) -> Self {
        SourceToken {
            token,
            position: position,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: String = match self {
            Token::Func => "func".to_string(),
            Token::Return => "return".to_string(),
            Token::Const => "const".to_string(),
            Token::Int => "int".to_string(),
            Token::String => "string".to_string(),
            Token::While => "while".to_string(),
            Token::CurlyL => "{".to_string(),
            Token::CurlyR => "}".to_string(),
            Token::LSquare => "[".to_string(),
            Token::RSquare => "]".to_string(),
            Token::Identifier(s) => s.clone(),
            Token::Assign => "=".to_string(),
            Token::Integer(i) => i.to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Print => "print".to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::Comma => ".to_string(),".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Star => "".to_string(),
            Token::Slash => "/".to_string(),
            Token::Percent => "%".to_string(),
            Token::Equals => "==".to_string(),
            Token::Less => "<".to_string(),
            Token::LessOrEqual => "<=".to_string(),
            Token::Procedure => "proc".to_string(),
            Token::Literal(v) => v.clone(),
            Token::Exit => "exit".to_string(),
            Token::EOF => "EOF".to_string(),
        };
        write!(f, "{}", s)
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    fn read_integer(&mut self) -> i32 {
        let mut value = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        value.parse::<i32>().unwrap()
    }

    fn getFileLocation(&self) -> (usize, usize) {
        // TODO: This is a very inefficient way to do this!
        let mut line = 1;
        let mut column = 1;
        for (i, ch) in self.input.chars().enumerate() {
            if i == self.position {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        (line, column)
    }

    pub fn tokenize(&mut self) -> Vec<SourceToken> {
        let mut tokens: Vec<SourceToken> = vec![];

        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    let start_pos = self.getFileLocation();
                    self.advance();
                    let mut value = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == '"' {
                            break;
                        } else {
                            value.push(ch);
                            self.advance();
                        }
                    }
                    self.advance();
                    tokens.push(SourceToken::get(Token::Literal(value), start_pos));
                }
                '[' => {
                    tokens.push(SourceToken::get(Token::LSquare, self.getFileLocation()));
                    self.advance();
                }
                ']' => {
                    tokens.push(SourceToken::get(Token::RSquare, self.getFileLocation()));
                    self.advance();
                }
                'a'..='z' | 'A'..='Z' => {
                    let start_pos = self.getFileLocation();
                    let mut identifier = String::new();
                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_' {
                            identifier.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    match identifier.as_str() {
                        "func" => tokens.push(SourceToken::get(Token::Func, start_pos)),
                        "return" => tokens.push(SourceToken::get(Token::Return, start_pos)),
                        "const" => tokens.push(SourceToken::get(Token::Const, start_pos)),
                        "string" => tokens.push(SourceToken::get(Token::String, start_pos)),
                        "int" => tokens.push(SourceToken::get(Token::Int, start_pos)),
                        "print" => tokens.push(SourceToken::get(Token::Print, start_pos)),
                        "exit" => tokens.push(SourceToken::get(Token::Exit, start_pos)),
                        "while" => tokens.push(SourceToken::get(Token::While, start_pos)),
                        _ => {
                            tokens.push(SourceToken::get(Token::Identifier(identifier), start_pos))
                        }
                    }
                }
                '0'..='9' => {
                    let start_pos = self.getFileLocation();
                    let value: i32 = self.read_integer();
                    tokens.push(SourceToken::get(Token::Integer(value), start_pos));
                }
                '-' => {
                    let start_pos = self.getFileLocation();
                    self.advance();
                    if let Some(ch) = self.peek() {
                        if ch.is_digit(10) {
                            let value = self.read_integer();
                            tokens.push(SourceToken::get(Token::Integer(-value), start_pos));
                        } else {
                            tokens.push(SourceToken::get(Token::Minus, start_pos));
                        }
                    }
                }
                ',' => {
                    tokens.push(SourceToken::get(Token::Comma, self.getFileLocation()));
                    self.advance();
                }
                '+' => {
                    let start_pos = self.getFileLocation();
                    self.advance();
                    if let Some(ch) = self.peek() {
                        if ch.is_digit(10) {
                            let value = self.read_integer();
                            tokens.push(SourceToken::get(Token::Integer(value), start_pos));
                        } else {
                            tokens.push(SourceToken::get(Token::Plus, start_pos));
                        }
                    }
                }
                '*' => {
                    tokens.push(SourceToken::get(Token::Star, self.getFileLocation()));
                    self.advance();
                }
                '/' => {
                    tokens.push(SourceToken::get(Token::Slash, self.getFileLocation()));
                    self.advance();
                }
                '%' => {
                    tokens.push(SourceToken::get(Token::Percent, self.getFileLocation()));
                    self.advance();
                }
                '(' => {
                    tokens.push(SourceToken::get(Token::LParen, self.getFileLocation()));
                    self.advance();
                }
                ')' => {
                    tokens.push(SourceToken::get(Token::RParen, self.getFileLocation()));
                    self.advance();
                }
                '{' => {
                    tokens.push(SourceToken::get(Token::CurlyL, self.getFileLocation()));
                    self.advance();
                }
                '}' => {
                    tokens.push(SourceToken::get(Token::CurlyR, self.getFileLocation()));
                    self.advance();
                }
                ';' => {
                    tokens.push(SourceToken::get(Token::Semicolon, self.getFileLocation()));
                    self.advance();
                }
                '=' => {
                    let start_pos = self.getFileLocation();
                    self.advance();
                    if self.peek() == Some('=') {
                        tokens.push(SourceToken::get(Token::Equals, start_pos));
                        self.advance();
                    } else {
                        tokens.push(SourceToken::get(Token::Assign, start_pos));
                    }
                }
                '<' => {
                    let start_pos = self.getFileLocation();
                    self.advance();
                    if self.peek() == Some('=') {
                        tokens.push(SourceToken::get(Token::LessOrEqual, start_pos));
                        self.advance();
                    } else {
                        tokens.push(SourceToken::get(Token::Less, start_pos));
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        tokens.push(SourceToken::get(Token::EOF, self.getFileLocation()));
        tokens
    }
}
