use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Arguments {
    #[clap(default_value = "Meow")]
    /// What does the cat say?
    message: String, // input message

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Import ascii art from file
    catfile: Option<std::path::PathBuf>,
}

static DEFAULT_CAT: &str = " ∧,,,∧\n({eye} · {eye})\n/    づ{heart} ";
fn main() {
    let args = Arguments::parse();
    run(args);
}

fn run(args: Arguments) {
    let message = args.message;

    if message.to_lowercase() == "woof" {
        eprintln!("Kitties don't bark! They mew!")
    }

    let mut cat;

    match args.catfile {
        Some(path) => {
            cat = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}", e));
        }
        None => {
            cat = DEFAULT_CAT.to_string();

            if args.dead {
                cat = cat.replace("{eye}", &"\u{02e3}".red().bold().to_string());
                cat = cat.replace("{heart}", "\u{1f480}");
            } else {
                cat = cat.replace("{eye}", &"•".green().bold().to_string());
                cat = cat.replace("{heart}", &"♡".red().to_string());
            }
        }
    }

    println!("{}", message.bright_yellow().underline().on_purple());
    println!(" \\");
    println!("  \\");
    println!("{}", cat);
}
