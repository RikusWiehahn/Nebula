use crate::services_telemetry::*;
use crate::types::*;
use ic_cdk::api::time;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::println;
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

//
//   ####  #####   ##   ##### ######
//  #        #    #  #    #   #
//   ####    #   #    #   #   #####
//       #   #   ######   #   #
//  #    #   #   #    #   #   #
//   ####    #   #    #   #   ######

pub type ModelMap = HashMap<String, Model>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct GlobalState {
    pub jwt_secret: RefCell<String>,
    pub auth: RefCell<Authentication>,
    pub last_status_update: RefCell<u64>,
    pub telemetry: RefCell<Telemetry>,
    pub models: RefCell<ModelMap>,
}

thread_local! {
    pub static STATE: GlobalState = GlobalState {
        jwt_secret: RefCell::new(String::new()),
        auth: RefCell::new(Authentication::default()),
        last_status_update: RefCell::new(0),
        telemetry: RefCell::new(Telemetry::default()),
        models: RefCell::new(ModelMap::new()),
    }
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

        let res = storage::stable_save((
            jwt_secret.clone(),
            auth.clone(),
            last_status_update.clone(),
            telemetry.clone(),
            models.clone(),
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
            let (jwt_secret, auth, last_status_update, telemetry, models) = res.unwrap();
            state.jwt_secret.replace(jwt_secret);
            state.auth.replace(auth);
            state.last_status_update.replace(last_status_update);
            state.telemetry.replace(telemetry);
            state.models.replace(models);
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
    let duration: u64 = 1_000_000_000 * 60 * 1; // 1 minutes
    let t = time();
    let prev = STATE.with(|s| s.last_status_update.borrow().clone());

    if t - duration > prev {
        println!("Update telemetry");
        STATE.with(|s| {
            let mut last_status_update = s.last_status_update.borrow_mut();
            *last_status_update = t;
        });
        ic_cdk::block_on(auto_update_telemetry());
    }
}
