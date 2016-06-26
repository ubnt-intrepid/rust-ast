mod ast_closure;
use ast_closure::*;

fn main() {
    let tashizan = _plus(_int(1), _int(2));

    let def_function = _let("abs",
                            _fun("x", _if(_var("x"), _int(0), _negate(_var("x")), _var("x"))),
                            _app(_var("abs"), _int(-42)));

    print_eval("tashizan", tashizan);
    print_eval("def_function", def_function);
}

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
    match eval(expr, Env::new()).unwrap() {
        VInt(i) => println!("{}", i),
        _ => panic!("evaluation failed.!"),
    }
}
