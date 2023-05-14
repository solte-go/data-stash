use std::fmt::Debug;
use super::super::ClipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use derive_more::From;

#[derive(Debug, Clone, Deserialize, Serialize, From)]
pub struct Shortcode(String);

impl Shortcode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars = [
            'a', 'b', 'c', 'd', '1', '2', '3', '4',
        ];
        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values")
            );
        }
        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for Shortcode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Shortcode> for String {
    fn from(shortcode: Shortcode) -> Self {
        shortcode.0
    }
}

impl From<&str> for Shortcode {
    fn from(shortcode: &str) -> Self {
        Shortcode(shortcode.to_owned())
    }
}

impl FromStr for Shortcode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}