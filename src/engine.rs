// src/engine.rs

//! This module contains the Quine Engine, the core orchestrator of the QEMO system.

use std::fs;
use std::path::{Path, PathBuf};

// In-memory representation of a single source file.
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
}

// The main engine struct.
pub struct QuineEngine {
    source_files: Vec<SourceFile>,
}

impl QuineEngine {
    // Creates a new, empty QuineEngine.
    pub fn new() -> Self {
        QuineEngine {
            source_files: Vec::new(),
        }
    }

    // The main execution loop of the engine.
    pub fn run(&mut self) {
        println!("🚀 Quine Engine Initialized. Beginning Ingestion Phase...");
        self.ingest_source_files(Path::new("."));
        println!("✅ Ingestion Phase Complete. Found {} source files.", self.source_files.len());
    }

    // Step 1: Ingestion.
    // Recursively finds all Markdown files and loads them into memory.
    fn ingest_source_files(&mut self, dir: &Path) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        self.ingest_source_files(&path);
                    } else if let Some(extension) = path.extension() {
                        if extension == "md" {
                            println!("   -> Ingesting: {}", path.display());
                            if let Ok(content) = fs::read_to_string(&path) {
                                self.source_files.push(SourceFile {
                                    path: path.clone(),
                                    content,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
