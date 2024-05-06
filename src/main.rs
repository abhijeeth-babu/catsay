use clap::Parser;

#[derive(Parser)]
struct Arguments {
    #[clap(default_value="Meow")]
    message: String, // input message
}
fn main() {
    let args = Arguments::parse();
    let message = args.message;
    
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    let cat = std::fs::read_to_string("ascii_art/cat.txt")
        .unwrap_or_else(|e| panic!("{}", e));
    println!("{}", cat);
}

