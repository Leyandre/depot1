use clap::{ArgGroup, Parser};

/// Représente les arguments en paramètres de ligne de commande
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(ArgGroup::new("commands").required(true).args(["scan","search"])))]
pub struct CliArguments {

    /// (Needs --path)   Scan and fetch metadata's file at the pointed path
    #[arg(long, conflicts_with = "kwrds", requires = "path")]
    scan: bool,

    /// (Used with --scan) Chemin où trouver les fichiers à analyser
    #[arg(long, value_name = "PATH")]
    path: Option<std::path::PathBuf>,

    /// (Needs --kwrds)   Search keywords in directory containing previous metadata scanned and return file's names associated
    #[arg(long, conflicts_with = "path", requires = "kwrds")]
    search : bool,

    /// (Used with --search) Keyword(s) used to filtered metadata's files
    #[arg(long, value_name = "KEYWORDS")]
    kwrds : Option<String>,
}

impl CliArguments {
    pub fn new() -> CliArguments {
        CliArguments::parse()
    }

    pub fn path(&self) -> &std::path::Path {
        self.path.as_ref().unwrap().as_path()
    }

    pub fn kwords(&self) -> &String {
        self.kwrds.as_ref().unwrap()
    }

    pub fn scan(&self) -> bool {
        self.scan
    }

    pub fn search(&self) -> bool {
        self.search
    }
}