use serde_repr::*;

use crate::models::advertisement::Advertisement::Push;

#[derive(Debug, PartialEq, Deserialize_repr, Serialize_repr, Copy, Clone)]
#[repr(u8)]
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[derive(Deserialize, Debug)]
    struct Item {
        push: Advertisement,
        inpage: Advertisement,
        ios: Advertisement,
        unknown: Advertisement,
    }

    #[test]
    fn enum_to_json() {
        let inpage = Advertisement::InPage;

        let str = serde_json::to_string(&inpage).unwrap();

        println!("result is {:#}", str);
    }

    #[test]
    fn json_to_enum() {
        let j = json!({
            "push": 0,
            "inpage": 1,
            "ios": 2,
            "unknown": 3, // TODO: but it panic if it's string or something else then i8.
        });

        let item: Item = serde_json::from_value(j).unwrap();

        println!("result is {:?}", item);
    }
}

// impl FromStr for Advertisement {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str() {
//             "push" => Ok(Advertisement::Push),
//             "inpage" => Ok(Advertisement::InPage),
//             "in-page" => Ok(Advertisement::InPage),
//             "ios" => Ok(Advertisement::IOS),
//             _ => Ok(Unknown),
//         }
//     }
// }

// impl Display for Advertisement {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Advertisement::Push => write!(f, "Push"),
//             Advertisement::InPage => write!(f, "InPage"),
//             Advertisement::IOS => write!(f, "IOS"),
//             Unknown => write!(f, "unknown"),
//         }
//     }
// }
