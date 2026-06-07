pub mod dump;
pub mod dumps;
pub mod generate;
pub mod list;

use clap::{Parser, Subcommand};

use crate::hdwallet::info;

use dump::{dump, DumpArgs};
use dumps::{dumps, DumpsArgs};
use generate::entropy::{generate_entropy, GenerateEntropyArgs};
use generate::mnemonic::{generate_mnemonic, GenerateMnemonicArgs};
use generate::seed::{generate_seed, GenerateSeedArgs};
use list::{list_cryptocurrencies, list_languages, list_strengths};

#[derive(Parser)]
#[command(
    name = "hdwallet",
    version = info::__VERSION__,
    about = info::__DESCRIPTION__,
    help_template = "{name} {version}\n{about}\n\n{usage-heading} {usage}\n\n{all-args}",
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        name = "generate",
        visible_aliases = ["g"],
        about = "Select Generate for HDWallet"
    )]
    Generate {
        #[command(subcommand)]
        command: GenerateCommands,
    },
    #[command(name = "dump", visible_aliases = ["d"], about = "Select Dump hdwallet keys")]
    Dump(DumpArgs),
    #[command(name = "dumps", visible_aliases = ["ds"], about = "Select Dumps hdwallet keys")]
    Dumps(DumpsArgs),
    #[command(
        name = "list",
        visible_aliases = ["l"],
        about = "Select List for HDWallet information"
    )]
    List {
        #[command(subcommand)]
        command: ListCommands,
    },
}

#[derive(Subcommand)]
enum GenerateCommands {
    #[command(
        name = "entropy",
        visible_aliases = ["e"],
        about = "Select Entropy for generation entropy"
    )]
    Entropy(GenerateEntropyArgs),
    #[command(
        name = "mnemonic",
        visible_aliases = ["m"],
        about = "Select Mnemonic for generation mnemonic"
    )]
    Mnemonic(GenerateMnemonicArgs),
    #[command(
        name = "seed",
        visible_aliases = ["s"],
        about = "Select Seed for generation seed"
    )]
    Seed(GenerateSeedArgs),
}

#[derive(Subcommand)]
enum ListCommands {
    #[command(
        name = "cryptocurrencies",
        visible_aliases = ["c"],
        about = "List Available cryptocurrencies of HDWallet"
    )]
    Cryptocurrencies,
    #[command(
        name = "languages",
        visible_aliases = ["l"],
        about = "List Languages of mnemonic words"
    )]
    Languages,
    #[command(
        name = "strengths",
        visible_aliases = ["s"],
        about = "List Strengths of mnemonic words"
    )]
    Strengths,
}

/// Run the HDWallet CLI (mirrors Python `hdwallet/cli/__main__.py`).
pub fn run() -> i32 {
    match Cli::try_parse() {
        Ok(cli) => match cli.execute() {
            Ok(output) => {
                println!("{output}");
                0
            }
            Err(error) => {
                eprintln!("Error: {error}");
                1
            }
        },
        Err(error) => {
            error.print().expect("print clap error");
            i32::from(error.exit_code())
        }
    }
}

impl Cli {
    fn execute(self) -> Result<String, String> {
        match self.command {
            Commands::Generate { command } => match command {
                GenerateCommands::Entropy(args) => {
                    generate_entropy(args).map_err(|error| error.to_string())
                }
                GenerateCommands::Mnemonic(args) => {
                    generate_mnemonic(args).map_err(|error| error.to_string())
                }
                GenerateCommands::Seed(args) => {
                    generate_seed(args).map_err(|error| error.to_string())
                }
            },
            Commands::Dump(args) => dump(args).map_err(|error| error.to_string()),
            Commands::Dumps(args) => dumps(args).map_err(|error| error.to_string()),
            Commands::List { command } => match command {
                ListCommands::Cryptocurrencies => {
                    list_cryptocurrencies().map_err(|error| error.to_string())
                }
                ListCommands::Languages => list_languages().map_err(|error| error.to_string()),
                ListCommands::Strengths => list_strengths().map_err(|error| error.to_string()),
            },
        }
    }
}
