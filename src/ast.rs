// http://qiita.com/esumii/items/0eeb30f35c2a9da4ab8a

#[derive(Clone)]
pub enum Expr {
    Int(i64),
    Sub(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Var(String),
    Fun(String, Box<Expr>),
}

pub use self::Expr::*;

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

pub fn eval(expr: Expr) -> Result<Expr, ()> {
    match expr {
        Int(i) => Ok(Int(i)),

        Var(_) => Err(()),

        Sub(e1, e2) => {
            let i = match try!(eval(*e1)) {
                Int(x) => x,
                _ => return Err(()),
            };

            let j = match try!(eval(*e2)) {
                Int(x) => x,
                _ => return Err(()),
            };

            Ok(Int(i - j))
        }

        If(e1, e2, e3, e4) => {
            let i = match try!(eval(*e1)) {
                Int(x) => x,
                _ => return Err(()),
            };

            let j = match try!(eval(*e2)) {
                Int(x) => x,
                _ => return Err(()),
            };

            let e = if i <= j {
                *e3
            } else {
                *e4
            };

            eval(e)
        }

        Fun(x, e) => Ok(Fun(x, e)),

        App(e1, e2) => {
            let (x, e) = match try!(eval(*e1)) {
                Fun(x, e) => (x, e),
                _ => return Err(()),
            };
            let v = try!(eval(*e2));
            let ee = try!(subst(*e, x, v));
            eval(ee)
        }
    }
}

fn subst(expr: Expr, x: String, v: Expr) -> Result<Expr, ()> {
    match expr {
        Int(i) => Ok(Int(i)),

        Var(y) => {
            if x == y {
                Ok(v)
            } else {
                Ok(Var(y))
            }
        }

        Sub(e1, e2) => {
            let i = try!(subst(*e1, x.clone(), v.clone()));
            let j = try!(subst(*e2, x.clone(), v.clone()));
            Ok(Sub(Box::new(i), Box::new(j)))
        }

        If(e1, e2, e3, e4) => {
            let i = try!(subst(*e1, x.clone(), v.clone()));
            let j = try!(subst(*e2, x.clone(), v.clone()));
            let k = try!(subst(*e3, x.clone(), v.clone()));
            let l = try!(subst(*e4, x.clone(), v.clone()));
            Ok(If(Box::new(i), Box::new(j), Box::new(k), Box::new(l)))
        }

        Fun(y, e) => {
            if x == y {
                Ok(Fun(y, e))
            } else {
                Ok(Fun(y, Box::new(try!(subst(*e, x, v)))))
            }
        }

        App(e1, e2) => {
            let i = try!(subst(*e1, x.clone(), v.clone()));
            let j = try!(subst(*e2, x.clone(), v.clone()));
            Ok(App(Box::new(i), Box::new(j)))
        }
    }
}

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
