use chrono::{Duration, Utc};
use jsonwebtoken::{/*decode,*/ encode, /*DecodingKey,*/ EncodingKey, Header/*, Validation*/};
use serde::{Deserialize, Serialize};
use tide::log;

const SECRET: &[u8] = b"I'm very secret";

#[derive(Deserialize, Serialize, Debug)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
    test: String,
}

pub fn get_token(public_key: &str) -> String {
    log::info!("Getting token for: {}", public_key);

    let iat = Utc::now().timestamp();
    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(30))
        .expect("invalid_timestamp")
        .timestamp();
    let claims = Claims {
        sub: "someone@somewhere.com".to_owned(),
        iat: iat,
        exp: exp,
        test: format!("Some stuff for {}", public_key)
    };

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)) {
        Ok(t) => t,
        Err(_) => panic!() 
    }
}
