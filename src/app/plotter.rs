use super::canvas::{draw_axes, get_color, print_canvas, HEIGHT, WIDTH};
use super::context::create_math_context;
use super::types::PlotArgs;
use colored::Color;
use meval::{Context, Expr};
use regex::Regex;
use std::error::Error;

pub fn run(args: PlotArgs) -> Result<(), Box<dyn Error>> {
    let mut canvas: Vec<Vec<Option<Color>>> = vec![vec![None; WIDTH]; HEIGHT];
    let mut context = create_math_context()?;

    // Regex to find √ followed by a variable/number and wrap it in sqrt()
    // e.g., √x -> sqrt(x) or √25 -> sqrt(25)
    let re_bare_sqrt = Regex::new(r"√([a-zA-Z_][a-zA-Z0-9_]*|[0-9]*\.?[0-9]+)")?;

    for (i, query) in args.queries.iter().enumerate() {
        if let Some(equation) = query.strip_prefix("y=") {
            // First, handle bare cases like √x
            let eq_pass1 = re_bare_sqrt.replace_all(equation, "sqrt($1)");
            // Then, handle any remaining √, which must have been followed by parentheses, e.g., √(x+1)
            let processed_equation = eq_pass1.replace('√', "sqrt");

            let expr: Expr = processed_equation.parse()?;
            let color = get_color(i);
            plot_equation(
                &mut canvas,
                &expr,
                &mut context,
                color,
                args.x_min,
                args.x_max,
                args.y_min,
                args.y_max,
            )?;
        }
    }

    draw_axes(&mut canvas, args.x_min, args.x_max, args.y_min, args.y_max);
    print_canvas(&canvas, &args.queries);

    Ok(())
}

fn plot_equation(
    canvas: &mut [Vec<Option<Color>>],
    expr: &Expr,
    context: &mut Context,
    color: Color,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Result<(), Box<dyn Error>> {
    let mut last_point = None;

    for i in 0..WIDTH {
        let x = x_min + (i as f64 / (WIDTH - 1) as f64) * (x_max - x_min);
        context.var("x", x);
        let y = expr.eval_with_context(&mut *context)?;

        if y.is_finite() {
            let j = ((y_max - y) / (y_max - y_min) * (HEIGHT - 1) as f64).round() as isize;
            let current_point = (i as isize, j);

            if let Some(last) = last_point {
                draw_line(canvas, last, current_point, color);
            }
            last_point = Some(current_point);
        } else {
            last_point = None;
        }
    }
    Ok(())
}

fn draw_line(canvas: &mut [Vec<Option<Color>>], p1: (isize, isize), p2: (isize, isize), color: Color) {
    let (mut x1, mut y1) = p1;
    let (x2, y2) = p2;

    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x1 >= 0 && x1 < WIDTH as isize && y1 >= 0 && y1 < HEIGHT as isize {
            canvas[y1 as usize][x1 as usize] = Some(color);
        }
        if x1 == x2 && y1 == y2 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }
        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
}
