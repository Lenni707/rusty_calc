use std::io;

#[derive(Debug)] // for printing test
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

fn lex_string(input: String) -> Vec<Token> { 
    let mut expr: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.peek() { // also basicly guckt peek() immer eine position nach vorne (returned eine Option) und while let guckt ob peek() Some oder None returned also wie if let
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        let token = match c {
            c if c.is_ascii_digit() => {
                let mut num_str = String::new();

                while let Some(c) = chars.peek() { // macht aus hintereiander steheden ziffern die gesamt nummer
                    if c.is_ascii_digit() || *c == '.' {
                        num_str.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let num = num_str.parse::<f64>().unwrap(); // cooles rust ding macht string => u64
                Token::Number(num)
            }

            '+' => { chars.next(); Token::Op(Operator::Add) }
            '-' => { chars.next(); Token::Op(Operator::Sub) }
            '*' => { chars.next(); Token::Op(Operator::Mul) }
            '/' => { chars.next(); Token::Op(Operator::Divi) }
            '(' => { chars.next(); Token::LParen }
            ')' => { chars.next(); Token::RParen }

            _ => panic!("failed to lex expression, unknown token: {}", c)
        };

        expr.push(token);
    }

    expr
}