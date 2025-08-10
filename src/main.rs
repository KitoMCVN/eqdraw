use clap::Parser;
use eqdraw::{PlotArgs, run};

const DEFAULT_X_MIN: f64 = -10.0;
const DEFAULT_X_MAX: f64 = 10.0;
const DEFAULT_Y_MIN: f64 = -1.2;
const DEFAULT_Y_MAX: f64 = 1.2;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, num_args = 1.., required = true)]
    queries: Vec<String>,

    #[arg(long, allow_negative_numbers = true)]
    xmin: Option<f64>,

    #[arg(long, allow_negative_numbers = true)]
    xmax: Option<f64>,

    #[arg(long, allow_negative_numbers = true)]
    ymin: Option<f64>,

    #[arg(long, allow_negative_numbers = true)]
    ymax: Option<f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let args = PlotArgs {
        queries: cli.queries,
        x_min: cli.xmin.unwrap_or(DEFAULT_X_MIN),
        x_max: cli.xmax.unwrap_or(DEFAULT_X_MAX),
        y_min: cli.ymin.unwrap_or(DEFAULT_Y_MIN),
        y_max: cli.ymax.unwrap_or(DEFAULT_Y_MAX),
    };

    run(args)
}