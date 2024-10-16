use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: Option<String>,  // Make 'name' required by removing Option<>

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    let users_name = match args.name {
        Some(name) => name,
        None => "Mysterious User".to_string(),
    };
    for _ in 0..args.count {
        println!("Hello {}!", users_name)
    }
}
