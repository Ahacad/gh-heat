use anyhow::Result;
use clap::Parser;
use colored::Colorize;

mod error;
mod github;
mod heatmap;

use github::GithubClient;
use heatmap::Heatmap;

/// GitHub Contribution Heatmap Generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// GitHub username to generate heatmap for
    #[clap(index = 1)]
    username: String,

    /// Number of days to include in the heatmap (default: 365)
    #[clap(short, long, default_value = "365")]
    days: u32,

    /// Use a dark color scheme (red gradient)
    #[clap(short, long)]
    dark_mode: bool,

    /// Use symbols instead of colors
    #[clap(short, long)]
    symbols: bool,

    /// Show numbers instead of colors or symbols
    #[clap(short = 'n', long)]
    numbers: bool,

    /// Show total contribution counts
    #[clap(short, long)]
    totals: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let client = GithubClient::new()?;
    let contributions = client.get_user_contributions(&args.username, args.days)?;
    
    let heatmap = Heatmap::new(contributions);
    
    if args.totals {
        let total_commits = heatmap.total_contributions();
        let active_days = heatmap.active_days();
        let max_contributions = heatmap.max_contributions_in_day();
        let average = if active_days > 0 {
            total_commits as f64 / active_days as f64
        } else {
            0.0
        };

        println!("\nUser: {}", args.username.bright_white().bold());
        println!("Total Contributions: {}", total_commits.to_string().green());
        println!("Active Days: {}", active_days.to_string().green());
        println!("Max Contributions in a Day: {}", max_contributions.to_string().green());
        println!("Average Contributions on Active Days: {:.2}\n", average);
    }
    
    // Render the heatmap
    heatmap.render(args.dark_mode, args.symbols, args.numbers);
    
    Ok(())
}
