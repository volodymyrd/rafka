use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader};
use indexmap::IndexMap;

/// Reads a properties file from the given path into a HashMap,
/// skipping empty lines and comments (lines starting with '#' or '!').
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the properties file.
///
/// # Returns
///
/// * `Ok(HashMap<String, String>)` if the file is read and parsed successfully.
/// * `Err(io::Error)` if there is an error opening or reading the file.
pub fn load_props(path: &str) -> io::Result<HashMap<String, String>> {
    let mut properties = HashMap::new();
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with('#') || trimmed_line.starts_with('!')
        {
            continue;
        }

        if let Some((key, value)) = trimmed_line.split_once('=') {
            properties.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(properties)
}

/// Creates an order-preserving map from a sequence of key-value pairs.
///
/// # Arguments
///
/// * `entries` - A slice of tuples, where each tuple is a key-value pair.
///
/// # Returns
///
/// An `IndexMap` containing the provided entries in their original order.
pub fn mk_map<K, V>(entries: &[(K, V)]) -> IndexMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    entries.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_valid_properties() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "# Database Configuration").unwrap();
        writeln!(file, "database.url=jdbc:mysql://localhost:3306/mydb").unwrap();
        writeln!(file, "database.user = myuser").unwrap();
        writeln!(file, "").unwrap(); // Empty line
        writeln!(file, "! Application Settings").unwrap();
        writeln!(file, "application.name = My Awesome App ").unwrap(); // Note the trailing space
        writeln!(file, "key.with.equals=value=with=equals").unwrap();

        let properties = load_props(file.path().to_str().unwrap()).unwrap();

        assert_eq!(properties.len(), 4);
        assert_eq!(
            properties.get("database.url").unwrap(),
            "jdbc:mysql://localhost:3306/mydb"
        );
        assert_eq!(properties.get("database.user").unwrap(), "myuser");
        assert_eq!(
            properties.get("application.name").unwrap(),
            "My Awesome App"
        );
        assert_eq!(
            properties.get("key.with.equals").unwrap(),
            "value=with=equals"
        );
    }

    #[test]
    fn test_file_not_found() {
        let result = load_props("non_existent_file.properties");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let properties = load_props(file.path().to_str().unwrap()).unwrap();
        assert!(properties.is_empty());
    }

    #[test]
    fn test_file_with_only_comments_and_empty_lines() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "# This is a comment").unwrap();
        writeln!(file, "! So is this").unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "   ").unwrap(); // Line with only whitespace

        let properties = load_props(file.path().to_str().unwrap()).unwrap();
        assert!(properties.is_empty());
    }

    #[test]
    fn test_malformed_line_is_skipped() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "this is a malformed line").unwrap();
        writeln!(file, "valid.key=valid.value").unwrap();

        let properties = load_props(file.path().to_str().unwrap()).unwrap();

        assert_eq!(properties.len(), 1);
        assert_eq!(properties.get("valid.key").unwrap(), "valid.value");
    }
}
