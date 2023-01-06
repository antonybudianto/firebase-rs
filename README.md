# firebase-jwt-rs

Firebase JWT decoding utility for Rust.

## What is this?

If you have a Rust server and plan to have API for your verifying your firebase users' [client token](https://firebase.google.com/docs/auth/admin/verify-id-tokens#retrieve_id_tokens_on_clients), then it's just for you!

It only contains small utility to decode your [client token](https://firebase.google.com/docs/auth/admin/verify-id-tokens#retrieve_id_tokens_on_clients).

**Read more:**

https://firebase.google.com/docs/auth/admin/verify-id-tokens#verify_id_tokens_using_a_third-party_jwt_library

## Flow

1. In your frontend, you can get your uid + client-token by using Firebase Client SDK.

    Sample:
    https://firebase.google.com/docs/auth/admin/verify-id-tokens#retrieve_id_tokens_on_clients

2. Then your frontend can hit the backend Rust API and send over uid+client-token to the HTTP Header/POST body (up to you), which will be passed to `verify_token(uid, client_token)` util.

3. Add `FIREBASE_PROJECT_ID` environment variable to your backend on local and production server

4. Backend Rust will verify and send back the JWT claim response (including extracted user's data, and verified user_id) back as json

## How to use

```rs
use firebase_jwt_rs::*;
use jsonwebtoken::TokenData;
use std::error::Error;

let uid = "your-user-uid";
let client_token = "your-client-token";

let result: Result<TokenData<Claims>, Box<dyn Error>> = verify_token(uid, client_token).await;

match result {
  Ok(res: TokenData<Claims>) => {
    let text: String = serde_json::to_string(&res.claims).unwrap();
    println!("result:{text}");
  }
  Err(e) => {
    println!("err:{e}");
  }
}
```

## License

MIT
