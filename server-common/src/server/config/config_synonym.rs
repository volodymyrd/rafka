use std::sync::Arc;
use std::time::Duration;
use tracing::error;

/// A type alias for the converter function for better readability.
/// The function takes an owned String and returns an owned String.
pub type Converter = Arc<dyn Fn(String) -> String + Send + Sync>;

/// Represents a synonym for a configuration plus a conversion function.
/// The conversion function is necessary for cases where the synonym is
/// denominated in different units (e.g., hours versus milliseconds).
#[derive(Clone)]
pub struct ConfigSynonym {
    name: String,
    converter: Converter,
}

impl ConfigSynonym {
    /// Creates a new ConfigSynonym with a specific name and converter.
    pub fn new(name: String, converter: Converter) -> Self {
        Self { name, converter }
    }

    /// Creates a new ConfigSynonym where the converter is the identity function.
    pub fn new_identity(name: String) -> Self {
        Self {
            name,
            converter: Arc::new(|s| s),
        }
    }

    /// Returns the name of the synonym.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a reference to the converter function.
    pub fn converter(&self) -> &Converter {
        &self.converter
    }

    /// Returns converter function.
    pub fn own_converter(self) -> Converter {
        self.converter
    }
}

/// A private helper function to parse a string input into an integer.
/// Corresponds to the `valueToInt` method in the Java code.
fn value_to_int(input: &str, default_value: i32, what: &str) -> i32 {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return default_value;
    }
    match trimmed.parse::<i32>() {
        Ok(val) => val,
        Err(e) => {
            error!(
                "{} failed: unable to parse '{}' as an integer: {}",
                what, trimmed, e
            );
            default_value
        }
    }
}

/// Converter function that turns a string representing hours into milliseconds.
pub fn hours_to_milliseconds(input: String) -> String {
    let hours = value_to_int(&input, 0, "hours_to_milliseconds");
    let millis = Duration::from_secs((hours as u64) * 3600).as_millis();
    millis.to_string()
}

/// Converter function that turns a string representing minutes into milliseconds.
pub fn minutes_to_milliseconds(input: String) -> String {
    let minutes = value_to_int(&input, 0, "minutes_to_milliseconds");
    let millis = Duration::from_secs((minutes as u64) * 60).as_millis();
    millis.to_string()
}

#[cfg(test)]
mod tests {
    // Import the functions from the parent module (the file scope).
    use super::*;

    #[test]
    fn test_hours_to_milliseconds() {
        assert_eq!("0", hours_to_milliseconds("".to_string()));
        assert_eq!("0", hours_to_milliseconds(" ".to_string()));
        assert_eq!("0", hours_to_milliseconds("0".to_string()));
        assert_eq!("442800000", hours_to_milliseconds("123".to_string()));
        assert_eq!("442800000", hours_to_milliseconds(" 123 ".to_string()));
        assert_eq!("0", hours_to_milliseconds("not_a_number".to_string()));
    }

    #[test]
    fn test_minutes_to_milliseconds() {
        assert_eq!("0", minutes_to_milliseconds("".to_string()));
        assert_eq!("0", minutes_to_milliseconds(" ".to_string()));
        assert_eq!("0", minutes_to_milliseconds("0".to_string()));
        assert_eq!("7380000", minutes_to_milliseconds("123".to_string()));
        assert_eq!("7380000", minutes_to_milliseconds(" 123 ".to_string()));
        assert_eq!("0", minutes_to_milliseconds("not_a_number".to_string()));
    }
}
