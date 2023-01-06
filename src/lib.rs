use std::{env, error::Error};

use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, TokenData, Validation};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct FirebaseClaims {
    sign_in_provider: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    sub: String,
    name: String,
    email: String,
    email_verified: bool,
    user_id: String,
    picture: String,
    firebase: FirebaseClaims,
    exp: u64,
    iat: u64,
}

async fn get_pbkey() -> Result<Map<String, Value>, Box<dyn Error>> {
    let url = format!(
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com",
    );
    let url = Url::parse(&*url)?;
    let res = reqwest::get(url).await?.json().await?;
    Ok(res)
}

/// Verify Firebase client token by passing the uid and client token.
///
/// Prerequisites:
/// - Please add env variable "FIREBASE_PROJECT_ID=your-firebase-project-id"
///
/// # Examples
///
/// ```
/// use firebase_jwt_rs::*;
///
///
/// async fn fetch_token() {
///     let uid = "your-uid";
///     let client_token = "your-client-token";
///     // Get your uid and client token from official Firebase Client SDK:
///     // https://firebase.google.com/docs/auth/admin/verify-id-tokens#retrieve_id_tokens_on_clients
///     let result = verify_token(uid, client_token).await;
///
///     match result {
///         Ok(res) => {
///             let text: String = serde_json::to_string(&res.claims).unwrap();
///             println!("result:{text}");
///         }
///         Err(e) => {
///             println!("err:{e}");
///         }
///     }
/// }
/// ```
pub async fn verify_token(
    uid: &str,
    client_token: &str,
) -> Result<TokenData<Claims>, Box<dyn Error>> {
    let res = get_pbkey().await?;
    let ky = res.keys().nth(1).unwrap(); // we just need to pick one of the provided public key from firebase
    let ky = res[ky].as_str().unwrap();
    let pub_key = ky.as_bytes();

    let mut validation = Validation::new(Algorithm::RS256);

    let fb_project_id = env::var("FIREBASE_PROJECT_ID")?;
    let issuer_str = format!("https://securetoken.google.com/{fb_project_id}");
    validation.sub = Some(uid.to_string());
    validation.set_audience(&[fb_project_id]);
    validation.set_issuer(&[issuer_str]);

    let token_data: Result<TokenData<Claims>, Box<dyn Error>> = match decode::<Claims>(
        &client_token,
        &DecodingKey::from_rsa_pem(pub_key).unwrap(),
        &validation,
    ) {
        Ok(c) => Ok(c),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => Err("invalid-token")?,
            ErrorKind::InvalidIssuer => Err("invalid-issuer")?,
            ErrorKind::InvalidSubject => Err("invalid-subject")?,
            ErrorKind::InvalidAlgorithm => Err("invalid-alg")?,
            ErrorKind::InvalidAudience => Err("invalid-aud")?,
            ErrorKind::InvalidEcdsaKey => Err("invalid-ecdsa-key")?,
            ErrorKind::InvalidAlgorithmName => Err("invalid-alg-name")?,
            ErrorKind::InvalidKeyFormat => Err("invalid-key-format")?,
            ErrorKind::ExpiredSignature => Err("expired-signature")?,
            ErrorKind::ImmatureSignature => Err("immature-signature")?,
            ErrorKind::RsaFailedSigning => Err("rsa-failed-signing")?,
            _ => Err("other-errors")?,
        },
    };

    return token_data;
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use jsonwebtoken::TokenData;

    use crate::{verify_token, Claims};

    #[tokio::test]
    async fn e2e_fetch() {
        let uid = "";
        let client_token = "";
        let result: Result<TokenData<Claims>, Box<dyn Error>> =
            verify_token(uid, client_token).await;
        let mut success = false;
        match result {
            Ok(res) => {
                let text: String = serde_json::to_string(&res.claims).unwrap();
                success = true;
                println!("result:{text}");
            }
            Err(e) => {
                println!("err:{e}");
            }
        }
        assert_eq!(success, false)
    }
}
