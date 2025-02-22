use std::path::PathBuf;

use clap::{AppSettings, ArgEnum, Parser, ValueHint};

const ENV_HELP: &str = "ENVIRONMENT VARIABLES:
    _ZO_DATA_DIR            Path for zoxide data files
    _ZO_ECHO                Print the matched directory before navigating to it when set to 1
    _ZO_EXCLUDE_DIRS        List of directory globs to be excluded
    _ZO_FZF_OPTS            Custom flags to pass to fzf
    _ZO_MAXAGE              Maximum total age after which entries start getting deleted
    _ZO_RESOLVE_SYMLINKS    Resolve symlinks when storing paths";

#[derive(Debug, Parser)]
#[clap(
    bin_name = env!("CARGO_PKG_NAME"),
    about,
    author,
    after_help = ENV_HELP,
    global_setting(AppSettings::DisableHelpSubcommand),
    global_setting(AppSettings::PropagateVersion),
    version = option_env!("ZOXIDE_VERSION").unwrap_or_default()
)]
pub enum App {
    Add(Add),
    Import(Import),
    Init(Init),
    Query(Query),
    Remove(Remove),
}

/// Add a new directory or increment its rank
#[derive(Debug, Parser)]
pub struct Add {
    #[clap(min_values = 1, required = true, value_hint = ValueHint::DirPath)]
    pub paths: Vec<PathBuf>,
}

/// Import entries from another application
#[derive(Debug, Parser)]
pub struct Import {
    #[clap(value_hint = ValueHint::FilePath)]
    pub path: PathBuf,

    /// Application to import from
    #[clap(arg_enum, long)]
    pub from: ImportFrom,

    /// Merge into existing database
    #[clap(long)]
    pub merge: bool,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum ImportFrom {
    Autojump,
    Z,
}

/// Generate shell configuration
#[derive(Debug, Parser)]
pub struct Init {
    #[clap(arg_enum)]
    pub shell: InitShell,

    /// Prevents zoxide from defining any commands
    #[clap(long)]
    pub no_aliases: bool,

    /// Renames the 'z' command and corresponding aliases
    #[clap(long, default_value = "z")]
    pub cmd: String,

    /// Chooses event upon which an entry is added to the database
    #[clap(arg_enum, long, default_value = "pwd")]
    pub hook: InitHook,
}

#[derive(ArgEnum, Clone, Copy, Debug, Eq, PartialEq)]
pub enum InitHook {
    None,
    Prompt,
    Pwd,
}

#[derive(ArgEnum, Clone, Debug)]
pub enum InitShell {
    Bash,
    Elvish,
    Fish,
    Nushell,
    Posix,
    Powershell,
    Xonsh,
    Zsh,
}

/// Search for a directory in the database
#[derive(Debug, Parser)]
pub struct Query {
    pub keywords: Vec<String>,

    /// Show deleted directories
    #[clap(long)]
    pub all: bool,

    /// Use interactive selection
    #[clap(long, short, conflicts_with = "list")]
    pub interactive: bool,

    /// List all matching directories
    #[clap(long, short, conflicts_with = "interactive")]
    pub list: bool,

    /// Print score with results
    #[clap(long, short, conflicts_with = "interactive")]
    pub score: bool,

    /// Exclude a path from results
    #[clap(long, value_hint = ValueHint::DirPath, value_name = "path")]
    pub exclude: Option<String>,
}

/// Remove a directory from the database
#[derive(Debug, Parser)]
pub struct Remove {
    // Use interactive selection
    #[clap(conflicts_with = "paths", long, short, value_name = "keywords")]
    pub interactive: Option<Vec<String>>,
    #[clap(
        conflicts_with = "interactive",
        required_unless_present = "interactive",
        value_hint = ValueHint::DirPath
    )]
    pub paths: Vec<String>,
}
