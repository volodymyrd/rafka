use crate::common::config::types::password::Password;
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::Display;
use std::str::FromStr;

/// A trait for any type that can be parsed from a string.
pub trait ConfigValueType: Sized {
    fn parse(key: &str, value_str: &str) -> Result<Self, ConfigError>;
}

fn parse_config_value<T>(key: &str, s: &str) -> Result<T, ConfigError>
where
    T: ConfigValueType + Copy + FromStr + 'static, // The type must be parsable from a string.
    <T as FromStr>::Err: Display,                  // The error it produces must be printable
{
    s.trim()
        .to_lowercase()
        .parse()
        .map_err(|e: <T as FromStr>::Err| ConfigError::InvalidValue {
            key: key.to_string(),
            message: e.to_string(),
        })
}

impl ConfigValueType for bool {
    fn parse(key: &str, s: &str) -> Result<Self, ConfigError> {
        parse_config_value(key, s)
    }
}

impl ConfigValueType for i32 {
    fn parse(key: &str, s: &str) -> Result<Self, ConfigError> {
        parse_config_value(key, s)
    }
}

impl ConfigValueType for i64 {
    fn parse(key: &str, s: &str) -> Result<Self, ConfigError> {
        parse_config_value(key, s)
    }
}

impl ConfigValueType for f32 {
    fn parse(key: &str, s: &str) -> Result<Self, ConfigError> {
        parse_config_value(key, s)
    }
}

impl ConfigValueType for f64 {
    fn parse(key: &str, s: &str) -> Result<Self, ConfigError> {
        parse_config_value(key, s)
    }
}

impl ConfigValueType for String {
    fn parse(_key: &str, s: &str) -> Result<Self, ConfigError> {
        Ok(s.trim().to_string())
    }
}

impl ConfigValueType for Vec<String> {
    fn parse(_key: &str, s: &str) -> Result<Self, ConfigError> {
        Ok(s.trim()
            .split(',')
            .map(|item| item.trim().to_string())
            .collect())
    }
}

impl ConfigValueType for Password {
    fn parse(_key: &str, s: &str) -> Result<Self, ConfigError> {
        Ok(Password::new(s.trim().to_string()))
    }
}

/// The mutable builder for creating a `ConfigDef`.
#[derive(Default, Debug)]
pub struct ConfigDefBuilder {
    def: ConfigDef,
    current_key: Option<ConfigKey>,
}

impl ConfigDefBuilder {
    fn commit_current_key(&mut self) {
        if let Some(key) = self.current_key.take() {
            if self.def.config_keys.contains_key(key.name) {
                panic!("Configuration key {} is defined twice", key.name);
            }

            if let Some(group_name) = key.group.as_ref() {
                let group_string = group_name.to_string();
                if !self.def.groups.contains(&group_string) {
                    self.def.groups.push_back(group_string);
                }
            }

            self.def.config_keys.insert(key.name, key);
        }
    }

    /// Adds a new key. Commits the previous one first.
    pub fn new(mut self, name: &'static str) -> Self {
        self.commit_current_key();
        self.current_key = Some(ConfigKey::new(name));
        self
    }

    fn current_key_mut(&mut self) -> &mut ConfigKey {
        self.current_key
            .as_mut()
            .expect("Builder logic error: current_key should never be None here")
    }

    pub fn importance(mut self, value: Importance) -> Self {
        self.current_key_mut().importance = Some(value);
        self
    }

    pub fn default_value(mut self, value: &'static str) -> Self {
        self.current_key_mut().default_value = Some(value.to_string());
        self
    }

    pub fn documentation(mut self, value: &'static str) -> Self {
        self.current_key_mut().documentation = Some(value.to_string());
        self
    }

    /// Commits the final key and builds the immutable `ConfigDef`.
    pub fn build(mut self) -> ConfigDef {
        self.commit_current_key();
        self.def
    }
}

/// The importance level for a configuration.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Importance {
    HIGH,
    MEDIUM,
    LOW,
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Missing required configuration key: '{0}'")]
    MissingKey(String),

    #[error("Failed to parse key '{key}': {message}")]
    InvalidValue { key: String, message: String },

    #[error("Validation failed for key '{key}': {message}")]
    ValidationFailed { key: String, message: String },
}

type Validator = fn(key: &str, value: &str) -> Result<(), ConfigError>;

/// A trait for any struct that can be constructed from a parsed configuration.
pub trait FromConfigDef: Sized {
    fn from_props(props: &HashMap<String, String>, def: &ConfigDef) -> Result<Self, ConfigError>;
}

