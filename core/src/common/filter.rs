use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Operator {
    And,
    Or,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Operator, Box<Expr>),
    Group(Box<Expr>),
    Value(String),
}

pub struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_expression()
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        let mut expr = self.parse_term()?;

        while let Some(op) = self.peek_logical_operator() {
            self.consume_logical_operator();
            let right = self.parse_term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        Ok(expr)
    }

    pub fn parse_term(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        if self.peek_char() == Some('(') {
            self.consume_char();
            let expr = self.parse_expression()?;
            self.skip_whitespace(); 
            if self.peek_char() == Some(')') {
                self.consume_char();
                Ok(Expr::Group(Box::new(expr)))
            } else {
                println!("--{}", self.peek_char().unwrap());
                Err("Expected closing parenthesis".to_string())
            }
        } else {
            self.parse_comparison()
        }
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        let left = self.parse_value()?;

        if let Some(op) = self.peek_comparison_operator() {
            self.skip_whitespace();
            let right = self.parse_value()?;
            Ok(Expr::Binary(Box::new(left), op, Box::new(right)))
        } else {
            // Ok(left)
            Err("No value".to_string())
        }
    }

    fn parse_value(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        let mut value = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() || c == '&' || c == '|' || c == '(' || c == ')' || c == '=' || c == '!' || c == '>' || c == '<' {
                break;
            }
            value.push(c);
            self.consume_char();
        }
        if !value.is_empty() {
            Ok(Expr::Value(value))
        } else {
            Err("Expected value".to_string())
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn consume_char(&mut self) {
        self.chars.next();
    }

    fn peek_logical_operator(&mut self) -> Option<Operator> {
        self.skip_whitespace();
        let chars: String = self.chars.clone().take(2).collect();
        match chars.as_str() {
            "&&" => Some(Operator::And),
            "||" => Some(Operator::Or),
            _ => None,
        }
    }

    fn consume_logical_operator(&mut self) {
        self.chars.next();
        self.chars.next();
    }
    fn skip_whitespace(&mut self) {
        loop {
            if let None = self.chars.next_if_eq(&' ') {
                break;
            }
        }
    }

    fn next_operetor(&mut self) -> String {
        let mut str = "".to_string();
        loop {
            if let Some(ch) = self.chars.next_if(|c| "=!><".contains(*c)) {
                str.push(ch);
            } else {
                break;
            }
        }
        str
    }
    fn peek_comparison_operator(&mut self) -> Option<Operator> {
        self.skip_whitespace();
        let chars = self.next_operetor();
        match chars.as_str() {
            "==" => Some(Operator::Equal),
            "!=" => Some(Operator::NotEqual),
            ">=" => Some(Operator::GreaterThanOrEqual),
            "<=" => Some(Operator::LessThanOrEqual),
            ">" => Some(Operator::GreaterThan),
            "<" => Some(Operator::LessThan),
            _ => None,
        }
    }
}
// Evaluator to match expression with a dictionary of values
pub fn evaluate_expression(expr: &Expr, data: &HashMap<String, String>) -> bool {
    match expr {
        Expr::Binary(left, op, right) => {
            let left_val = evaluate_expression(left, data);
            let right_val = evaluate_expression(right, data);

            match op {
                Operator::And => left_val && right_val,
                Operator::Or => left_val || right_val,
                _ => {
                    let left_value = if let Expr::Value(ref key) = **left {
                        data.get(key).unwrap_or(&"".to_string()).to_string()
                    } else {
                        return false;
                    };

                    let right_value = if let Expr::Value(ref val) = **right {
                        val.clone()
                    } else {
                        return false;
                    };

                    match op {
                        Operator::Equal => left_value == right_value,
                        Operator::NotEqual => left_value != right_value,
                        Operator::GreaterThan => left_value > right_value,
                        Operator::LessThan => left_value < right_value,
                        Operator::GreaterThanOrEqual => left_value >= right_value,
                        Operator::LessThanOrEqual => left_value <= right_value,
                        _ => false,
                    }
                }
            }
        }
        Expr::Group(inner_expr) => evaluate_expression(inner_expr, data),
        Expr::Value(key) => data.contains_key(key),
    }
}