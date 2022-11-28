use clap::Parser;

/// Représente les arguments en paramètres de ligne de commande
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArguments {
    /// Commande à exécuter
    #[arg(short, long)]
    command: String,

    /// Chemin où trouver les fichiers à analyser
    #[arg(short, long)]
    path: std::path::PathBuf,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::parse()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_path()
    }
}