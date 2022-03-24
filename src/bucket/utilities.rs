use ic_cdk::{export::Principal, api::call::RejectionCode};
extern crate serde_json;
use std::result::Result;
use std::vec::Vec;
use uuid::{Builder, Variant, Version};

use crate::{helpers::caller_is_admin, types::{ModelInstanceResponse, BasicResponse}};

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
//    ##    ####   ####  ###### #####  #####     ####  #   #  ####  #      ######  ####
//   #  #  #    # #    # #      #    #   #      #    #  # #  #    # #      #      #
//  #    # #      #      #####  #    #   #      #        #   #      #      #####   ####
//  ###### #      #      #      #####    #      #        #   #      #      #           #
//  #    # #    # #    # #      #        #      #    #   #   #    # #      #      #    #
//  #    #  ####   ####  ###### #        #       ####    #    ####  ###### ######  ####

pub fn accept_cycles() -> u64 {
    let cycles_available = ic_cdk::api::call::msg_cycles_available();
    let cycles_accepted = ic_cdk::api::call::msg_cycles_accept(cycles_available);

    return cycles_accepted;
}

//
//  #####  #####    ##   # #    #     ####  #   #  ####  #      ######  ####
//  #    # #    #  #  #  # ##   #    #    #  # #  #    # #      #      #
//  #    # #    # #    # # # #  #    #        #   #      #      #####   ####
//  #    # #####  ###### # #  # #    #        #   #      #      #           #
//  #    # #   #  #    # # #   ##    #    #   #   #    # #      #      #    #
//  #####  #    # #    # # #    #     ####    #    ####  ###### ######  ####

pub async fn drain_cycles() -> u64 {
    println!("Drain cycles");
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        ic_cdk::println!("{}", is_admin_res.unwrap_err());
        return 0;
    }

    let destination = ic_cdk::caller();
    let balance = ic_cdk::api::canister_balance();

    let call_res: Result<((),), (RejectionCode, String)> =
        ic_cdk::call(destination, "wallet_receive", (balance - 1_000_000_000_000,)).await;
    if call_res.is_err() {
        ic_cdk::println!("{}", call_res.err().unwrap().1);
        return 0;
    }

    return balance;
}
