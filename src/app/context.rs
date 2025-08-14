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
    ctx.func("asin", |x| x.asin());
    ctx.func("acos", |x| x.acos());
    ctx.func("atan", |x| x.atan());

    ctx.func("sinh", |x| x.sinh());
    ctx.func("cosh", |x| x.cosh());
    ctx.func("tanh", |x| x.tanh());

    ctx.func("ln", |x| x.ln());
    ctx.func("log", |x| x.log10());
    ctx.func2("logb", |x, b| x.log(b));

    ctx.func("floor", |x| x.floor());
    ctx.func("ceil", |x| x.ceil());
    ctx.func("round", |x| x.round());

    Ok(ctx)
}