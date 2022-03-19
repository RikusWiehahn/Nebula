use crate::types::*;
use crate::utilities::*;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::*;
use std::cell::RefCell;

//
//   ####  #####   ##   ##### ######
//  #        #    #  #    #   #
//   ####    #   #    #   #   #####
//       #   #   ######   #   #
//  #    #   #   #    #   #   #
//   ####    #   #    #   #   ######

pub type JwtSecret = String;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct GlobalState {
    pub jwt_secret: RefCell<JwtSecret>,
    pub auth: RefCell<Authentication>,
}

thread_local! {
    pub static STATE: GlobalState = GlobalState {
        jwt_secret: RefCell::new(String::new()),
        auth: RefCell::new(Authentication::default()),
    }
}

//
//  # #    # # #####
//  # ##   # #   #
//  # # #  # #   #
//  # #  # # #   #
//  # #   ## #   #
//  # #    # #   #

#[init]
fn init() {
    unsafe {
        ic_cdk::block_on(init_jwt_key());
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

        let res = storage::stable_save((jwt_secret.clone(), auth.clone()));
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
            let (jwt_secret, auth) = res.unwrap();
            state.jwt_secret.replace(jwt_secret);
            state.auth.replace(auth);
        }
    });
}
