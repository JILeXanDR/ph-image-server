use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::advertisement::Advertisement::{Push, Unknown};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum Advertisement {
    Push = 0,   // 0
    InPage = 1, // 1
    IOS = 2,    // 2
    Unknown,
}

impl Default for Advertisement {
    fn default() -> Self {
        Push
    }
}

impl FromStr for Advertisement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "push" => Ok(Advertisement::Push),
            "inpage" => Ok(Advertisement::InPage),
            "in-page" => Ok(Advertisement::InPage),
            "ios" => Ok(Advertisement::IOS),
            _ => Ok(Unknown),
        }
    }
}

impl Display for Advertisement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Advertisement::Push => write!(f, "Push"),
            Advertisement::InPage => write!(f, "InPage"),
            Advertisement::IOS => write!(f, "IOS"),
            Unknown => write!(f, "unknown"),
        }
    }
}
