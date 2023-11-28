#[derive(Debug)]
pub enum Token {
    Const,
    Int,
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: String = match self {
            Token::Const => "const".to_string(),
            Token::Int => "int".to_string(),
            Token::While => "while".to_string(),
            Token::CurlyL => "{".to_string(),
            Token::CurlyR => "}".to_string(),
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

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
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
                    tokens.push(Token::Literal(value));
                }
                'a'..='z' | 'A'..='Z' => {
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
                        "const" => tokens.push(Token::Const),
                        "int" => tokens.push(Token::Int),
                        "print" => tokens.push(Token::Print),
                        "exit" => tokens.push(Token::Exit),
                        "while" => tokens.push(Token::While),
                        "proc" => tokens.push(Token::Procedure),
                        _ => tokens.push(Token::Identifier(identifier)),
                    }
                }
                '0'..='9' => {
                    let value: i32 = self.read_integer();
                    tokens.push(Token::Integer(value));
                }
                '-' => {
                    self.advance();
                    if let Some(ch) = self.peek() {
                        if ch.is_digit(10) {
                            let value = self.read_integer();
                            tokens.push(Token::Integer(-value));
                        } else {
                            tokens.push(Token::Minus);
                        }
                    }
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                '+' => {
                    self.advance();
                    if let Some(ch) = self.peek() {
                        if ch.is_digit(10) {
                            let value = self.read_integer();
                            tokens.push(Token::Integer(value));
                        } else {
                            tokens.push(Token::Plus);
                        }
                    }
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    self.advance();
                }
                '%' => {
                    tokens.push(Token::Percent);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                }
                '{' => {
                    tokens.push(Token::CurlyL);
                    self.advance();
                }
                '}' => {
                    tokens.push(Token::CurlyR);
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        tokens.push(Token::Equals);
                        self.advance();
                    } else {
                        tokens.push(Token::Assign);
                    }
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        tokens.push(Token::LessOrEqual);
                        self.advance();
                    } else {
                        tokens.push(Token::Less);
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }
        tokens.push(Token::EOF);
        tokens
    }
}
