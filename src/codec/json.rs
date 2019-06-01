extern crate rustc_serialize;
use self::rustc_serialize::json::Object;
use rustc_serialize::json::{Json, ToJson};

pub fn decode(s: &str) -> Json {
    Json::from_str(s).expect("json parse error")
}

pub fn encode(t: Object) -> Json {
    Json::encode(&t).expect("json encode error")
}
