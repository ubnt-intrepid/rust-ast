// http://qiita.com/nobsun/items/cbb780ffb68634873639

use std::collections::HashMap;

#[derive(Clone)]
pub enum Expr {
    Int(i64),
    Sub(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Var(String),
    Fun(String, Box<Expr>),
}

pub type Env = HashMap<String, Value>;

#[derive(Clone)]
pub enum Value {
    VInt(i64),
    VClosure(String, Expr, Env),
}

pub use self::Expr::*;
pub use self::Value::*;

pub fn eval(expr: Expr, env: Env) -> Result<Value, ()> {
    match expr {
        Int(i) => Ok(VInt(i)),

        Var(x) => env.get(&x).cloned().ok_or_else(|| ()),

        Sub(e1, e2) => {
            let i = match try!(eval(*e1, env.clone())) {
                VInt(x) => x,
                _ => return Err(()),
            };
            let j = match try!(eval(*e2, env.clone())) {
                VInt(x) => x,
                _ => return Err(()),
            };
            Ok(VInt(i - j))
        }

        If(e1, e2, e3, e4) => {
            let i = match try!(eval(*e1, env.clone())) {
                VInt(x) => x,
                _ => return Err(()),
            };
            let j = match try!(eval(*e2, env.clone())) {
                VInt(x) => x,
                _ => return Err(()),
            };
            if i <= j {
                eval(*e3, env.clone())
            } else {
                eval(*e4, env.clone())
            }
        }

        Fun(x, e) => Ok(VClosure(x, *e, env)),

        App(e1, e2) => {
            let (x, e, mut env_) = match try!(eval(*e1, env.clone())) {
                VClosure(x, e, env) => (x, e, env),
                _ => return Err(()),
            };
            let v = try!(eval(*e2, env.clone()));
            env_.insert(x, v);
            eval(e, env_)
        }
    }
}