#[derive(Debug)]
pub struct ConfigKey {
    pub name: &'static str,
    pub documentation: Option<String>,
    pub default_value: Option<String>,
    pub validator: Option<Validator>,
    pub importance: Option<Importance>,
    pub group: Option<String>,
    pub order_in_group: Option<usize>,
    // pub width: Width,
    pub display_name: Option<String>,
    pub dependents: Vec<String>,
    // pub recommender: Recommender,
    pub internal_config: bool,
    pub alternative_string: Option<String>,
}

impl ConfigKey {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            documentation: None,
            default_value: None,
            validator: None,
            importance: None,
            group: None,
            order_in_group: None,
            // width: Width::NONE,
            display_name: None,
            dependents: Vec::new(),
            internal_config: false,
            alternative_string: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct ConfigDef {
    config_keys: IndexMap<&'static str, ConfigKey>,
    groups: LinkedList<String>,
    _configs_with_no_parent: HashSet<String>,
}

impl ConfigDef {
    pub fn builder() -> ConfigDefBuilder {
        ConfigDefBuilder::default()
    }

    pub fn find_key(&self, name: &str) -> Option<&ConfigKey> {
        self.config_keys.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::config::types::password::Password;
    use std::collections::HashMap;

    #[test]
    fn test_basic_types() {
        // Define the strongly typed final struct. This IS the schema.
        #[derive(Debug, PartialEq)]
        struct TestConfig {
            a: i32,
            b: i64,
            c: String,
            d: Vec<String>,
            e: f64,
            f: String,
            g: bool,
            h: bool,
            i: bool,
            j: Password,
        }

        impl FromConfigDef for TestConfig {
            fn from_props(
                props: &HashMap<String, String>,
                def: &ConfigDef,
            ) -> Result<Self, ConfigError> {
                let get_value = |name: &str| -> Result<_, ConfigError> {
                    let meta = def
                        .find_key(name)
                        .ok_or_else(|| ConfigError::MissingKey(name.to_string()))?;
                    let val_str = props
                        .get(name)
                        .map(|s| s)
                        .or(meta.default_value.as_ref())
                        .ok_or_else(|| ConfigError::MissingKey(name.to_string()))?;

                    if let Some(validator) = meta.validator {
                        validator(name, val_str)?;
                    }
                    Ok(val_str)
                };

                Ok(TestConfig {
                    a: i32::parse("a", get_value("a")?)?,
                    b: i64::parse("b", get_value("b")?)?,
                    c: String::parse("c", get_value("c")?)?,
                    d: Vec::parse("d", get_value("d")?)?,
                    e: f64::parse("e", get_value("e")?)?,
                    f: String::parse("f", get_value("f")?)?,
                    g: bool::parse("g", get_value("g")?)?,
                    h: bool::parse("h", get_value("h")?)?,
                    i: bool::parse("i", get_value("i")?)?,
                    j: Password::parse("j", get_value("j")?)?,
                })
            }
        }

        // Arrange: Define the configuration schema.
        let def = ConfigDef::builder()
            .new("a")
            .default_value("5")
            // Range.between(0, 14)
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("b")
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("c")
            .default_value("hello")
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("d")
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("e")
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("f")
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("g")
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("h")
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("i")
            .importance(Importance::HIGH)
            .documentation("docs")
            .new("j")
            .importance(Importance::HIGH)
            .documentation("docs")
            .build();

        // Arrange: Set up the raw string properties.
        let mut props = HashMap::new();
        props.insert("a".to_string(), "1   ".to_string());
        props.insert("b".to_string(), "2".to_string());
        // "c" is omitted to test the default value.
        props.insert("d".to_string(), " a , b, c".to_string());
        props.insert("e".to_string(), "42.5".to_string());
        props.insert("f".to_string(), "java.lang.String".to_string());
        props.insert("g".to_string(), "true".to_string());
        props.insert("h".to_string(), "FalSE".to_string());
        props.insert("i".to_string(), "TRUE".to_string());
        props.insert("j".to_string(), "password".to_string());

        // Act: Parse the properties into the strongly typed struct.
        let config = TestConfig::from_props(&props, &def).unwrap();

        // Assert: Check the final parsed values.
        assert_eq!(config.a, 1);
        assert_eq!(config.b, 2);
        assert_eq!(config.c, "hello"); // Correctly uses the default
        assert_eq!(config.d, vec!["a", "b", "c"]);
        assert_eq!(config.e, 42.5);
        assert_eq!(config.f, "java.lang.String");
        assert_eq!(config.g, true);
        assert_eq!(config.h, false);
        assert_eq!(config.i, true);
        assert_eq!(config.j, Password::new("password".to_string()));
        assert_eq!(config.j.to_string(), "[hidden]");
    }
}
