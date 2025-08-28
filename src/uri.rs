use std::{hash::Hash, ops::Deref, str::FromStr};

use serde::{de::Error, Deserialize, Serialize};

/// Newtype struct around `url::Url` with serialization implementations that properly encode brackets for LSP compatibility.
#[derive(Debug, Clone)]
pub struct Uri(url::Url);

impl Serialize for Uri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Encode brackets for LSP compatibility
        let url_str = self.0.as_str();
        if url_str.contains('[') || url_str.contains(']') {
            // Fast bracket encoding - count brackets first for precise capacity
            let bracket_count = url_str.chars().filter(|&c| c == '[' || c == ']').count();
            let mut encoded = String::with_capacity(url_str.len() + bracket_count * 2);

            for c in url_str.chars() {
                match c {
                    '[' => encoded.push_str("%5B"),
                    ']' => encoded.push_str("%5D"),
                    _ => encoded.push(c),
                }
            }
            encoded.serialize(serializer)
        } else {
            url_str.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Uri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        url::Url::parse(&string)
            .map(Uri)
            .map_err(|error| Error::custom(error.to_string()))
    }
}

impl Ord for Uri {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for Uri {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Uri {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::parse(s).map(Self)
    }
}

impl Deref for Uri {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Uri {}

impl Hash for Uri {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}
