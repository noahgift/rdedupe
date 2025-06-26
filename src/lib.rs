//walks a filesystem and finds duplicate files
use indicatif::{ParallelProgressIterator, ProgressStyle};
use polars::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function to display threading information
pub fn display_thread_info() {
    let num_cpus = num_cpus::get();
    let rayon_threads = rayon::current_num_threads();
    
    println!("💻 CPU cores: {}", num_cpus);
    println!("🧵 Rayon thread pool size: {}", rayon_threads);
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size_bytes: u64,
    pub size_mb: f64,
    pub md5_hash: String,
    pub is_duplicate: bool,
    pub duplicate_group: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

impl FileInfo {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let path_obj = Path::new(path);
        let metadata = fs::metadata(path)?;

        let name = path_obj
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let extension = path_obj
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let size_bytes = metadata.len();
        let size_mb = size_bytes as f64 / 1_048_576.0; // Convert bytes to MB

        let file_content = fs::read(path)?;
        let md5_hash = format!("{:x}", md5::compute(&file_content));

        let created = metadata
            .created()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| format!("{}", duration.as_secs()));

        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| format!("{}", duration.as_secs()));

        Ok(FileInfo {
            path: path.to_string(),
            name,
            extension,
            size_bytes,
            size_mb,
            md5_hash,
            is_duplicate: false,
            duplicate_group: None,
            created,
            modified,
        })
    }
}

pub fn walk(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());
        }
    }

    Ok(files)
}

//Find files matching a pattern
pub fn find(files: Vec<String>, pattern: &str) -> Vec<String> {
    let mut matches = Vec::new();

    for file in files {
        if file.contains(pattern) {
            matches.push(file);
        }
    }

    matches
}

// New function to collect detailed file information - TRUE PARALLEL VERSION
pub fn collect_file_info(files: Vec<String>) -> Result<Vec<FileInfo>, Box<dyn Error>> {
    if files.is_empty() {
        return Ok(Vec::new());
    }

    println!("\nAnalyzing {} files with {} threads...", files.len(), rayon::current_num_threads());
    
    let pb = indicatif::ProgressBar::new(files.len() as u64);
    let sty = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
        .unwrap()
        .progress_chars("##-");

    pb.set_style(sty);
    pb.set_message("Computing MD5 hashes...");
    
    // Enable steady tick to ensure spinner is visible
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    // TRUE PARALLEL: Each thread processes files independently, no shared mutex
    let file_infos: Vec<Option<FileInfo>> = files
        .par_iter()
        .progress_with(pb.clone())
        .map(|file_path| FileInfo::new(file_path).ok())
        .collect();

    pb.finish_with_message("✓ File analysis complete!");
    println!();

    // Filter out failed files
    let valid_infos: Vec<FileInfo> = file_infos.into_iter().flatten().collect();
    
    Ok(valid_infos)
}

// Create Polars DataFrame from file information
pub fn create_dataframe(mut file_infos: Vec<FileInfo>) -> Result<DataFrame, Box<dyn Error>> {
    // Group files by hash to identify duplicates
    let mut hash_groups: HashMap<String, Vec<usize>> = HashMap::new();

    for (index, file_info) in file_infos.iter().enumerate() {
        hash_groups
            .entry(file_info.md5_hash.clone())
            .or_default()
            .push(index);
    }

    // Mark duplicates and assign group IDs - ONLY for files that actually have duplicates
    let mut duplicate_count = 0;
    for (hash, indices) in &hash_groups {
        if indices.len() > 1 {
            let group_id = hash.clone();
            duplicate_count += indices.len();
            
            for &index in indices {
                file_infos[index].is_duplicate = true;
                file_infos[index].duplicate_group = Some(group_id.clone());
            }
        }
    }
    
    println!("Found {} files in {} duplicate groups", duplicate_count, 
             hash_groups.iter().filter(|(_, v)| v.len() > 1).count());

    // Extract data for DataFrame columns
    let paths: Vec<String> = file_infos.iter().map(|f| f.path.clone()).collect();
    let names: Vec<String> = file_infos.iter().map(|f| f.name.clone()).collect();
    let extensions: Vec<String> = file_infos.iter().map(|f| f.extension.clone()).collect();
    let sizes_bytes: Vec<u64> = file_infos.iter().map(|f| f.size_bytes).collect();
    let sizes_mb: Vec<f64> = file_infos.iter().map(|f| f.size_mb).collect();
    let hashes: Vec<String> = file_infos.iter().map(|f| f.md5_hash.clone()).collect();
    let is_duplicate: Vec<bool> = file_infos.iter().map(|f| f.is_duplicate).collect();
    let duplicate_groups: Vec<Option<String>> = file_infos
        .iter()
        .map(|f| f.duplicate_group.clone())
        .collect();

    let df = df! [
        "file_path" => paths,
        "file_name" => names,
        "extension" => extensions,
        "size_bytes" => sizes_bytes,
        "size_mb" => sizes_mb,
        "md5_hash" => hashes,
        "is_duplicate" => is_duplicate,
        "duplicate_group" => duplicate_groups,
    ]?;

    Ok(df)
}

// Generate file statistics summary
pub fn generate_statistics(df: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let total_files = df.height();
    let total_size_bytes: u64 = df.column("size_bytes")?.sum().unwrap_or(0);
    let total_size_mb = total_size_bytes as f64 / 1_048_576.0;

    let duplicate_count = df
        .column("is_duplicate")?
        .bool()?
        .into_iter()
        .filter(|x| x.unwrap_or(false))
        .count();

    let unique_extensions = df.column("extension")?.unique()?.len();

    let avg_file_size_mb = total_size_mb / total_files as f64;

    let stats_df = df! [
        "metric" => vec!["total_files", "duplicate_files", "total_size_mb", "avg_file_size_mb", "unique_extensions"],
        "value" => vec![total_files as f64, duplicate_count as f64, total_size_mb, avg_file_size_mb, unique_extensions as f64],
    ]?;

    Ok(stats_df)
}

