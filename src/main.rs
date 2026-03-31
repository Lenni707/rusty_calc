use std::io;

#[derive(Debug)]
enum Token {
    Number(f64),
    Op(Operator),
    LParen,
    RParen
}

#[derive(Debug)]
enum Operator  {
    Add,
    Sub,
    Mul,
    Divi
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_string(); // ← FIX

    let expr = lex_string(input);

    println!("{:?}", expr);
}

fn lex_string(str: String) -> Vec<Token> { 
    let mut expr: Vec<Token> = Vec::new();
    for i in str.chars() {
        if i == ' ' { continue; }
        let token: Token = match i {
            c if c.is_ascii_digit() => {
                    let num = c.to_digit(10).unwrap() as f64;
                    Token::Number(num)
                    
                },
            '+' => { Token::Op(Operator::Add) },
            '-' => { Token::Op(Operator::Sub) },
            '*' => { Token::Op(Operator::Mul) },
            '/' => { Token::Op(Operator::Divi) },
            '(' => { Token::LParen },
            ')' => { Token::RParen },
            _ => panic!("failed to lex expression, unknown token")
        };
        expr.push(token)
    }
    expr
}