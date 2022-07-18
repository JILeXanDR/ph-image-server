use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::traffic_channel::TrafficChannel::Unknown;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TrafficChannel {
    Unknown = 0, // 0
    Our = 1,     // 1
    Feed = 2,    // 2
}

impl Default for TrafficChannel {
    fn default() -> Self {
        Unknown
    }
}

impl FromStr for TrafficChannel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UnknownTrafficSource" => Ok(TrafficChannel::Unknown),
            "Our" => Ok(TrafficChannel::Our),
            "Feed" => Ok(TrafficChannel::Feed),
            _ => Ok(TrafficChannel::Unknown),
        }
    }
}

impl Display for TrafficChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unknown => write!(f, "UnknownTrafficSource"),
            TrafficChannel::Our => write!(f, "Our"),
            TrafficChannel::Feed => write!(f, "Feed"),
        }
    }
}

// UnknownTrafficSourceOurFeed
