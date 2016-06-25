// http://qiita.com/esumii/items/0eeb30f35c2a9da4ab8a

#[derive(Clone)]
enum Expr {
    Int(i64),
    Sub(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Var(String),
    Fun(String, Box<Expr>),
}
use Expr::*;

fn eval(expr: Expr) -> Option<Expr> {
    match expr {
        Int(_) => Some(expr),

        Sub(e1, e2) => {
            let i = match eval(*e1) {
                Some(Int(x)) => x,
                _ => return None,
            };

            let j = match eval(*e2) {
                Some(Int(x)) => x,
                _ => return None,
            };

            Some(Int(i - j))
        }

        If(e1, e2, e3, e4) => {
            let i = match eval(*e1) {
                Some(Int(x)) => x,
                _ => return None,
            };

            let j = match eval(*e2) {
                Some(Int(x)) => x,
                _ => return None,
            };

            eval(if i <= j {
                *e3
            } else {
                *e4
            })
        }

        Fun(_, _) => Some(expr),

        App(e1, e2) => {
            let (x, e) = match eval(*e1) {
                Some(Fun(x, e)) => (x, e),
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
        Int(_) => Some(expr),
        Var(y) => {
            if x == y {
                Some(v)
            } else {
                Some(Var(y))
            }
        }

        Sub(e1, e2) => {
            let i = match subst(*e1, x.clone(), v.clone()) {
                None => return None,
                Some(a) => a,
            };
            let j = match subst(*e2, x, v) {
                None => return None,
                Some(a) => a,
            };
            Some(Sub(Box::new(i), Box::new(j)))
        }

        If(e1, e2, e3, e4) => {
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

            Some(If(Box::new(i), Box::new(j), Box::new(k), Box::new(l)))
        }

        Fun(y, e) => {
            if x == y {
                Some(Fun(y, e))
            } else {
                let w = match subst(*e, x, v) {
                    None => return None,
                    Some(a) => a,
                };
                Some(Fun(y, Box::new(w)))
            }
        }

        App(e1, e2) => {
            let i = match subst(*e1, x.clone(), v.clone()) {
                None => return None,
                Some(a) => a,
            };
            let j = match subst(*e2, x.clone(), v.clone()) {
                None => return None,
                Some(a) => a,
            };
            Some(App(Box::new(i), Box::new(j)))
        }
    }
}

fn print_eval(name: &str, expr: Expr) {
    print!("eval {}: ", name);
    match eval(expr) {
        Some(Int(i)) => println!("{}", i),
        _ => panic!("evaluation failed.!"),
    }
}

fn _let(x: &str, e1: Expr, e2: Expr) -> Expr {
    App(Box::new(Fun(x.to_string(), Box::new(e2))), Box::new(e1))
}

fn main() {
    let tashizan = Sub(Box::new(Int(1)),
                       Box::new(Sub(Box::new(Int(0)), Box::new(Int(2)))));

    let def_function = {
        _let("abs",
             Fun("x".to_string(),
                 Box::new(If(Box::new(Var("x".to_string())),
                             Box::new(Int(0)),
                             Box::new(Sub(Box::new(Int(0)), Box::new(Var("x".to_string())))),
                             Box::new(Var("x".to_string()))))),
             App(Box::new(Var("abs".to_string())), Box::new(Int(-42))))
    };

    print_eval("tashizan", tashizan);
    print_eval("def_function", def_function);
}
