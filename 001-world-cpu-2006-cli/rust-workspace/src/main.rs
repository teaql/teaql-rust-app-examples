use clap::{Parser, Subcommand};
use fifa_world_cup_2026_service::{service_runtime, ServiceRuntimeConfig};


mod seed;
mod commands;
mod ansi;

#[derive(Parser)]
#[command(name = "wc2026", version, about = "FIFA World Cup 2026 CLI - groups, fixtures, standings & more")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show all 12 World Cup groups
    Groups,
    /// Show detailed standings for a specific group
    Group {
        /// Group letter (A-L)
        #[arg(index = 1)]
        group_letter: String,
    },
    /// Show global power rankings across all 48 teams
    Rank {
        /// Show top N teams
        #[arg(short, long, default_value_t = 48)]
        top: usize,
        /// Filter by group letter
        #[arg(short, long)]
        group: Option<String>,
    },
    /// Record a match result
    Record {
        /// Match number
        #[arg(short, long)]
        match_number: Option<i32>,
        /// Home team code (e.g. BRA)
        #[arg(long)]
        home: Option<String>,
        /// Away team code (e.g. MAR)
        #[arg(long)]
        away: Option<String>,
        /// Score (e.g. 3-1)
        #[arg(short, long)]
        score: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = {
        if let Some(mut path) = dirs::home_dir() {
            path.push(".wc2026");
            std::fs::create_dir_all(&path)?;
            path.push("worldcup.db");
            path.to_string_lossy().to_string()
        } else {
            "worldcup.db".to_string()
        }
    };
    
    let config = ServiceRuntimeConfig {
        database_url: format!("sqlite:{}", db_path),
    };
    
    let ctx = service_runtime(config).await?;
    
    // Seed data if not initialized
    seed::seed_data(&ctx).await?;

    if std::env::args().len() > 1 {
        let cli = Cli::parse();
        run_command(&ctx, cli.command).await?;
    } else {
        use std::io::{self, Write};
        
        println!("{}{}Welcome to FIFA World Cup 2026 Interactive CLI!{}", ansi::AnsiColors::BOLD, ansi::AnsiColors::GREEN, ansi::AnsiColors::RESET);
        println!("{}Type 'help' for commands, 'clear' to clear screen, 'exit' or 'quit' to exit.{}", ansi::AnsiColors::DIM, ansi::AnsiColors::RESET);
        println!();
        
        let mut input = String::new();
        loop {
            print!("{}{}{}wc2026> {}", ansi::AnsiColors::BOLD, ansi::AnsiColors::CYAN, ansi::AnsiColors::RESET, ansi::AnsiColors::RESET);
            io::stdout().flush()?;
            
            input.clear();
            if io::stdin().read_line(&mut input)? == 0 {
                break;
            }
            
            let line = input.trim();
            if line.is_empty() {
                continue;
            }
            if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
                break;
            }
            if line.eq_ignore_ascii_case("clear") {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush()?;
                continue;
            }
            
            let mut args = vec!["wc2026"];
            // We use simple split_whitespace. This won't support quoted strings like "3-1", 
            // but our CLI arguments (like MEX RSA 3-1) don't require quotes.
            args.extend(line.split_whitespace());
            
            match Cli::try_parse_from(args) {
                Ok(cli) => {
                    if let Err(e) = run_command(&ctx, cli.command).await {
                        println!("{}Error: {}{}", ansi::AnsiColors::RED, e, ansi::AnsiColors::RESET);
                    }
                    println!();
                }
                Err(e) => {
                    let _ = e.print();
                    println!();
                }
            }
        }
    }

    Ok(())
}

async fn run_command(ctx: &teaql_runtime::UserContext, command: Commands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Groups => commands::groups(ctx).await?,
        Commands::Group { group_letter } => commands::group(ctx, &group_letter).await?,
        Commands::Rank { top, group } => commands::rank(ctx, top, group).await?,
        Commands::Record { match_number, home, away, score } => {
            commands::record(ctx, match_number, home, away, &score).await?
        }
    }
    Ok(())
}