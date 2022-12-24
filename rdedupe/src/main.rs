//Searches a path for duplicate files
use clap::Parser;

#[derive(Parser)]
//add extended help

#[clap(version = "1.0", author = "Noah Gift", about = "Finds duplicate files"
, after_help = "Example: rdedupe search --path . --pattern .txt")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Search {
        #[clap(short, long, default_value = ".")]
        path: String,
        pattern: Option<String>,
    },
    Dedupe {
        #[clap(short, long, default_value = ".")]
        path: String,
        pattern: Option<String>,
    },
    //create count with path and pattern defaults for both
    Count {
        #[clap(short, long, default_value = ".")]
        path: String,
        pattern: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Search { path, pattern }) => {
            println!("Searching for files in {}", path);
            let files = rdedupe::walk(&path).unwrap();
            println!("Found {} files", files.len());
            //if pattern is not none, call find
            if let Some(pattern) = pattern {
                let files = rdedupe::find(files, &pattern);
                println!("Found {} files matching {}", files.len(), pattern);
                //print files
                for file in files {
                    println!("{}", file);
                }
            }
        }
        Some(Commands::Dedupe { path, pattern }) => {
            println!("Finding duplicate files in {}", path);
            let files = rdedupe::walk(&path).unwrap();
            println!("Found {} files", files.len());
            //if pattern is not none, call find
            if let Some(pattern) = pattern {
                let files = rdedupe::find(files, &pattern);
                println!("Found {} files matching {}", files.len(), pattern);
                //print files
                for file in files {
                    println!("{}", file);
                }
            }
        }
        Some(Commands::Count { path, pattern }) => {
            println!("Counting files in {}", path);
            let files = rdedupe::walk(&path).unwrap();
            println!("Found {} files", files.len());
            //if pattern is not none, call find
            if let Some(pattern) = pattern {
                let files = rdedupe::find(files, &pattern);
                println!("Found {} files matching {}", files.len(), pattern);
                //print files
                for file in files {
                    println!("{}", file);
                }
            }
        }
        None => {
            println!("No command given");
        }
    }
}
