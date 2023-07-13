use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    container: Option<String>,
    #[arg(short, long)]
    dockerfile: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("container: {:?}", cli.container.as_deref());
    println!("dockerfile: {:?}", cli.dockerfile.as_deref());
}