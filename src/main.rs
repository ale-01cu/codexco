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
#[command(about = "Monitor y calculador de costos de sesiones para Codex", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Muestra el costo y uso de tokens de la sesión activa o más reciente
    Current,
    /// Muestra el historial de sesiones recientes con sus costos
    History {
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    /// Resumen global acumulado de costos (hoy, últimos días, total)
    Summary,
    /// Monitorea en tiempo real el costo de la sesión activa
    Watch {
        #[arg(short, long, default_value_t = 2)]
        interval: u64,
    },
}

fn print_session_card(session: &models::SessionSummary) {
    println!("{}", "=".repeat(60).bright_blue());
    println!(
        "{} {}",
        "Sesión:".bold(),
        session.session_id.bright_yellow()
    );
    if let Some(ts) = &session.timestamp {
        println!("{} {}", "Fecha: ".bold(), ts.bright_black());
    }
    if let Some(cwd) = &session.cwd {
        println!("{} {}", "Workspace:".bold(), cwd.cyan());
    }
    println!("{} {}", "Modelo:".bold(), session.model.magenta());
    println!("{}", "-".repeat(60).bright_blue());

    let u = &session.token_usage;
    println!("{:<28} {:>12}", "Tokens Entrada (Total):", u.input_tokens.to_string().white());
    println!("{:<28} {:>12}", "  - No cacheados:", u.non_cached_input().to_string().white());
    println!("{:<28} {:>12}", "  - Cacheados (ahorro):", u.cached_input().to_string().green());
    println!("{:<28} {:>12}", "Tokens Salida:", u.output_tokens.to_string().white());
    if u.reasoning_output_tokens > 0 {
        println!("{:<28} {:>12}", "  - Razonamiento:", u.reasoning_output_tokens.to_string().bright_black());
    }
    println!("{:<28} {:>12}", "Total Tokens:", u.total_tokens.to_string().bold());
    println!("{}", "-".repeat(60).bright_blue());
    println!(
        "{:<28} {:>12}",
        "COSTO ESTIMADO:".bold().green(),
        format!("${:.5} USD", session.estimated_cost_usd).bold().bright_green()
    );
    println!("{}", "=".repeat(60).bright_blue());
}

fn main() {
    let cli = Cli::parse();
    let scanner = match RolloutScanner::new() {
        Some(s) => s,
        None => {
            eprintln!(
                "{} No se encontró el directorio ~/.codex en tu sistema.",
                "ERROR:".bold().red()
            );
            std::process::exit(1);
        }
    };

    match cli.command.unwrap_or(Commands::Current) {
        Commands::Current => {
            match scanner.get_latest_session() {
                Some(session) => print_session_card(&session),
                None => println!("{}", "No se encontraron sesiones registradas de Codex.".yellow()),
            }
        }
        Commands::History { limit } => {
            let history = scanner.get_session_history(limit);
            if history.is_empty() {
                println!("{}", "No se encontraron sesiones en el historial.".yellow());
                return;
            }

            println!("{}", "HISTORIAL DE SESIONES DE CODEX".bold().underline());
            println!(
                "{:<38} {:<15} {:>12} {:>14}",
                "ID SESIÓN", "MODELO", "TOKENS", "COSTO (USD)"
            );
            println!("{}", "-".repeat(82));

            for s in history {
                println!(
                    "{:<38} {:<15} {:>12} {:>14}",
                    s.session_id.chars().take(36).collect::<String>().bright_yellow(),
                    s.model.bright_magenta(),
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

            println!("{}", "RESUMEN ACUMULADO DE GASTOS".bold().underline());
            println!("{:<25} {}", "Sesiones analizadas:", session_count.to_string().cyan());
            println!("{:<25} {}", "Tokens consumidos:", total_tokens.to_string().white());
            println!(
                "{:<25} {}",
                "Gasto Total Estimado:".bold(),
                format!("${:.4} USD", total_cost).bold().bright_green()
            );
        }
        Commands::Watch { interval } => {
            println!("{}", "Monitoreando sesión activa de Codex (Ctrl+C para salir)...".cyan());
            let mut last_tokens = -1i64;

            loop {
                if let Some(session) = scanner.get_latest_session() {
                    if session.token_usage.total_tokens != last_tokens {
                        // Limpiar pantalla simple en terminal
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
