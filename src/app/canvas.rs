use colored::{Color, Colorize};

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 80;

const COLORS: [Color; 6] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
];
pub const AXIS_COLOR: Color = Color::White;

pub fn get_color(index: usize) -> Color {
    COLORS[index % COLORS.len()]
}

pub fn draw_axes(
    canvas: &mut [Vec<Option<Color>>],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) {
    if x_min <= 0.0 && x_max >= 0.0 {
        let x_zero = ((0.0 - x_min) / (x_max - x_min) * (WIDTH - 1) as f64).round() as usize;
        if x_zero < WIDTH {
            for j in 0..HEIGHT {
                if canvas[j][x_zero].is_none() {
                    canvas[j][x_zero] = Some(AXIS_COLOR);
                }
            }
        }
    }

    if y_min <= 0.0 && y_max >= 0.0 {
        let y_zero = ((y_max - 0.0) / (y_max - y_min) * (HEIGHT - 1) as f64).round() as usize;
        if y_zero < HEIGHT {
            for i in 0..WIDTH {
                if canvas[y_zero][i].is_none() {
                    canvas[y_zero][i] = Some(AXIS_COLOR);
                }
            }
        }
    }
}

pub fn print_canvas(canvas: &[Vec<Option<Color>>], queries: &[String]) {
    let braille_dots = [
        (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (3, 0), (3, 1),
    ];

    for y in (0..HEIGHT).step_by(4) {
        for x in (0..WIDTH).step_by(2) {
            let mut char_code = 0x2800;
            let mut point_colors = [None; 8];

            for (i, &(dy, dx)) in braille_dots.iter().enumerate() {
                let ny = y + dy;
                let nx = x + dx;
                if nx < WIDTH && ny < HEIGHT {
                    if let Some(c) = canvas[ny][nx] {
                        char_code |= 1 << i;
                        point_colors[i] = Some(c);
                    }
                }
            }

            let color = point_colors.iter().filter_map(|c| *c).find(|&c| c != AXIS_COLOR).or_else(|| point_colors.iter().filter_map(|c| *c).next());

            let character = std::char::from_u32(char_code).unwrap_or(' ');
            if let Some(c) = color {
                print!("{}", character.to_string().color(c));
            } else {
                print!("{}", character);
            }
        }
        println!();
    }

    println!("\nEquations:");
    for (i, query) in queries.iter().enumerate() {
        let color = get_color(i);
        println!("• {}", query.color(color));
    }
}