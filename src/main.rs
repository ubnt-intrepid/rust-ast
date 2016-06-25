mod ast;
use ast::*;

fn main() {
    let tashizan = _plus(_int(1), _int(2));

    let def_function = _let("abs",
                            _fun("x", _if(_var("x"), _int(0), _negate(_var("x")), _var("x"))),
                            _app(_var("abs"), _int(-42)));

    print_eval("tashizan", tashizan);
    print_eval("def_function", def_function);
}
