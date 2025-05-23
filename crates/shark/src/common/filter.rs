use std::cmp;
use std::collections::{HashMap, HashSet};
use std::str::Chars;
use std::iter::Peekable;
pub trait Superem{
    
}
pub trait FilterValue {
    
}

pub struct PacketProps {
    _map: HashMap<&'static str, HashSet<&'static str>>,
}

impl PacketProps{
    pub fn new() -> Self {
        Self{_map: HashMap::new()}
    }
    pub fn add(&mut self, key: &'static str, val: &'static str){
        if let Some(set) = self._map.get_mut(&key) {
            set.insert(val);
        } else {
            let mut _set = HashSet::new();
            _set.insert(val);
            self._map.insert(key, _set);
        }
    }
    pub fn get(&self, key: &'static str) -> Option<Vec<&'static str>>{
        if let Some(set) = self._map.get(&key) {
            let mut aa: Vec<&'static str> = set.into_iter().map(|f| *f).collect();
            aa.sort();
            return Some(aa);
        }
        None
    }
    pub fn merge(&mut self, other: &mut PacketProps) {
        for (key, values) in other._map.drain() {
            self._map
                .entry(key)
                .or_insert_with(HashSet::new)
                .extend(values);
        }
    }

    pub fn match_expr(&self, statement: &str) -> bool {
        let mut parser = Parser::new(statement);
        if let Ok(expr) = parser.parse() {
            return evaluate_expression(&expr, &self._map);
        } 
        false
    }
}

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
            Ok(left)
            // Err("No value".to_string())
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
pub fn evaluate_expression(expr: &Expr, data: &HashMap<&'static str, HashSet<&'static str>>) -> bool {
    match expr {
        Expr::Binary(left, op, right) => {
            let left_val = evaluate_expression(left, data);
            let right_val = evaluate_expression(right, data);

            match op {
                Operator::And => left_val && right_val,
                Operator::Or => left_val || right_val,
                _ => {
                    let left_value = if let Expr::Value(ref key) = **left {
                        data.get(&(key.as_ref()))
                    } else {
                        return false;
                    };

                    let right_value:&str = if let Expr::Value(ref val) = **right {
                        val
                    } else {
                        return false;
                    };
                    if let Some(_set) = left_value {
                        for v in _set.iter() {
                            let left = *v;
                            let max_len = cmp::max(left.len(), right_value.len());
                            let result = match op {
                                Operator::Equal => left == right_value,
                                Operator::NotEqual => left != right_value,
                                Operator::GreaterThan => {
                                    let _left = format!("{:0>max_len$}", left);
                                    let _right = format!("{:0>max_len$}", right_value);
                                    _left > _right
                                },
                                Operator::LessThan => {
                                    let _left = format!("{:0>max_len$}", left);
                                    let _right = format!("{:0>max_len$}", right_value);
                                    _left < _right
                                },
                                Operator::GreaterThanOrEqual => {
                                    let _left = format!("{:0>max_len$}", left);
                                    let _right = format!("{:0>max_len$}", right_value);
                                    _left >= _right

                                },
                                Operator::LessThanOrEqual => {
                                    let _left = format!("{:0>max_len$}", left);
                                    let _right = format!("{:0>max_len$}", right_value);
                                    _left <= _right
                                },
                                _ => false,
                            };
                            if result {
                                return true;
                            }
                        }
                        return false
                    } else {
                        return false;
                    }
                }
            }
        }
        Expr::Group(inner_expr) => evaluate_expression(inner_expr, data),
        Expr::Value(key) => {
            data.contains_key(&(*key).as_ref())
        },
    }
}