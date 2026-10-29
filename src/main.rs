mod models;
mod pricing;
mod rollout;

use clap::{Parser, Subcommand};
use colored::*;
use rollout::RolloutScanner;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "codexco")]
#[command(about = "Real-time session cost analyzer and monitor for OpenAI Codex", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show cost and token usage for the active or most recent session
    Current,
    /// Display recent session history with associated costs
    History {
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    /// Show aggregate spend across all recorded sessions
    Summary,
    /// Monitor the active session in real-time
    Watch {
        #[arg(short, long, default_value_t = 2)]
        interval: u64,
    },
}

fn print_session_card(session: &models::SessionSummary) {
    println!("{}", "=".repeat(65).bright_blue());
    println!(
        "{} {}",
        "Session:   ".bold(),
        session.session_id.bright_yellow()
    );
    if let Some(ts) = &session.timestamp {
        println!("{} {}", "Timestamp: ".bold(), ts.bright_black());
    }
    if let Some(cwd) = &session.cwd {
        println!("{} {}", "Workspace: ".bold(), cwd.cyan());
    }
    println!(
        "{} {}",
        "Models:    ".bold(),
        session.models_used.join(", ").magenta()
    );
    println!("{}", "-".repeat(65).bright_blue());

    if session.breakdown.len() > 1 {
        println!("{}", "BREAKDOWN BY MODEL:".bold().yellow());
        for m in &session.breakdown {
            let u = &m.token_usage;
            println!(
                "  • {:<16} {:>10} tok (in: {}, cached: {}, out: {}) -> {}",
                m.model.bright_magenta(),
                u.total_tokens,
                u.non_cached_input(),
                u.cached_input(),
                u.output_tokens,
                format!("${:.5} USD", m.cost_usd).green().bold()
            );
        }
        println!("{}", "-".repeat(65).bright_blue());
    }

    let u = &session.token_usage;
    println!("{:<30} {:>14}", "Input Tokens (Total):", u.input_tokens.to_string().white());
    println!("{:<30} {:>14}", "  - Non-cached:", u.non_cached_input().to_string().white());
    println!("{:<30} {:>14}", "  - Cached (savings):", u.cached_input().to_string().green());
    println!("{:<30} {:>14}", "Output Tokens:", u.output_tokens.to_string().white());
    if u.reasoning_output_tokens > 0 {
        println!("{:<30} {:>14}", "  - Reasoning:", u.reasoning_output_tokens.to_string().bright_black());
    }
    println!("{:<30} {:>14}", "Total Accumulated Tokens:", u.total_tokens.to_string().bold());
    println!("{}", "-".repeat(65).bright_blue());
    println!(
        "{:<30} {:>14}",
        "ESTIMATED TOTAL COST:".bold().green(),
        format!("${:.5} USD", session.estimated_cost_usd).bold().bright_green()
    );
    println!("{}", "=".repeat(65).bright_blue());
}

fn main() {
    let cli = Cli::parse();
    let scanner = match RolloutScanner::new() {
        Some(s) => s,
        None => {
            eprintln!(
                "{} Could not locate ~/.codex directory on your system.",
                "ERROR:".bold().red()
            );
            std::process::exit(1);
        }
    };

    match cli.command.unwrap_or(Commands::Current) {
        Commands::Current => {
            match scanner.get_latest_session() {
                Some(session) => print_session_card(&session),
                None => println!("{}", "No Codex sessions found.".yellow()),
            }
        }
        Commands::History { limit } => {
            let history = scanner.get_session_history(limit);
            if history.is_empty() {
                println!("{}", "No sessions found in history.".yellow());
                return;
            }

            println!("{}", "CODEX SESSION HISTORY".bold().underline());
            println!(
                "{:<38} {:<18} {:>12} {:>14}",
                "SESSION ID", "MODELS", "TOKENS", "COST (USD)"
            );
            println!("{}", "-".repeat(86));

            for s in history {
                let models_str = if s.models_used.len() == 1 {
                    s.models_used[0].clone()
                } else {
                    format!("{} (+{})", s.models_used[0], s.models_used.len() - 1)
                };

                println!(
                    "{:<38} {:<18} {:>12} {:>14}",
                    s.session_id.chars().take(36).collect::<String>().bright_yellow(),
                    models_str.bright_magenta(),
                    s.token_usage.total_tokens,
                    format!("${:.4}", s.estimated_cost_usd).bold().green()
                );
            }
        }
        Commands::Summary => {
            let history = scanner.get_session_history(100);
            let mut total_tokens = 0i64;
            let mut total_cost = 0.0f64;
            let session_count = history.len();

            for s in &history {
                total_tokens += s.token_usage.total_tokens;
                total_cost += s.estimated_cost_usd;
            }

            println!("{}", "AGGREGATE SPEND SUMMARY".bold().underline());
            println!("{:<25} {}", "Analyzed sessions:", session_count.to_string().cyan());
            println!("{:<25} {}", "Consumed tokens:", total_tokens.to_string().white());
            println!(
                "{:<25} {}",
                "Estimated Total Spend:".bold(),
                format!("${:.4} USD", total_cost).bold().bright_green()
            );
        }
        Commands::Watch { interval } => {
            println!("{}", "Watching active Codex session in real-time (Press Ctrl+C to exit)...".cyan());
            let mut last_tokens = -1i64;

            loop {
                if let Some(session) = scanner.get_latest_session() {
                    if session.token_usage.total_tokens != last_tokens {
                        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                        print_session_card(&session);
                        last_tokens = session.token_usage.total_tokens;
                    }
                }
                thread::sleep(Duration::from_secs(interval));
            }
        }
    }
}
