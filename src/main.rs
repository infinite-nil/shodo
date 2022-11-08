use clap::Parser;

use crate::post::generate_post;

mod post;
mod slugify;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    gen: Option<String>,
}

fn main() {
    let cli = Args::parse();

    if let Some(gen) = cli.gen.as_deref() {
        generate_post(gen.to_string());
    }
}
