use super::canvas::{draw_axes, get_color, print_canvas, HEIGHT, WIDTH};
use super::context::create_math_context;
use super::types::PlotArgs;
use colored::Color;
use meval::{Context, Expr};
use regex::Regex;
use std::error::Error;

const DEFAULT_X_MIN: f64 = -10.0;
const DEFAULT_X_MAX: f64 = 10.0;
const DEFAULT_Y_MIN: f64 = -10.0;
const DEFAULT_Y_MAX: f64 = 10.0;

struct BoundingBox {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

pub fn run(args: PlotArgs) -> Result<(), Box<dyn Error>> {
    let context = create_math_context()?;
    let re_bare_sqrt = Regex::new(r"√([a-zA-Z_][a-zA-Z0-9_]*|-?[0-9]*\.?[0-9]+)")?;

    let equation_bounds = find_equation_bounds(&args.queries, &context)?;

    if equation_bounds.is_none() && args.x_min.is_none() && args.x_max.is_none() {
        eprintln!("Could not automatically determine the plot range for the given equation(s).\nThe function might be undefined in the default sampling range [-10, 10].\nPlease specify the range manually using --xmin and --xmax.");
        return Ok(())
    }

    let (x_min, x_max, y_min, y_max) = determine_ranges(
        args.x_min,
        args.x_max,
        args.y_min,
        args.y_max,
        equation_bounds,
    );


    let mut canvas: Vec<Vec<Option<Color>>> = vec![vec![None; WIDTH]; HEIGHT];

    for (i, query) in args.queries.iter().enumerate() {
        let color = get_color(i);
        if let Some(equation) = query.strip_prefix("y=") {
            let eq_pass1 = re_bare_sqrt.replace_all(equation, "sqrt($1)");
            let processed_equation = eq_pass1.replace('√', "sqrt");
            let expr: Expr = processed_equation.parse()?;
            plot_equation(
                &mut canvas,
                &expr,
                &context,
                color,
                x_min,
                x_max,
                y_min,
                y_max,
            )?;
        }
    }

    draw_axes(&mut canvas, x_min, x_max, y_min, y_max);
    print_canvas(&canvas, &args.queries);

    Ok(())
}

fn find_equation_bounds(
    queries: &[String],
    context_template: &Context,
) -> Result<Option<BoundingBox>, Box<dyn Error>> {
    let mut finite_points = Vec::new();
    let sample_points = (-100..=100).map(|i| i as f64 * 0.1); // -10 to 10, step 0.1

    for query in queries {
        if let Some(equation) = query.strip_prefix("y=") {
            let expr: Expr = equation.parse()?;
            for x in sample_points.clone() {
                let mut context = context_template.clone();
                context.var("x", x);
                if let Ok(y) = expr.eval_with_context(&mut context) {
                    if y.is_finite() {
                        finite_points.push((x, y));
                    }
                }
            }
        }
    }

    if finite_points.is_empty() {
        return Ok(None);
    }

    let mut x_min = finite_points[0].0;
    let mut x_max = finite_points[0].0;
    let mut y_min = finite_points[0].1;
    let mut y_max = finite_points[0].1;

    for (x, y) in &finite_points[1..] {
        x_min = x_min.min(*x);
        x_max = x_max.max(*x);
        y_min = y_min.min(*y);
        y_max = y_max.max(*y);
    }

    Ok(Some(BoundingBox { x_min, x_max, y_min, y_max }))
}

fn determine_ranges(
    arg_x_min: Option<f64>,
    arg_x_max: Option<f64>,
    arg_y_min: Option<f64>,
    arg_y_max: Option<f64>,
    bounds: Option<BoundingBox>,
) -> (f64, f64, f64, f64) {
    let mut x_min = arg_x_min;
    let mut x_max = arg_x_max;
    let mut y_min = arg_y_min;
    let mut y_max = arg_y_max;

    if let Some(b) = bounds {
        if x_min.is_none() {
            x_min = Some(b.x_min - (b.x_max - b.x_min).abs() * 0.1);
        }
        if x_max.is_none() {
            x_max = Some(b.x_max + (b.x_max - b.x_min).abs() * 0.1);
        }
        if y_min.is_none() {
            y_min = Some(b.y_min - (b.y_max - b.y_min).abs() * 0.1);
        }
        if y_max.is_none() {
            y_max = Some(b.y_max + (b.y_max - b.y_min).abs() * 0.1);
        }
    }

    let mut final_x_min = x_min.unwrap_or(DEFAULT_X_MIN);
    let mut final_x_max = x_max.unwrap_or(DEFAULT_X_MAX);
    let mut final_y_min = y_min.unwrap_or(DEFAULT_Y_MIN);
    let mut final_y_max = y_max.unwrap_or(DEFAULT_Y_MAX);

    let x_span = final_x_max - final_x_min;
    let y_span = final_y_max - final_y_min;

    let desired_y_span = x_span / 2.0; 

    if y_span < desired_y_span {
        let y_center = (final_y_max + final_y_min) / 2.0;
        final_y_min = y_center - desired_y_span / 2.0;
        final_y_max = y_center + desired_y_span / 2.0;
    } else {
        let x_center = (final_x_max + final_x_min) / 2.0;
        let desired_x_span = y_span * 2.0;
        final_x_min = x_center - desired_x_span / 2.0;
        final_x_max = x_center + desired_x_span / 2.0;
    }

    final_x_min = final_x_min.min(0.0);
    final_x_max = final_x_max.max(0.0);
    final_y_min = final_y_min.min(0.0);
    final_y_max = final_y_max.max(0.0);

    if final_x_min == final_x_max {
        final_x_min -= 1.0;
        final_x_max += 1.0;
    }
    if final_y_min == final_y_max {
        final_y_min -= 1.0;
        final_y_max += 1.0;
    }

    (final_x_min, final_x_max, final_y_min, final_y_max)
}

fn plot_equation(
    canvas: &mut [Vec<Option<Color>>],
    expr: &Expr,
    context_template: &Context,
    color: Color,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Result<(), Box<dyn Error>> {
    let mut last_point = None;

    for i in 0..WIDTH {
        let mut context = context_template.clone();
        let x = x_min + (i as f64 / (WIDTH - 1) as f64) * (x_max - x_min);
        context.var("x", x);
        let y = expr.eval_with_context(&mut context)?;

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
