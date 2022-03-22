use crate::services_telemetry::*;
use crate::types::*;
use crate::utilities::accept_cycles;
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::println;
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

//
//  #####    ##   #        ##   #    #  ####  ######
//  #    #  #  #  #       #  #  ##   # #    # #
//  #####  #    # #      #    # # #  # #      #####
//  #    # ###### #      ###### #  # # #      #
//  #    # #    # #      #    # #   ## #    # #
//  #####  #    # ###### #    # #    #  ####  ######

#[update]
fn wallet_receive() {
    accept_cycles();
}

//
//   ####  #####   ##   ##### ######
//  #        #    #  #    #   #
//   ####    #   #    #   #   #####
//       #   #   ######   #   #
//  #    #   #   #    #   #   #
//   ####    #   #    #   #   ######

pub type ModelMap = HashMap<String, Model>;

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct GlobalState {
    pub jwt_secret: RefCell<String>,
    pub auth: RefCell<Authentication>,
    pub last_status_update: RefCell<u64>,
    pub telemetry: RefCell<Telemetry>,
    pub models: RefCell<ModelMap>,
    pub bucket_wasm: RefCell<Vec<u8>>,
}

thread_local! {
    pub static STATE: GlobalState = GlobalState {
        jwt_secret: RefCell::new("".to_string()),
        auth: RefCell::new(Authentication::default()),
        last_status_update: RefCell::new(0),
        telemetry: RefCell::new(Telemetry::default()),
        models: RefCell::new(ModelMap::new()),
        bucket_wasm: RefCell::new(Vec::new()),
    }
}

//
//  #    # ######   ##   #####      ####  # ###### ######
//  #    # #       #  #  #    #    #      #     #  #
//  ###### #####  #    # #    #     ####  #    #   #####
//  #    # #      ###### #####          # #   #    #
//  #    # #      #    # #         #    # #  #     #
//  #    # ###### #    # #          ####  # ###### ######

pub fn get_heap_size() -> u64 {
    let mut heap_size = 0;

    STATE.with(|state| {
        heap_size += bincode::serialized_size(&*state).unwrap();
    });

    return heap_size;
}

//
//  #    # #####   ####  #####    ##   #####  ######
//  #    # #    # #    # #    #  #  #  #    # #
//  #    # #    # #      #    # #    # #    # #####
//  #    # #####  #  ### #####  ###### #    # #
//  #    # #      #    # #   #  #    # #    # #
//   ####  #       ####  #    # #    # #####  ######

#[pre_upgrade]
fn save_data() {
    STATE.with(|state| {
        let jwt_secret = state.jwt_secret.borrow();
        let auth = state.auth.borrow();
        let last_status_update = state.last_status_update.borrow();
        let telemetry = state.telemetry.borrow();
        let models = state.models.borrow();
        let bucket_wasm = state.bucket_wasm.borrow();

        let res = storage::stable_save((
            jwt_secret.clone(),
            auth.clone(),
            last_status_update.clone(),
            telemetry.clone(),
            models.clone(),
            bucket_wasm.clone(),
        ));
        if res.is_err() {
            println!("Error saving data: {:?}", res.err().unwrap());
        }
    });
}

#[post_upgrade]
fn retrieve_data() {
    STATE.with(|state| {
        let res = storage::stable_restore();
        if res.is_err() {
            println!("Error retrieving data: {:?}", res.err().unwrap());
            return;
        } else {
            let (jwt_secret, auth, last_status_update, telemetry, models, bucket_wasm) =
                res.unwrap();
            state.jwt_secret.replace(jwt_secret);
            state.auth.replace(auth);
            state.last_status_update.replace(last_status_update);
            state.telemetry.replace(telemetry);
            state.models.replace(models);
            state.bucket_wasm.replace(bucket_wasm);
        }
    });
}

//
//   ####  #####   ##   ##### #    #  ####
//  #        #    #  #    #   #    # #
//   ####    #   #    #   #   #    #  ####
//       #   #   ######   #   #    #      #
//  #    #   #   #    #   #   #    # #    #
//   ####    #   #    #   #    ####   ####

#[heartbeat]
fn heartbeat() {
    let duration: u64 = 1_000_000_000 * 30; // 30 seconds
    let t = time();
    let prev = STATE.with(|s| s.last_status_update.borrow().clone());

    let mut should_run = false;
    let mut is_activated: bool = false;

    if t - duration > prev {
        ic_cdk::println!("Update main telemetry");
        should_run = true;

        STATE.with(|s| {
            let mut last_status_update = s.last_status_update.borrow_mut();
            *last_status_update = t;

            let auth = s.auth.borrow();
            if auth.password_hash != "" {
                is_activated = true;
            }
        });
    }

    if is_activated && should_run {
        ic_cdk::println!("main telemetry...");
        ic_cdk::block_on(auto_update_telemetry());
    }
}
