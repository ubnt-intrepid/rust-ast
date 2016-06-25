// http://qiita.com/esumii/items/0eeb30f35c2a9da4ab8a

mod ast {

    #[derive(Clone)]
    pub enum Expr {
        Int(i64),
        Sub(Box<Expr>, Box<Expr>),
        If(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
        App(Box<Expr>, Box<Expr>),
        Var(String),
        Fun(String, Box<Expr>),
    }

    pub fn eval(expr: Expr) -> Option<Expr> {
        match expr {
            Expr::Int(_) => Some(expr),

            Expr::Sub(e1, e2) => {
                let i = match eval(*e1) {
                    Some(Expr::Int(x)) => x,
                    _ => return None,
                };

                let j = match eval(*e2) {
                    Some(Expr::Int(x)) => x,
                    _ => return None,
                };

                Some(Expr::Int(i - j))
            }

            Expr::If(e1, e2, e3, e4) => {
                let i = match eval(*e1) {
                    Some(Expr::Int(x)) => x,
                    _ => return None,
                };

                let j = match eval(*e2) {
                    Some(Expr::Int(x)) => x,
                    _ => return None,
                };

                eval(if i <= j {
                    *e3
                } else {
                    *e4
                })
            }

            Expr::Fun(_, _) => Some(expr),

            Expr::App(e1, e2) => {
                let (x, e) = match eval(*e1) {
                    Some(Expr::Fun(x, e)) => (x, e),
                    _ => return None,
                };
                let v = match eval(*e2) {
                    Some(e) => e,
                    _ => return None,
                };
                let ee = match subst(*e, x, v) {
                    Some(e) => e,
                    _ => return None,
                };
                eval(ee)
            }

            _ => None,
        }
    }

    fn subst(expr: Expr, x: String, v: Expr) -> Option<Expr> {
        match expr {
            Expr::Int(_) => Some(expr),
            Expr::Var(y) => {
                if x == y {
                    Some(v)
                } else {
                    Some(Expr::Var(y))
                }
            }

            Expr::Sub(e1, e2) => {
                let i = match subst(*e1, x.clone(), v.clone()) {
                    None => return None,
                    Some(a) => a,
                };
                let j = match subst(*e2, x, v) {
                    None => return None,
                    Some(a) => a,
                };
                Some(Expr::Sub(Box::new(i), Box::new(j)))
            }

            Expr::If(e1, e2, e3, e4) => {
                let i = match subst(*e1, x.clone(), v.clone()) {
                    None => return None,
                    Some(a) => a,
                };
                let j = match subst(*e2, x.clone(), v.clone()) {
                    None => return None,
                    Some(a) => a,
                };
                let k = match subst(*e3, x.clone(), v.clone()) {
                    None => return None,
                    Some(a) => a,
                };
                let l = match subst(*e4, x.clone(), v.clone()) {
                    None => return None,
                    Some(a) => a,
                };

                Some(Expr::If(Box::new(i), Box::new(j), Box::new(k), Box::new(l)))
            }

            Expr::Fun(y, e) => {
                if x == y {
                    Some(Expr::Fun(y, e))
                } else {
                    let w = match subst(*e, x, v) {
                        None => return None,
                        Some(a) => a,
                    };
                    Some(Expr::Fun(y, Box::new(w)))
                }
            }

            Expr::App(e1, e2) => {
                let i = match subst(*e1, x.clone(), v.clone()) {
                    None => return None,
                    Some(a) => a,
                };
                let j = match subst(*e2, x.clone(), v.clone()) {
                    None => return None,
                    Some(a) => a,
                };
                Some(Expr::App(Box::new(i), Box::new(j)))
            }
        }
    }

    pub fn _int(i: i64) -> Expr {
        Expr::Int(i)
    }

    pub fn _var(v: &str) -> Expr {
        Expr::Var(v.to_string())
    }

    pub fn _sub(e1: Expr, e2: Expr) -> Expr {
        Expr::Sub(Box::new(e1), Box::new(e2))
    }

    pub fn _if(e1: Expr, e2: Expr, e3: Expr, e4: Expr) -> Expr {
        Expr::If(Box::new(e1), Box::new(e2), Box::new(e3), Box::new(e4))
    }

    pub fn _app(e1: Expr, e2: Expr) -> Expr {
        Expr::App(Box::new(e1), Box::new(e2))
    }

    pub fn _fun(v: &str, e: Expr) -> Expr {
        Expr::Fun(v.to_string(), Box::new(e))
    }

    pub fn _let(x: &str, e1: Expr, e2: Expr) -> Expr {
        _app(_fun(x, e2), e1)
    }

} // mod ast;

use ast::*;

fn print_eval(name: &str, expr: Expr) {
    print!("eval {}: ", name);
    match eval(expr) {
        Some(Expr::Int(i)) => println!("{}", i),
        _ => panic!("evaluation failed.!"),
    }
}

fn main() {
    let tashizan = _sub(_int(1), _sub(_int(0), _int(2)));

    let def_function = _let("abs",
                            _fun("x",
                                 _if(_var("x"), _int(0), _sub(_int(0), _var("x")), _var("x"))),
                            _app(_var("abs"), _int(-42)));

    print_eval("tashizan", tashizan);
    print_eval("def_function", def_function);
}
