use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let contents = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in contents.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
