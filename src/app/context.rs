use meval::{Context, Error};

pub fn create_math_context() -> Result<Context<'static>, Error> {
    let mut ctx = Context::new();

    ctx.var("pi", std::f64::consts::PI);
    ctx.var("e", std::f64::consts::E);

    ctx.func("sqrt", |x| x.sqrt());
    ctx.func("exp", |x| x.exp());
    ctx.func2("pow", |b, e| b.powf(e));
    ctx.func2("root", |x, n| x.powf(1.0 / n));
    ctx.func("abs", |x| x.abs());

    ctx.func("sin", |x| x.sin());
    ctx.func("cos", |x| x.cos());
    ctx.func("tan", |x| x.tan());
    ctx.func("cot", |x| 1.0 / x.tan());
    ctx.func("sec", |x| 1.0 / x.cos());
    ctx.func("csc", |x| 1.0 / x.sin());

    ctx.func("asin", |x| x.asin());
    ctx.func("acos", |x| x.acos());
    ctx.func("atan", |x| x.atan());
    ctx.func("acot", |x| std::f64::consts::PI / 2.0 - x.atan());
    ctx.func("asec", |x| (1.0/x).acos());
    ctx.func("acsc", |x| (1.0/x).asin());
    ctx.func("arcsin", |x| x.asin());
    ctx.func("arccos", |x| x.acos());
    ctx.func("arctan", |x| x.atan());
    ctx.func("arccot", |x| std::f64::consts::PI / 2.0 - x.atan());
    ctx.func("arcsec", |x| (1.0/x).acos());
    ctx.func("arccsc", |x| (1.0/x).asin());

    ctx.func("sinh", |x| x.sinh());
    ctx.func("cosh", |x| x.cosh());
    ctx.func("tanh", |x| x.tanh());
    ctx.func("coth", |x| 1.0 / x.tanh());
    ctx.func("sech", |x| 1.0 / x.cosh());
    ctx.func("csch", |x| 1.0 / x.sinh());

    ctx.func("asinh", |x| x.asinh());
    ctx.func("acosh", |x| x.acosh());
    ctx.func("atanh", |x| x.atanh());
    ctx.func("acoth", |x| (1.0/x).atanh());
    ctx.func("asech", |x| (1.0/x).acosh());
    ctx.func("acsch", |x| (1.0/x).asinh());
    ctx.func("arsinh", |x| x.asinh());
    ctx.func("arcosh", |x| x.acosh());
    ctx.func("artanh", |x| x.atanh());
    ctx.func("arcoth", |x| (1.0/x).atanh());
    ctx.func("arsech", |x| (1.0/x).acosh());
    ctx.func("arcsch", |x| (1.0/x).asinh());

    ctx.func("ln", |x| x.ln());
    ctx.func("log", |x| x.log10());
    ctx.func2("logb", |x, b| x.log(b));

    ctx.func("floor", |x| x.floor());
    ctx.func("ceil", |x| x.ceil());
    ctx.func("round", |x| x.round());

    Ok(ctx)
}