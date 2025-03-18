pub mod common;
pub mod data;
pub mod index;
pub mod sense;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::data::parse_data_line;
use crate::common::{IndexEntry, SenseEntry, Synset};
use crate::index::parse_index_line;
use crate::sense::parse_sense_line;

/// Read a data file (data.noun, data.verb, data.adj, data.adv) and return all synsets
pub fn read_data_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<Synset>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut synsets = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // Skip copyright lines at the beginning (they start with spaces)
        if line.starts_with(' ') {
            continue;
        }
        match parse_data_line(&line) {
            Ok(synset) => synsets.push(synset),
            Err(err) => eprintln!("Error parsing data line: {}", err),
        }
    }

    Ok(synsets)
}

/// Read an index file (index.noun, index.verb, index.adj, index.adv) and return all index entries
pub fn read_index_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<IndexEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // Skip copyright lines at the beginning (they start with spaces)
        if line.starts_with(' ') {
            continue;
        }
        match parse_index_line(&line) {
            Ok(entry) => entries.push(entry),
            Err(err) => {
                if err != "Skipping copyright notice line" {
                    eprintln!("Error parsing index line: {}", err);
                }
            },
        }
    }

    Ok(entries)
}

/// Read a sense index file (index.sense) and return all sense entries
pub fn read_sense_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<SenseEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        match parse_sense_line(&line) {
            Ok(entry) => entries.push(entry),
            Err(err) => eprintln!("Error parsing sense line: {}", err),
        }
    }

    Ok(entries)
}