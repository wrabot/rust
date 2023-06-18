use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::ops::Deref;
use std::rc::Rc;

// start parsing tools

// for printing
// #[derive(Debug)]

#[allow(unused)]
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[allow(unused)]
fn read_lines(size: usize) -> Vec<String> {
    (0..size).map(|_| read_line()).collect()
}

#[allow(unused)]
fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_matches('\n').to_string()
}

// end parsing tools

pub fn main() {
    let f = read_line();
    let vs = read_line()
        .split_whitespace()
        .map(|v| v.to_string())
        .collect_vec();
    let dict = read_line()
        .split_whitespace()
        .tuples()
        .map(|(k, v)| (k.to_string(), parse_input!(v, f32)))
        .collect::<HashMap<_, _>>();
    let (mut g, _) = parse(&f);
    eprintln!("f {:?}", g);
    for v in vs {
        g = g.d(&v);
        eprintln!("{:?} {:?}", v, g);
    }
    println!("{:?}", g.eval(&dict) as i32);
}

const TOKENS: [char; 4] = ['+', '*', '^', ')'];

fn parse(text: &str) -> (Rc<Expr>, &str) {
    match text.as_bytes()[0] {
        b' ' => parse(&text[1..]),
        b'(' => {
            let (left, remaining) = parse(&text[1..]);
            let op = remaining.as_bytes()[0];
            if op == b')' {
                (left, &remaining[1..])
            } else {
                let (right, remaining) = parse(&remaining[1..]);
                let expr = match op {
                    b'+' => add(left, right),
                    b'*' => mul(left, right),
                    b'^' => pow(left, right),
                    _ => panic!("invalid op {}", op),
                };
                (expr, &remaining[1..])
            }
        }
        _ => {
            let end = text.find(&TOKENS).unwrap();
            let operand = &text[..end];
            let expr: Expr = match operand.parse() {
                Ok(value) => Expr::Val(value),
                Err(_) => Expr::Var(operand.to_string()),
            };
            (Rc::new(expr), &text[end..])
        }
    }
}

#[derive(Debug)]
enum Expr {
    Val(f32),
    Var(String),
    Add(Rc<Expr>, Rc<Expr>),
    Mul(Rc<Expr>, Rc<Expr>),
    Pow(Rc<Expr>, Rc<Expr>),
}

impl Expr {
    fn d(&self, var: &str) -> Rc<Expr> {
        match self {
            Expr::Val(_) => val(0.0),
            Expr::Var(name) => {
                if name == var {
                    val(1.0)
                } else {
                    val(0.0)
                }
            }
            Expr::Add(left, right) => add(left.d(var), right.d(var)),
            Expr::Mul(left, right) => add(
                mul(left.d(var), right.to_owned()),
                mul(left.to_owned(), right.d(var)),
            ),
            Expr::Pow(left, right) => {
                match left.as_ref() {
                    // simplify pow
                    Expr::Var(name) => {
                        if name == var {
                            mul(
                                right.to_owned(),
                                pow(left.to_owned(), add(right.to_owned(), val(-1.0))),
                            )
                        } else {
                            val(0.0)
                        }
                    }
                    _ => val(0.0),
                }
            }
        }
    }

    fn eval(&self, vars: &HashMap<String, f32>) -> f32 {
        match self {
            Expr::Val(value) => *value,
            Expr::Var(name) => vars[name],
            Expr::Add(left, right) => left.eval(vars) + right.eval(vars),
            Expr::Mul(left, right) => left.eval(vars) * right.eval(vars),
            Expr::Pow(left, right) => left.eval(vars).powf(right.eval(vars)),
        }
    }
}

fn val(value: f32) -> Rc<Expr> {
    Rc::new(Expr::Val(value))
}

fn add(left: Rc<Expr>, right: Rc<Expr>) -> Rc<Expr> {
    // start optimization (not needed)
    if let Expr::Val(value) = left.deref() {
        if *value == 0.0 {
            return right;
        }
        if let Expr::Val(right_value) = right.deref() {
            return val(value + right_value);
        }
    }
    if let Expr::Val(_) = right.deref() {
        return add(right, left);
    }
    if let Expr::Add(left2, right2) = right.deref() {
        return add(add(left, left2.to_owned()), right2.to_owned());
    }
    // end optimization
    Rc::new(Expr::Add(left, right))
}

fn mul(left: Rc<Expr>, right: Rc<Expr>) -> Rc<Expr> {
    // start optimization (not needed)
    if let Expr::Val(value) = left.deref() {
        if *value == 0.0 {
            return left;
        }
        if *value == 1.0 {
            return right;
        }
        if let Expr::Val(right_value) = right.deref() {
            return val(value * right_value);
        }
    }
    if let Expr::Val(_) = right.deref() {
        return mul(right, left);
    }
    if let Expr::Mul(left2, right2) = right.deref() {
        return mul(mul(left, left2.to_owned()), right2.to_owned());
    }
    // end optimization
    Rc::new(Expr::Mul(left, right))
}

fn pow(left: Rc<Expr>, right: Rc<Expr>) -> Rc<Expr> {
    // start optimization (not needed)
    if let Expr::Val(value) = right.deref() {
        if *value == 0.0 {
            return val(1.0);
        }
        if *value == 1.0 {
            return left;
        }
        if let Expr::Val(left_value) = left.deref() {
            return val(value.powf(*left_value));
        }
    }
    // end optimization
    Rc::new(Expr::Pow(left, right))
}
