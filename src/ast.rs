// http://qiita.com/esumii/items/0eeb30f35c2a9da4ab8a

macro_rules! get (
    ($e:expr) => (
        match $e {
            Some(x) => x,
            None => return None,
        }
    )
);

#[derive(Clone)]
pub enum Expr {
    Int(i64),
    Sub(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Var(String),
    Fun(String, Box<Expr>),
}
use self::Expr::*;

/// / constructors for Enums.

pub fn _int(i: i64) -> Expr {
    Int(i)
}

pub fn _var(v: &str) -> Expr {
    Var(v.to_string())
}

pub fn _sub(e1: Expr, e2: Expr) -> Expr {
    Sub(Box::new(e1), Box::new(e2))
}

pub fn _if(e1: Expr, e2: Expr, e3: Expr, e4: Expr) -> Expr {
    If(Box::new(e1), Box::new(e2), Box::new(e3), Box::new(e4))
}

pub fn _app(e1: Expr, e2: Expr) -> Expr {
    App(Box::new(e1), Box::new(e2))
}

pub fn _fun(v: &str, e: Expr) -> Expr {
    Fun(v.to_string(), Box::new(e))
}

/// Evaluate an expression.
pub fn eval(expr: Expr) -> Option<Expr> {
    match expr {
        Int(i) => Some(Int(i)),

        Var(_) => None,

        Sub(e1, e2) => {
            let i = match get!(eval(*e1)) {
                Int(x) => x,
                _ => return None,
            };

            let j = match get!(eval(*e2)) {
                Int(x) => x,
                _ => return None,
            };

            Some(Int(i - j))
        }

        If(e1, e2, e3, e4) => {
            let i = match get!(eval(*e1)) {
                Int(x) => x,
                _ => return None,
            };

            let j = match get!(eval(*e2)) {
                Int(x) => x,
                _ => return None,
            };

            let e = if i <= j {
                *e3
            } else {
                *e4
            };

            eval(e)
        }

        Fun(x, e) => Some(Fun(x, e)),

        App(e1, e2) => {
            let (x, e) = match get!(eval(*e1)) {
                Fun(x, e) => (x, e),
                _ => return None,
            };
            let v = get!(eval(*e2));
            let ee = get!(subst(*e, x, v));
            eval(ee)
        }
    }
}

/// substitutes the variable with name `x` to expression `v` (in `expr`).
fn subst(expr: Expr, x: String, v: Expr) -> Option<Expr> {
    match expr {
        Int(i) => Some(Int(i)),

        Var(y) => {
            if x == y {
                Some(v)
            } else {
                Some(Var(y))
            }
        }

        Sub(e1, e2) => {
            let i = get!(subst(*e1, x.clone(), v.clone()));
            let j = get!(subst(*e2, x.clone(), v.clone()));
            Some(Sub(Box::new(i), Box::new(j)))
        }

        If(e1, e2, e3, e4) => {
            let i = get!(subst(*e1, x.clone(), v.clone()));
            let j = get!(subst(*e2, x.clone(), v.clone()));
            let k = get!(subst(*e3, x.clone(), v.clone()));
            let l = get!(subst(*e4, x.clone(), v.clone()));
            Some(If(Box::new(i), Box::new(j), Box::new(k), Box::new(l)))
        }

        Fun(y, e) => {
            if x == y {
                Some(Fun(y, e))
            } else {
                Some(Fun(y, Box::new(get!(subst(*e, x, v)))))
            }
        }

        App(e1, e2) => {
            let i = get!(subst(*e1, x.clone(), v.clone()));
            let j = get!(subst(*e2, x.clone(), v.clone()));
            Some(App(Box::new(i), Box::new(j)))
        }
    }
}

/// / Utility functions.

pub fn _negate(e: Expr) -> Expr {
    _sub(_int(0), e)
}

pub fn _plus(e1: Expr, e2: Expr) -> Expr {
    _sub(e1, _negate(e2))
}

pub fn _let(x: &str, e1: Expr, e2: Expr) -> Expr {
    _app(_fun(x, e2), e1)
}

pub fn print_eval(name: &str, expr: Expr) {
    print!("eval {}: ", name);
    match eval(expr).unwrap() {
        Int(i) => println!("{}", i),
        _ => panic!("evaluation failed.!"),
    }
}
