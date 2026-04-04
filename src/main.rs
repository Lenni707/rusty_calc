use std::io;

#[derive(Debug)] // for printing test
enum Token {
    Number(f64),
    Op(Operator),
    LParen,
    RParen
}

#[derive(Debug, Clone)] 
enum Operator  {
    Add,
    Sub,
    Mul,
    Divi
}

#[derive(Debug)] 
enum Expr{
    Number(f64),
    MathOp {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    }
}

#[derive(Debug)] 
struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    fn peek(&self) -> Option<&Token> { // checks the value at a pos without advancing the pos
        self.tokens.get(self.pos)
    }
    fn consume(&mut self) -> Option<&Token> { // gets the value at this specifc pos and advances the pos 
        let t = self.tokens.get(self.pos);
        self.pos += 1;
        t
    }
    fn parse_exp(&mut self) -> Expr {
        let mut left = self.parse_term();

        loop {
            match self.peek() {
                Some(Token::Op(Operator::Add)) | Some(Token::Op(Operator::Sub)) => {
                    let op = if let Some(Token::Op(op)) = self.consume() {
                        op.clone()
                    } else {
                        unreachable!()
                    };
                    let right = self.parse_term();
                    left = Expr::MathOp {
                        op: op.clone(),
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break
            };
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_primary();

        loop {
            match self.peek() {
                Some(Token::Op(Operator::Mul)) | Some(Token::Op(Operator::Divi)) => {
                    let op = if let Some(Token::Op(op)) = self.consume() {
                        op.clone()
                    } else {
                        unreachable!()
                    };
                    let right = self.parse_primary();
                    left = Expr::MathOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.consume() {
            Some(Token::Number(n)) => Expr::Number(*n),
            Some(Token::LParen) => panic!("rekursion schäre diggi {:?}", self.peek()),
            _ => panic!("grammar rules not satisfied {:?}", self.peek())
        }
    }
}



fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_string();

    let tokens = lex_string(input);

    let mut parser = Parser::new(tokens);

    let ast = parser.parse_exp();

    println!("{:?}", ast);
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

                let num = num_str.parse::<f64>().expect("parsing failed with"); // cooles rust ding macht string => u64
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