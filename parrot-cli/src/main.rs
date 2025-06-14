use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="parrot-cli", version="0.0.1", about="Pretty useless", long_about=None)]
struct Args {
    #[arg(short, long, help = "Input to be repeated back")]
    input: String,
}

fn main() {
    let args = Args::parse();

    print!("{}", args.input)
}
