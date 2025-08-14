use clap::Parser;
use eqdraw::{run, PlotArgs};

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
        x_min: cli.xmin,
        x_max: cli.xmax,
        y_min: cli.ymin,
        y_max: cli.ymax,
    };

    run(args)
}
