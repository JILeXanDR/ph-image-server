extern crate base64;
extern crate serde_json;

use std::error::Error;

use base64::{decode, encode};
use serde::{Deserialize, Serialize};

use super::{advertisement::Advertisement, traffic_channel::TrafficChannel};

// JSON = "advType":0,"trafficChannel":0
// JSON is "advType":"Push","trafficChannel":"Feed"

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct IconRequest {
    #[serde(default)]
    pub icon: String,
    #[serde(default, rename(serialize = "uid", deserialize = "uid"))]
    pub web_user_id: i64,
    #[serde(default, rename(serialize = "cid", deserialize = "cid"))]
    pub campaign_id: i64,
    #[serde(default)]
    os: i64,
    // TODO: implement OS type like in Go?
    #[serde(default)]
    browser: i64,
    // TODO: implement Browser type like in Go?
    #[serde(default, rename(serialize = "country", deserialize = "country"))]
    pub country_id: i64,
    #[serde(default)]
    pub operator: i64,
    #[serde(default, rename(serialize = "subAcc", deserialize = "subAcc"))]
    pub sub_acc_id: i64,
    #[serde(default, rename(serialize = "subId", deserialize = "subId"))]
    subscription_id: i64,
    #[serde(default, rename(serialize = "advType", deserialize = "advType"))]
    pub advertisement_type: Advertisement,
    #[serde(
        default,
        rename(serialize = "trafficChannel", deserialize = "trafficChannel")
    )]
    traffic_channel: TrafficChannel,
}

impl IconRequest {
    /// Convert request into base64 format.
    pub fn to_base64(&self) -> Result<String, Box<dyn Error>> {
        let json = serde_json::to_string(self)?;
        println!("JSON is {:#}", json);
        let base64 = encode(json);
        Ok(base64)
    }

    /// Create icon request from base64 string.
    pub fn from_base64(base64_str: &String) -> Result<IconRequest, Box<dyn Error>> {
        let raw_json =
            decode(base64_str).or_else(|err| Err(format!("Decode base64: {:?}", err)))?;

        let str = String::from_utf8(raw_json)
            .or_else(|err| Err(format!("Get UTF8 string: {:?}", err)))?;

        println!("JSON = {:#}", str);

        let icon_request: IconRequest =
            serde_json::from_str(&str).or_else(|err| Err(format!("Decode JSON: {:?}", err)))?;

        Ok(icon_request)
    }
}

impl Default for IconRequest {
    fn default() -> Self {
        IconRequest {
            icon: String::from(""),
            web_user_id: 0,
            campaign_id: 0,
            os: 0,
            browser: 0,
            country_id: 0,
            operator: 0,
            sub_acc_id: 0,
            subscription_id: 0,
            advertisement_type: Advertisement::Push,
            traffic_channel: TrafficChannel::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_result_the_same_as_in_go() {
        let json_str = r#"{"icon":"dude.jpg","uid":1,"cid":2,"os":3,"browser":4,"country":5,"operator":6,"subAcc":7,"subId":8,"advType":1,"trafficChannel":0}"#;
        let encoded = "eyJpY29uIjoiZHVkZS5qcGciLCJ1aWQiOjEsImNpZCI6Miwib3MiOjMsImJyb3dzZXIiOjQsImNvdW50cnkiOjUsIm9wZXJhdG9yIjo2LCJzdWJBY2MiOjcsInN1YklkIjo4LCJhZHZUeXBlIjoxLCJ0cmFmZmljQ2hhbm5lbCI6MH0=";
    }

    #[test]
    #[should_panic]
    fn build_icon_request_from_invalid_base64() {
        IconRequest::from_base64(&String::from("")).unwrap();
    }

    #[test]
    fn build_icon_request_from_valid_base64_and_empty_struct() {
        // It's base64 format of Go code.
        let base64 = String::from("eyJpY29uIjoiIiwidWlkIjowLCJjaWQiOjAsIm9zIjowLCJicm93c2VyIjowLCJjb3VudHJ5IjowLCJvcGVyYXRvciI6MCwic3ViQWNjIjowLCJzdWJJZCI6MCwiYWR2VHlwZSI6MCwidHJhZmZpY0NoYW5uZWwiOjB9");
        let ir = IconRequest::from_base64(&base64).expect("Failed to get from base64");

        assert_eq!(ir, IconRequest::default());
    }

    #[test]
    fn build_icon_request_from_valid_base64_and_filled_struct() {
        // It's base64 format of Go code.
        let base64 = String::from("eyJpY29uIjoiZHVkZS5qcGciLCJ1aWQiOjEsImNpZCI6Miwib3MiOjMsImJyb3dzZXIiOjQsImNvdW50cnkiOjUsIm9wZXJhdG9yIjo2LCJzdWJBY2MiOjcsInN1YklkIjo4LCJhZHZUeXBlIjoxLCJ0cmFmZmljQ2hhbm5lbCI6MH0=");
        let icon_request = IconRequest::from_base64(&base64).expect("Failed to get from base64");

        assert_eq!(
            icon_request,
            IconRequest {
                icon: String::from("dude.jpg"),
                web_user_id: 1,
                campaign_id: 2,
                os: 3,
                browser: 4,
                country_id: 5,
                operator: 6,
                sub_acc_id: 7,
                subscription_id: 8,
                advertisement_type: Advertisement::InPage,
                traffic_channel: TrafficChannel::Unknown,
            }
        );
    }

    #[test]
    fn convert_to_base64_and_do_back_decoding() {
        let ir = IconRequest {
            icon: String::from("icon.png"),
            web_user_id: 111,
            campaign_id: 222,
            os: 123,
            browser: 12,
            country_id: 213,
            operator: 123,
            sub_acc_id: 23,
            subscription_id: 222222,
            advertisement_type: Advertisement::Push,
            traffic_channel: TrafficChannel::Feed,
        };

        let result = ir.to_base64().expect("Failed to get base64");

        // It's our base64 format.
        assert_eq!(result, "eyJpY29uIjoiaWNvbi5wbmciLCJ1aWQiOjExMSwiY2lkIjoyMjIsIm9zIjoxMjMsImJyb3dzZXIiOjEyLCJjb3VudHJ5IjoyMTMsIm9wZXJhdG9yIjoxMjMsInN1YkFjYyI6MjMsInN1YklkIjoyMjIyMjIsImFkdlR5cGUiOjAsInRyYWZmaWNDaGFubmVsIjoyfQ==");

        let ir1 = IconRequest::from_base64(&result).expect("Failed to convert from base64");

        assert_eq!(ir, ir1);
    }
}
