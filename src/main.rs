use clap::Parser;

#[derive(Parser)]
struct Arguments {
    message: String // input message
}
fn main() {
    let args = Arguments::parse();
    let message = args.message;
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    =( I )=");
}
