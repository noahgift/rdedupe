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
        #[clap(long, default_value = "*")]
        pattern: String,
    },
    Dedupe {
        #[clap(long, default_value = ".")]
        path: String,
        #[clap(long, default_value = "*")]
        pattern: String,
    },
    //create count with path and pattern defaults for both
    Count {
        #[clap(long, default_value = ".")]
        path: String,
        #[clap(long, default_value = "*")]
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
            //find duplicates matching a pattern
            println!("Searching for duplicates in {}", path);
            let files = rdedupe::walk(&path).unwrap();
            //for files matching pattern, find duplicates
            let files = rdedupe::find(files, &pattern);
            println!("Found {} files matching {}", files.len(), pattern);
            //found duplicates
            let checksums = rdedupe::checksum(files).unwrap();
            let duplicates = rdedupe::find_duplicates(checksums);
            println!("Found {} duplicate(s)", duplicates.len());
            //print duplicates
            for duplicate in duplicates {
                println!("Duplicate files:");
                for file in duplicate {
                    println!("{}", file);
                }
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
