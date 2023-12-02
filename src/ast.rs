use crate::lexer::Token;

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    CompareEquals,
    CompareLessThan,
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Operation::Add => "+".to_string(),
            Operation::Sub => "-".to_string(),
            Operation::Mul => "*".to_string(),
            Operation::Div => "/".to_string(),
            Operation::Mod => "%".to_string(),
            Operation::CompareEquals => "==".to_string(),
            Operation::CompareLessThan => "<".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Int,
    String,
}

#[derive(Debug)]
pub enum Expression {
    Variable(String),
    Integer(i32),
    Declaration {
        identifier: String,
        dataType: DataType,
        value: Box<Expression>,
    },
    Literal(String),
    BinaryOp {
        operator: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Literal(value) => format!("{}", value),
            Expression::Variable(id) => format!("{}", id),
            Expression::Integer(value) => format!("{}", value.to_string()),
            Expression::Declaration {
                identifier,
                dataType,
                value,
            } => format!(
                "let {} = {}",
                identifier,
                match dataType {
                    DataType::Int => value.to_string(),
                    DataType::String => value.to_string(),
                }
            ),
            Expression::BinaryOp {
                operator,
                left,
                right,
            } => format!(
                "{} {} {}",
                left.to_string(),
                operator.to_string(),
                right.to_string()
            ),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    IntDeclaration(String),
    StringDeclaration(String, usize),
    Assignment(String, Expression),
    Exit,
    WhileLoop {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
    Call(String, Vec<Expression>),
    Block(Vec<Statement>),
}

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Statement>, String> {
    let mut ast = vec![];
    let mut tokens = tokens.iter().peekable();

    while let Some(&token) = tokens.peek() {
        match token {
            Token::Int => {
                tokens.next();
                let identifier = match tokens.next() {
                    Some(&Token::Identifier(ref id)) => id.clone(),
                    _ => return Err("Expected identifier".to_string()),
                };
                ast.push(Statement::IntDeclaration(identifier.clone()));
                if let Some(&Token::Assign) = tokens.peek() {
                    tokens.next();
                    let expr = parse_expression(&mut tokens)?;
                    ast.push(Statement::Assignment(identifier, expr));
                }
                if let Some(Token::Semicolon) = tokens.peek() {
                    tokens.next();
                } else {
                    return Err("Expected ;".to_string());
                }
            }
            Token::String => {
                tokens.next();
                if let Some(&Token::LSquare) = tokens.peek() {
                    tokens.next();
                } else {
                    return Err(format!("Expected [, found {:?}", tokens.peek()));
                }
                let length = match tokens.next() {
                    Some(&Token::Integer(value)) => {
                        if value <= 0 {
                            return Err("String size must be greater than 0".to_string());
                        }
                        let value = value as usize;
                        if let Some(&Token::RSquare) = tokens.peek() {
                            tokens.next();
                            value
                        } else {
                            return Err("Expected ]".to_string());
                        }
                    }
                    _ => return Err("Expected integer value".to_string()),
                };
                let identifier = match tokens.next() {
                    Some(&Token::Identifier(ref id)) => id.clone(),
                    _ => return Err("Expected identifier".to_string()),
                };
                ast.push(Statement::StringDeclaration(identifier.clone(), length));
                if let Some(&Token::Assign) = tokens.peek() {
                    tokens.next();
                    let expr = parse_expression(&mut tokens)?;
                    ast.push(Statement::Assignment(identifier, expr));
                }
                if let Some(Token::Semicolon) = tokens.peek() {
                    tokens.next();
                } else {
                    return Err("Expected ;".to_string());
                }
            }
            // TODO: Delete?
            Token::Const => {
                tokens.next();
                let identifier = match tokens.next() {
                    Some(&Token::Identifier(ref id)) => id.clone(),
                    _ => return Err("Expected identifier".to_string()),
                };
                if let Some(&Token::Integer(value)) = tokens.next() {
                    ast.push(Statement::IntDeclaration(identifier.clone()));
                    ast.push(Statement::Assignment(
                        identifier,
                        Expression::Integer(value),
                    ));
                } else {
                    return Err("Expected integer value".to_string());
                }

                if let Some(Token::Semicolon) = tokens.peek() {
                    tokens.next();
                } else {
                    return Err("Expected ;".to_string());
                }
            }
            Token::Print => {
                tokens.next();
                let expr = parse_expression(&mut tokens)?;
                ast.push(Statement::Call("print".to_string(), vec![expr]));

                if let Some(Token::Semicolon) = tokens.peek() {
                    tokens.next();
                } else {
                    return Err("Expected ;".to_string());
                }
            }
            Token::Exit => {
                tokens.next();
                ast.push(Statement::Exit);

                if let Some(Token::Semicolon) = tokens.peek() {
                    tokens.next();
                } else {
                    return Err("Expected ;".to_string());
                }
            }
            Token::While => {
                tokens.next();
                if let Some(&Token::LParen) = tokens.peek() {
                    tokens.next();
                    let condition: Expression = parse_expression(&mut tokens)?;

                    if let Some(&Token::RParen) = tokens.peek() {
                        tokens.next();
                        ast.push(Statement::WhileLoop {
                            condition: Box::new(condition),
                            body: Box::new(parse_statement(&mut tokens)?),
                        });
                    } else {
                        return Err("Expected )".to_string());
                    }
                } else {
                    return Err("Expected (".to_string());
                }
            }
            Token::Identifier(name) => {
                tokens.next();
                if let Some(&Token::Assign) = tokens.peek() {
                    tokens.next();
                    let expr = parse_expression(&mut tokens)?;
                    ast.push(Statement::Assignment(name.clone(), expr));
                } else if let Some(&Token::LParen) = tokens.peek() {
                    let mut args = vec![];
                    tokens.next();
                    while let Some(&token) = tokens.peek() {
                        match token {
                            Token::RParen => {
                                tokens.next();
                                break;
                            }
                            Token::Comma => {
                                tokens.next();
                            }
                            _ => args.push(parse_expression(&mut tokens)?),
                        }
                    }
                    ast.push(Statement::Call(name.clone(), args));
                } else {
                    return Err(format!("Unexpected identifier: {}", name));
                }
                if let Some(Token::Semicolon) = tokens.peek() {
                    tokens.next();
                } else {
                    return Err("Expected ;".to_string());
                }
            }
            Token::EOF => break,
            _ => return Err(format!("Unexpected token: {:?}", token)),
        }
    }

    Ok(ast)
}

fn parse_statement(
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Result<Statement, String> {
    let token = tokens.next();
    let statement = match token {
        Some(&Token::Const) => {
            let identifier = match tokens.next() {
                Some(&Token::Identifier(ref id)) => id.clone(),
                _ => return Err("Expected identifier".to_string()),
            };
            if let Some(&Token::Integer(value)) = tokens.next() {
                Statement::IntDeclaration(identifier)
            } else {
                return Err("Expected integer value".to_string());
            }
        }
        Some(&Token::Print) => {
            let expr = parse_expression(tokens)?;
            Statement::Call("print".to_string(), vec![expr])
        }
        Some(&Token::Exit) => Statement::Exit,
        Some(&Token::While) => {
            tokens.next();
            if let Some(&Token::LParen) = tokens.peek() {
                tokens.next();
                let condition: Expression = parse_expression(tokens)?;
                let mut body: Vec<Statement> = vec![];
                while let Some(&token) = tokens.peek() {
                    match token {
                        Token::RParen => {
                            tokens.next();
                            break;
                        }
                        _ => body.push(parse_statement(tokens)?),
                    }
                }
                Statement::WhileLoop {
                    condition: Box::new(condition),
                    body: Box::new(Statement::Block(body)),
                }
            } else {
                return Err("Expected (".to_string());
            }
        }
        Some(&Token::Identifier(ref id)) => {
            if let Some(&Token::Assign) = tokens.peek() {
                tokens.next();
                let expr = parse_expression(tokens)?;
                Statement::Assignment(id.clone(), expr)
            } else {
                return Err("Expected =".to_string());
            }
        }
        Some(&Token::CurlyL) => {
            let mut body: Vec<Statement> = vec![];
            while let Some(&token) = tokens.peek() {
                match token {
                    Token::CurlyR => {
                        tokens.next();
                        break;
                    }
                    Token::Semicolon => {
                        tokens.next();
                    }
                    _ => body.push(parse_statement(tokens)?),
                }
            }
            Statement::Block(body)
        }
        _ => return Err(format!("Statement, unexpected token: {:?}", token)),
    };

    Ok(statement)
}

fn parse_expression(
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Result<Expression, String> {
    let mut expr: Expression = parse_factor(tokens)?;
    while let Some(&token) = tokens.peek() {
        match token {
            Token::Plus => {
                tokens.next();
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::Add,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Minus => {
                tokens.next();
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::Sub,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Star => {
                tokens.next();
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::Mul,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Slash => {
                tokens.next();
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::Div,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Percent => {
                tokens.next();
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::Mod,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Equals => {
                tokens.next();
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::CompareEquals,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Less => {
                tokens.next();
                let right = parse_factor(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::CompareLessThan,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_factor(
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Result<Expression, String> {
    let mut expr: Expression = parse_unary(tokens)?;
    while let Some(&token) = tokens.peek() {
        match token {
            Token::Star => {
                tokens.next();
                let right = parse_unary(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::Mul,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Slash => {
                tokens.next();
                let right = parse_unary(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::Div,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            Token::Percent => {
                tokens.next();
                let right = parse_unary(tokens)?;
                expr = Expression::BinaryOp {
                    operator: Operation::Mod,
                    left: Box::new(expr),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_unary(
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Result<Expression, String> {
    let mut expr: Expression = parse_primary(tokens)?;
    while let Some(&token) = tokens.peek() {
        match token {
            // Token::Minus => {
            //     tokens.next();
            //     expr = Expression::BinaryOp {
            //         operator: Operation::Sub,
            //         left: Box::new(Expression::Integer(0)),
            //         right: Box::new(expr),
            //     };
            // }
            _ => break,
        }
    }
    Ok(expr)
}

fn parse_primary(
    tokens: &mut std::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Result<Expression, String> {
    let token = tokens.next();
    let expr = match token {
        Some(&Token::Literal(ref value)) => Expression::Literal(value.clone()),
        Some(&Token::Integer(value)) => Expression::Integer(value),
        Some(&Token::Identifier(ref id)) => Expression::Variable(id.clone()),
        Some(&Token::LParen) => {
            let expr = parse_expression(tokens)?;
            if let Some(&Token::RParen) = tokens.peek() {
                tokens.next();
                expr
            } else {
                return Err("Expected )".to_string());
            }
        }
        _ => return Err(format!("Primary, unexpected token: {:?}", token)),
    };

    Ok(expr)
}
