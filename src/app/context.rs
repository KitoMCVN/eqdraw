use meval::{Context, Error};

pub fn create_math_context() -> Result<Context<'static>, Error> {
    let mut ctx = Context::new();
    // Constants
    ctx.var("pi", std::f64::consts::PI);
    ctx.var("e", std::f64::consts::E);

    // Basic Functions
    ctx.func("sqrt", |x| x.sqrt());
    ctx.func2("pow", |b, e| b.powf(e));
    ctx.func2("root", |x, n| x.powf(1.0 / n));
    ctx.func("abs", |x| x.abs());

    // Trigonometric Functions
    ctx.func("sin", |x| x.sin());
    ctx.func("cos", |x| x.cos());
    ctx.func("tan", |x| x.tan());
    ctx.func("cot", |x| 1.0 / x.tan());
    ctx.func("asin", |x| x.asin());
    ctx.func("acos", |x| x.acos());
    ctx.func("atan", |x| x.atan());

    // Hyperbolic Functions
    ctx.func("sinh", |x| x.sinh());
    ctx.func("cosh", |x| x.cosh());
    ctx.func("tanh", |x| x.tanh());

    // Logarithmic Functions
    ctx.func("ln", |x| x.ln());
    ctx.func2("log", |x, r| x.log(r));

    // Rounding Functions
    ctx.func("floor", |x| x.floor());
    ctx.func("ceil", |x| x.ceil());
    ctx.func("round", |x| x.round());

    Ok(ctx)
}
