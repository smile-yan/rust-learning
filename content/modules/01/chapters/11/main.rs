use clap::Parser;

#[derive(Parser)]
#[command(name = "greet")]
struct Cli {
    #[arg(short, long, default_value = "World")]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u32,

    #[arg(short, long)]
    shout: bool,
}

fn main() {
    let args = Cli::parse();
    for _ in 0..args.count {
        let mut msg = format!("Hello, {}!", args.name);
        if args.shout { msg = msg.to_uppercase(); }
        println!("{}", msg);
    }
}
