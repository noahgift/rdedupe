//walks a filesystem and finds duplicate files
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::error::Error;
use walkdir::WalkDir;

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

/*  Parallel version of checksum using rayon with a mutex to ensure
 that the HashMap is not accessed by multiple threads at the same time
Uses indicatif to show a progress bar
*/
pub fn checksum(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    //set the progress bar style to allow for elapsed time and percentage complete
    let checksums = std::sync::Mutex::new(HashMap::new());
    let pb = indicatif::ProgressBar::new(files.len() as u64);
    let sty = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap();
    pb.set_style(sty);
    files.par_iter().progress_with(pb).for_each(|file| {
        let checksum = md5::compute(std::fs::read(file).unwrap());
        let checksum = format!("{:x}", checksum);
        let mut checksums = checksums.lock().unwrap();
        checksums
            .entry(checksum)
            .or_insert_with(Vec::new)
            .push(file.to_string());
    });
    Ok(checksums.into_inner().unwrap())
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
