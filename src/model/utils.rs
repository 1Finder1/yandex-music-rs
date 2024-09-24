use serde::de::{self, Deserializer, Error};
use serde::Deserialize;

pub fn string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Deserialize::deserialize(deserializer)?;
    println!("{}", value);
    match value {
        serde_json::Value::Number(num) => Ok(num.as_i64().unwrap() as i32),
        serde_json::Value::String(s) => {
            s.parse::<i32>().map_err(de::Error::custom)
        }
        _ => Err(D::Error::custom("expected number or string")),
    }
}

// pub fn string_to_i32_options<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
// where
//     D: Deserializer<'de>
// {
//     Ok(match string_to_i32(deserializer){
//         Ok(res) => Some(res),
//         Err(_) => None
//     })
// }

pub fn number_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(num) => Ok(num.to_string()),
        serde_json::Value::String(s) => Ok(s),
        _ => Err(D::Error::custom("expected number or string")),
    }
}
