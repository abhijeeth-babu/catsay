use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::io::Read;

#[derive(Parser, Default, Debug)]
#[command(author = "abhi", version, about)]
/// What does the kitty say!
struct Arguments {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String, // input message

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Import ascii art from file
    catfile: Option<std::path::PathBuf>,

    #[clap(short = 'i', long = "stdin")]
    /// Read message from STDIN
    stdin: bool,
}

static DEFAULT_CAT: &str = " ∧,,,∧\n({eye} · {eye})\n/    づ{heart} ";
fn main() -> Result<()> {
    let args = Arguments::parse();
    let mut message: String = String::new();

    if args.stdin {
        std::io::stdin()
            .read_to_string(&mut message)
            .with_context(|| "Could not read from stdin")?;
        message.pop();
    } else {
        message = args.message;
    }

    if message.to_lowercase() == "woof" {
        eprintln!("Kitties don't bark! They meow!")
    }

    let mut cat: String;

    match args.catfile {
        Some(path) => {
            cat = std::fs::read_to_string(&path)
                .with_context(|| format!("Could not read file {:?}", path))?;
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

    Ok(())
}
