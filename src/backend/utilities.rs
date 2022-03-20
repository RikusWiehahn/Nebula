use crate::main::*;
use ic_cdk::export::Principal;
use ic_cdk::{api::time, println};
use serde::{Deserialize, Serialize};
extern crate serde_json;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::result::Result;
use std::vec::Vec;
use uuid::{Builder, Variant, Version};
extern crate argon2;
use argon2::Config;

//
//  #    # #    # # #####
//  #    # #    # # #    #
//  #    # #    # # #    #
//  #    # #    # # #    #
//  #    # #    # # #    #
//   ####   ####  # #####

pub async fn generate_uuid() -> Result<String, String> {
    let res = ic_cdk::call(Principal::management_canister(), "raw_rand", ()).await;
    if res.is_err() {
        return Err("Failed to generate UUID".to_string());
    }
    let (bytes,): (Vec<u8>,) = res.unwrap();
    let mut random_bytes: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    for i in 0..16 {
        random_bytes[i] = bytes[i];
    }
    let uuid = Builder::from_bytes(random_bytes)
        .set_variant(Variant::RFC4122)
        .set_version(Version::Random)
        .build();
    Ok(uuid.to_string())
}

//
//       # #    # #####    #    # ###### #   #  ####  ###### #    #
//       # #    #   #      #   #  #       # #  #    # #      ##   #
//       # #    #   #      ####   #####    #   #      #####  # #  #
//       # # ## #   #      #  #   #        #   #  ### #      #  # #
//  #    # ##  ##   #      #   #  #        #   #    # #      #   ##
//   ####  #    #   #      #    # ######   #    ####  ###### #    #

pub async fn init_jwt_key() {
    let uuid_res_1 = generate_uuid().await;
    let uuid_res_2 = generate_uuid().await;
    if uuid_res_1.is_err() || uuid_res_2.is_err() {
        println!("Failed to generate UUID");
        return;
    }
    let uuid_1 = uuid_res_1.unwrap();
    let uuid_2 = uuid_res_2.unwrap();
    let new_jwt_secret: String = uuid_1 + &uuid_2;

    STATE.with(|state: &GlobalState| {
        let mut jwt_secret = state.jwt_secret.borrow_mut();
        if jwt_secret.is_empty() {
            *jwt_secret = new_jwt_secret.clone();
        }
    });
}

//
//   ####  ###### #####         # #    # #####     ####  ######  ####  #####  ###### #####
//  #    # #        #           # #    #   #      #      #      #    # #    # #        #
//  #      #####    #           # #    #   #       ####  #####  #      #    # #####    #
//  #  ### #        #           # # ## #   #           # #      #      #####  #        #
//  #    # #        #      #    # ##  ##   #      #    # #      #    # #   #  #        #
//   ####  ######   #       ####  #    #   #       ####  ######  ####  #    # ######   #

pub fn get_jwt_secret() -> Result<String, String> {
    let mut jwt_secret_opt: Option<String> = None;
    STATE.with(|state: &GlobalState| {
        let jwt_secret = state.jwt_secret.borrow();
        if !jwt_secret.is_empty() {
            jwt_secret_opt = Some(jwt_secret.clone());
        }
    });
    if jwt_secret_opt.is_none() {
        return Err("JWT secret not set".to_string());
    }
    Ok(jwt_secret_opt.unwrap())
}

//
//  #    #   ##    ####  #    #      ##      #####    ##    ####   ####  #    #  ####  #####  #####
//  #    #  #  #  #      #    #     #  #     #    #  #  #  #      #      #    # #    # #    # #    #
//  ###### #    #  ####  ######    #    #    #    # #    #  ####   ####  #    # #    # #    # #    #
//  #    # ######      # #    #    ######    #####  ######      #      # # ## # #    # #####  #    #
//  #    # #    # #    # #    #    #    #    #      #    # #    # #    # ##  ## #    # #   #  #    #
//  #    # #    #  ####  #    #    #    #    #      #    #  ####   ####  #    #  ####  #    # #####

pub async fn hash_password(password: &str) -> Result<String, String> {
    let rand_res = ic_cdk::call(Principal::management_canister(), "raw_rand", ()).await;
    if rand_res.is_err() {
        return Err("Failed to generate UUID".to_string());
    }
    let (bytes,): (Vec<u8>,) = rand_res.unwrap();
    let mut salt: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    for i in 0..16 {
        salt[i] = bytes[i];
    }

    let password = password.as_bytes();
    let config = Config::default();
    let hash = argon2::hash_encoded(password, &salt, &config).unwrap();

    Ok(hash)
}

pub async fn verify_password(password: &str, hash: &str) -> Result<(), String> {
    let matches_res = argon2::verify_encoded(&hash, password.as_bytes());
    if matches_res.is_err() {
        return Err("Password does not match".to_string());
    }
    if matches_res.unwrap() {
        return Ok(());
    }
    return Err("Password does not match".to_string());
}

//
//  ###### #    #  ####   ####  #####  ######         # #    # #####
//  #      ##   # #    # #    # #    # #              # #    #   #
//  #####  # #  # #      #    # #    # #####          # #    #   #
//  #      #  # # #      #    # #    # #              # # ## #   #
//  #      #   ## #    # #    # #    # #         #    # ##  ##   #
//  ###### #    #  ####   ####  #####  ######     ####  #    #   #
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub id: String,
    pub exp: u64,
}

pub fn generate_token_from_id(id: &str) -> Result<String, String> {
    let jwt_secret_res = get_jwt_secret();
    if jwt_secret_res.is_err() {
        return Err("JWT secret not set".to_string());
    }
    let jwt_secret = jwt_secret_res.unwrap();

    let to_encode: Payload = Payload {
        id: id.to_string(),
        exp: (time() + 1_000_000_000 * 60 * 60 * 24 * 7) as u64, // 7 days
    };
    println!("{:?}", to_encode);

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

    let token_str = to_encode.sign_with_key(&key).unwrap();

    Ok(token_str)
}

//
//  #####  ######  ####   ####  #####  ######         # #    # #####
//  #    # #      #    # #    # #    # #              # #    #   #
//  #    # #####  #      #    # #    # #####          # #    #   #
//  #    # #      #      #    # #    # #              # # ## #   #
//  #    # #      #    # #    # #    # #         #    # ##  ##   #
//  #####  ######  ####   ####  #####  ######     ####  #    #   #

pub fn decode_id_from_token(token: &str) -> Result<String, String> {
    let jwt_secret_res = get_jwt_secret();
    if jwt_secret_res.is_err() {
        return Err("JWT secret not set".to_string());
    }
    let jwt_secret = jwt_secret_res.unwrap();

    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();

    let payload_res = token.verify_with_key(&key);
    if payload_res.is_err() {
        return Err("Failed to decode token".to_string());
    }
    let payload: Payload = payload_res.unwrap();
    if payload.id.len() == 0 {
        return Err("Invalid token".to_string());
    }
    if payload.exp < time() as u64 {
        return Err("Token expired".to_string());
    }
    Ok(payload.id)
}