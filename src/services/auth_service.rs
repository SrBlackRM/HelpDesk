use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    sub: String,    // ID do usuário
    exp: usize,     // expiração
}

// cria hash bcript para senha
pub fn password_hash(password: &str) -> String{
    hash(password, DEFAULT_COST).unwrap()
}


// verifica senha
pub fn password_verify(password: &str, hash: &str) -> bool{
    verify(password, hash).unwrap()
}


// gerar jwt
pub fn jwt_gen(user_id: i32) -> String {
    dotenv().ok();

    let secret_key: &str = &env::var("SECRET_KEY").expect("Erro ao identificar secret key");
    
    let claims = Claims{
        sub: user_id.to_string(),
        exp: (Utc::now() + Duration::hours(2)).timestamp() as usize
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_bytes())).unwrap()
}