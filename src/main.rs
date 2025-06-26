//Searches a path for duplicate files
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.1",
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
        #[clap(long, help = "Generate detailed CSV report")]
        csv: Option<String>,
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
        Some(Commands::Dedupe { path, pattern, csv }) => {
            //dedupe files matching a pattern with enhanced reporting
            //display the progress bar using indicatif
            rdedupe::display_thread_info();
            println!("Analyzing files in {} matching '{}'", path, pattern);
            
            // Always use enhanced DataFrame functionality for better progress reporting
            let result = rdedupe::run_with_dataframe(&path, &pattern, csv.as_deref());

            match result {
                Ok(df) => {
                    println!("\n=== Analysis Complete ===");
                    println!("Total files analyzed: {}", df.height());
                    if let Some(csv_path) = csv {
                        println!("Detailed CSV report saved to: {}", csv_path);
                    }
                }
                
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