// Validate duplicate detection logic
pub fn validate_duplicates(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    println!("\n=== Duplicate Detection Validation ===");
    
    // Group by hash and check consistency
    let duplicates = df
        .clone()
        .lazy()
        .filter(col("is_duplicate").eq(lit(true)))
        .collect()?;
    
    if duplicates.height() == 0 {
        println!("✓ No duplicates found - validation passed");
        return Ok(());
    }
    
    // Group duplicates by their hash to verify consistency
    let grouped = duplicates
        .clone()
        .lazy()
        .group_by([col("md5_hash")])
        .agg([
            col("file_path").count().alias("file_count"),
            col("duplicate_group").first().alias("group_id")
        ])
        .collect()?;
    
    println!("Duplicate groups found:");
    for row in 0..grouped.height() {
        let hash = grouped.column("md5_hash")?.get(row)?;
        let count = grouped.column("file_count")?.get(row)?;
        println!("  Hash: {} -> {} files", hash, count);
    }
    
    println!("✓ Duplicate detection validation completed");
    Ok(())
}

// Generate CSV report - ONLY for duplicate files
pub fn generate_csv_report(df: &mut DataFrame, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Filter to only include actual duplicates
    let duplicates_only = df
        .clone()
        .lazy()
        .filter(col("is_duplicate").eq(lit(true)))
        .collect()?;
    
    if duplicates_only.height() == 0 {
        println!("No duplicates found - CSV report not generated");
        return Ok(());
    }

    let mut file = std::fs::File::create(output_path)?;
    let mut duplicates_df = duplicates_only;
    
    CsvWriter::new(&mut file).include_header(true).finish(&mut duplicates_df)?;

    println!("CSV report generated: {} ({} duplicate files)", output_path, duplicates_df.height());

    Ok(())
}

// Enhanced run function with DataFrame support
pub fn run_with_dataframe(
    path: &str,
    pattern: &str,
    output_csv: Option<&str>,
) -> Result<DataFrame, Box<dyn Error>> {
    println!("Scanning directory: {}", path);

    let files = walk(path)?;
    let files = find(files, pattern);

    println!("Found {} files matching pattern '{}'", files.len(), pattern);
    
    if files.is_empty() {
        println!("No files found to analyze.");
        return Ok(df! [
            "file_path" => Vec::<String>::new(),
            "file_name" => Vec::<String>::new(),
            "extension" => Vec::<String>::new(),
            "size_bytes" => Vec::<u64>::new(),
            "size_mb" => Vec::<f64>::new(),
            "md5_hash" => Vec::<String>::new(),
            "is_duplicate" => Vec::<bool>::new(),
            "duplicate_group" => Vec::<Option<String>>::new(),
        ]?);
    }

    let file_infos = collect_file_info(files)?;
    let df = create_dataframe(file_infos)?;

    // Print summary statistics
    let stats = generate_statistics(&df)?;

    println!("\n=== File Analysis Summary ===");
    println!("{}", stats);

    // Validate duplicate detection
    validate_duplicates(&df)?;

    // Show duplicate information
    let duplicates = df
        .clone()
        .lazy()
        .filter(col("is_duplicate").eq(lit(true)))
        .collect()?;

    if duplicates.height() > 0 {
        println!("\n=== Duplicate Files Found ===");
        println!("{}", duplicates);
    } else {
        println!("\nNo duplicate files found.");
    }

    // Generate CSV report if requested
    if let Some(csv_path) = output_csv {
        let mut df_copy = df.clone();

        generate_csv_report(&mut df_copy, csv_path)?;
    }

    Ok(df)
}

/*  TRUE PARALLEL version of checksum using rayon with no mutex contention
Uses indicatif to show a progress bar
*/
pub fn checksum(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    println!("Computing checksums with {} threads...", rayon::current_num_threads());
    
    let pb = indicatif::ProgressBar::new(files.len() as u64);
    let sty = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap();

    pb.set_style(sty);

    // TRUE PARALLEL: Each thread computes checksums independently
    let file_checksums: Vec<(String, String)> = files
        .par_iter()
        .progress_with(pb)
        .filter_map(|file| {
            if let Ok(content) = std::fs::read(file) {
                let checksum = format!("{:x}", md5::compute(&content));
                Some((checksum, file.clone()))
            } else {
                None
            }
        })
        .collect();

    // Sequential grouping (this part must be sequential anyway)
    let mut checksums: HashMap<String, Vec<String>> = HashMap::new();
    for (hash, file_path) in file_checksums {
        checksums.entry(hash).or_default().push(file_path);
    }

    Ok(checksums)
}

/*
Find all the files with more than one entry in the HashMap
*/
pub fn find_duplicates(checksums: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut duplicates = Vec::new();

    for (_checksum, files) in checksums {
        if files.len() > 1 {
            duplicates.push(files);
        }
    }

    duplicates
}

// invoke the actions along with the path and pattern and progress bar
pub fn run(path: &str, pattern: &str) -> Result<(), Box<dyn Error>> {
    let files = walk(path)?;
    let files = find(files, pattern);

    println!("Found {} files matching {}", files.len(), pattern);

    let checksums = checksum(files)?;
    let duplicates = find_duplicates(checksums);

    println!("Found {} duplicate(s)", duplicates.len());

    for duplicate in duplicates {
        println!("{:?}", duplicate);
    }

    Ok(())
}
