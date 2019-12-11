use std::option::Option::Some;

trait AST {
    fn eval(&self) -> f64;
}

struct Add(Box<dyn AST>, Box<dyn AST>);

struct Sub(Box<dyn AST>, Box<dyn AST>);

struct Mul(Box<dyn AST>, Box<dyn AST>);

struct Div(Box<dyn AST>, Box<dyn AST>);

struct Num(f64);

#[derive(Debug)]
enum Token {
    TNumber(f64),
    TOp(char),
    TQUOTE,
    TEOF,
}

impl AST for Add {
    fn eval(&self) -> f64 {
        self.0.eval() + self.1.eval()
    }
}

impl AST for Sub {
    fn eval(&self) -> f64 {
        self.0.eval() - self.1.eval()
    }
}

impl AST for Mul {
    fn eval(&self) -> f64 {
        self.0.eval() * self.1.eval()
    }
}

impl AST for Div {
    fn eval(&self) -> f64 {
        self.0.eval() / self.1.eval()
    }
}

impl AST for Num {
    fn eval(&self) -> f64 {
        self.0
    }
}

struct Parser {
    source: String,
    position: usize,
}

impl Parser {
    fn new(source: &str) -> Parser {
        return Parser { source: String::from(source), position: 0 };
    }

    fn build_ast(&mut self) -> Box<dyn AST> {

    }

    fn term() {

    }

    fn quoted_or_num(&mut self) {
        let c = self.next_token();

    }

    fn factor(&mut self) {
    }

    fn next_token(&mut self) -> Token {
        return if let Some(c) = self.next_char() {
            match c {
                '+' => Token::TOp(c),
                '-' => Token::TOp(c),
                '*' => Token::TOp(c),
                '/' => Token::TOp(c),
                '(' => Token::TQUOTE,

                x if x >= '0' && x <= '9' => {
                    let mut n = (x as u8 - '0' as u8) as f64;
                    loop {
                        if let Some(c) = self.peek_char() {
                            if c.is_numeric() {
                                n = n * 10.0 + (c as u8 - '0' as u8) as f64;
                                self.satisfied(c);
                                continue;
                            }
                        }
                        break;
                    }
                    Token::TNumber(n)
                }
                _ => unreachable!()
            }
        } else {
            Token::TEOF
        };
    }

    fn satisfied(&mut self, c: char) {
        if let Some(c0) = self.next_char() {
            if c == c0 {
                self.position += 1;
                return;
            }
        }

        panic!("unsatisfied char {}", c);
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek_char();
        self.position += 1;
        return c;
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.position >= self.source.len() {
            return None;
        }

        let mut c = self.source.as_bytes()[self.position] as char;

        while c.is_whitespace() {
            self.position += 1;
            c = self.source.as_bytes()[self.position] as char;
        }

        return Some(c);
    }
}

fn eval_ast(ast: Box<dyn AST>) -> f64 {
    return ast.eval();
}

pub fn eval(expr: &str) -> f64 {
    eval_ast(Parser::new(expr).build_ast())
}
