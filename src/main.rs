use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, use_value_delimiter = true)]
    interactions: Vec<String>,

    #[arg(short, long, required=false)]
    boardStateFilePath: Option<String>,
}

fn main() {
    let args = Args::parse();

    for x in &args.interactions {
        println!("{x}");
    }
}
