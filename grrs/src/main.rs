use clap::Parser;

// Search for a pattern in a file and display the lines that contains it.
#[derive(Parser)]
struct CLI {
    // pattern to look for
    pattern: String,
    // The path to the file to read.
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}