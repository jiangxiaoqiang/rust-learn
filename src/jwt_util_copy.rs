
use jsonwebtoken::{decode, Validation, DecodingKey, decode_header, Algorithm, EncodingKey, encode,Header};
use chrono::{Duration, TimeZone, Utc, DateTime, Timelike};
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

const EXPECTED_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJDdXN0b20gRGF0ZVRpbWUgc2VyL2RlIiwiaWF0IjowLCJleHAiOjMyNTAzNjgwMDAwfQ.RTgha0S53MjPC2pMA4e2oMzaBxSY3DMjiYR2qFfV55A";
const SECRET: &str = "kjiwtuEQjdh223Hagkweptweijgiwlkfowewewgewgewgewdfhrhrtytuwrw5234354772hrhrhtrhrtj456547joqwe43534ddw2324323yy4u64kj1312o4j1";


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    sub: String,
    #[serde(with = "jwt_numeric_date")]
    iat: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    exp: DateTime<Utc>,
}

impl Claims {
    /// If a token should always be equal to its representation after serializing and deserializing
    /// again, this function must be used for construction. `DateTime` contains a microsecond field
    /// but JWT timestamps are defined as UNIX timestamps (seconds). This function normalizes the
    /// timestamps.
    pub fn new(sub: String, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        // normalize the timestamps by stripping of microseconds
        let iat = iat.date().and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
        let exp = exp.date().and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);
        Self { sub, iat, exp }
    }
}
pub mod jwt_numeric_date {

    use jsonwebtoken::{decode, Validation, DecodingKey, decode_header, Algorithm, EncodingKey, encode,Header};
    use chrono::{Duration, TimeZone, Utc, DateTime, Timelike};
    use rocket::form::FromForm;
    use serde::{self, Deserialize, Deserializer, Serializer,Serialize};
    use crate::jwt_util::{Claims, SECRET};

    pub fn parse_jwt(token: &str) {
        let iat = Utc::now();
        let exp = iat + Duration::days(1);
        let sub = "Custom DateTime ser/de".to_string();

        let claims = Claims::new(sub.clone(), iat, exp);

        let token =
            encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref()))
                .expect("Failed to encode claims");

        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
            .expect("Failed to decode token")
            .claims;

        assert_eq!(claims, decoded);
    }
}




