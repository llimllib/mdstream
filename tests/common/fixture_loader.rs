use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct ConformanceTest {
    pub name: String,
    pub description: String,
    pub chunks: Vec<StreamChunk>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreamChunk {
    pub input: String,
    pub emit: String,
}

impl ConformanceTest {
    /// Load a test fixture from a TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let test: ConformanceTest = toml::from_str(&content)?;
        Ok(test)
    }

    /// Load all test fixtures from a directory
    pub fn load_from_directory<P: AsRef<Path>>(
        dir: P,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut tests = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                match Self::load_from_file(&path) {
                    Ok(test) => tests.push(test),
                    Err(e) => eprintln!("Warning: Failed to load {}: {}", path.display(), e),
                }
            }
        }

        Ok(tests)
    }
}

// Tests are in the main conformance test file
