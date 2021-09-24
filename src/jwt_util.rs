
use jsonwebtoken::{decode, Validation, DecodingKey, decode_header, Algorithm, EncodingKey, encode,Header};
use chrono::{Duration, TimeZone, Utc, DateTime, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    #[serde(with = "jwt_numeric_date")]
    iat: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    exp: DateTime<Utc>,
}

impl Claims {
    pub fn new(sub: String, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = iat.date().and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
        let exp = exp.date().and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);
        Self { sub, iat, exp }
    }
}
pub mod jwt_numeric_date {

    use serde::{self, Deserialize, Deserializer, Serializer,Serialize};

    pub fn parse_jwt(token: &str) {

    }
}




