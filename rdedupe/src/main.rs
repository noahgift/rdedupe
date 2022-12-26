//Searches a path for duplicate files
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Finds duplicate files",
    after_help = "Example: rdedupe search --path . --pattern .txt"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Search {
        #[clap(long, default_value = ".")]
        path: String,
        #[clap(long, default_value = "")]
        pattern: String,
    },
    Dedupe {
        #[clap(long, default_value = ".")]
        path: String,
        #[clap(long, default_value = "")]
        pattern: String,
    },
    //create count with path and pattern defaults for both
    Count {
        #[clap(long, default_value = ".")]
        path: String,
        #[clap(long, default_value = "")]
        pattern: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Search { path, pattern }) => {
            println!("Searching for files in {} matching {}", path, pattern);
            let files = rdedupe::walk(&path).unwrap();
            let files = rdedupe::find(files, &pattern);
            //print count of files matching pattern
            println!("Found {} files matching {}", files.len(), pattern);
            //print files
            for file in files {
                println!("{}", file);
            }
        }
        Some(Commands::Dedupe { path, pattern }) => {
            //dedupe files matching a pattern
            //display the progress bar using indicatif
            println!("Deduping files in {} matching {}", path, pattern);
            let result = rdedupe::run(&path, &pattern);
            match result {
                Ok(_) => println!("Deduping complete"),
                Err(e) => println!("Error: {}", e),
            }
        }
        Some(Commands::Count { path, pattern }) => {
            //count files matching a pattern
            println!("Counting files in {} matching {}", path, pattern);
            let files = rdedupe::walk(&path).unwrap();
            let files = rdedupe::find(files, &pattern);
            println!("Found {} files matching {}", files.len(), pattern);
        }

        None => {
            println!("No command given");
        }
    }
}
