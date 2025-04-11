use uuid::Uuid;
use std::fmt;
use std::str::FromStr;
use crate::core::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GUID(Uuid);

impl GUID {
    /// Create a new random GUID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a GUID from a string
    pub fn from_string(s: &str) -> Result<Self, Error> {
        match Uuid::from_str(s) {
            Ok(uuid) => Ok(Self(uuid)),
            Err(_) => Err(Error::Config(format!("Invalid GUID format: {}", s).into())),
        }
    }

    /// Get the GUID as a string with braces
    pub fn to_string_braced(&self) -> String {
        format!("{{{}}}", self.0.hyphenated())
    }

    /// Get the GUID as a string without braces
    pub fn to_string_simple(&self) -> String {
        self.0.hyphenated().to_string()
    }

    /// Check if the GUID is nil (all zeros)
    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }

    /// Get a nil GUID (all zeros)
    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for GUID {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}", self.0.hyphenated())
    }
}

impl FromStr for GUID {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid_creation() {
        let guid = GUID::new();
        assert!(!guid.is_nil());
    }

    #[test]
    fn test_guid_from_string() {
        let guid_str = "{9D499A2C-72B6-40B0-8C8C-995831B10BB4}";
        let guid = GUID::from_string(guid_str).unwrap();
        assert_eq!(guid.to_string(), guid_str);
    }

    #[test]
    fn test_guid_nil() {
        let guid = GUID::nil();
        assert!(guid.is_nil());
        assert_eq!(guid.to_string(), "{00000000-0000-0000-0000-000000000000}");
    }
} 
