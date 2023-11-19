#[derive(Debug)]
pub enum Token {
    Push,
    Number(i64),
    OutputAsNumber,
    Exit,
    Invalid,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for word in input.split_whitespace() {
        let token = match word {
            "push" => Token::Push,
            "output_as_number" => Token::OutputAsNumber,
            "exit" => Token::Exit,
            // Check if the word starts with a '+' followed by digits
            _ if word.starts_with('+') && word[1..].chars().all(|c| c.is_numeric()) => {
                let number = word[1..].parse::<i64>().unwrap();
                Token::Number(number)
            }
            _ if word.starts_with('-') && word[1..].chars().all(|c| c.is_numeric()) => {
                let number = word[1..].parse::<i64>().unwrap() * -1;
                Token::Number(number)
            }
            _ => Token::Invalid,
        };

        tokens.push(token);
    }

    tokens
}
