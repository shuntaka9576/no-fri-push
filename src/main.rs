use clap::Parser;
use unicode_width::UnicodeWidthStr;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(
    name = "no-fri-push",
    version = VERSION,
    about = "Don't Push to Production on Friday!"
)]
struct Cli {
    #[arg(
        long,
        default_value = "Don't push on Friday!",
        help = "Specify message"
    )]
    message: String,
}

fn main() {
    let cli = Cli::parse();
    let msg_width = UnicodeWidthStr::width(cli.message.as_str());
    let dash_line = "-".repeat(msg_width + 2);

    println!("|{}|", dash_line);
    println!("| {} |", cli.message);
    println!("|{}|", dash_line);

    let ascii_art_lines = [r"\ (•◡•) /", r" \     /", r"  --", r"  |  |", r"   |_ |_"];

    let total_width = msg_width + 4;

    for &line in &ascii_art_lines {
        let line_width = UnicodeWidthStr::width(line);
        if line_width < total_width {
            let left_spaces = (total_width - line_width) / 2;
            print!("{}", " ".repeat(left_spaces));
        }
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli_definition() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parse_default_args() {
        let cli = Cli::parse_from(&["test-binary"]);
        assert_eq!(cli.message, "Don't push on Friday!");
    }

    #[test]
    fn parse_custom_message() {
        let cli = Cli::parse_from(&["test-binary", "--message", "Hello, world!"]);
        assert_eq!(cli.message, "Hello, world!");
    }
}
