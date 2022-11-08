use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    gen: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello, {:?}", args);
}
