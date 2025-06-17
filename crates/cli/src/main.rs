use clap::Parser;
use core::extract::export_json;

/// Petit wrapper de démonstration.
#[derive(Parser)]
#[command(
    name    = "commitvista",
    version = "0.1.0",
    about   = "Export Git history to JSON (POC)"
)]
struct Cli {
    /// Chemin du dépôt Git (par défaut = dossier courant)
    #[arg(default_value = ".")]
    repo: String,

    /// Fichier de sortie JSON
    #[arg(long, default_value = "commits.json")]
    out: String,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    export_json(args.repo, args.out)?;
    println!("✅ commits exported successfully");
    Ok(())
}

