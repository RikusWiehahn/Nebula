use crate::services::auto_update_telemetry;
use crate::types::*;
use crate::utilities::{accept_cycles, drain_cycles};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk::api::time;
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

#[update]
async fn wallet_drain() {
    let _ = drain_cycles().await;
}

//
//   ####  #####   ##   ##### ######
//  #        #    #  #    #   #
//   ####    #   #    #   #   #####
//       #   #   ######   #   #
//  #    #   #   #    #   #   #
//   ####    #   #    #   #   ######

pub type ModelInstanceMap = HashMap<String, ModelInstance>;
pub type DataFieldTypeMap = HashMap<String, ModelDataFieldType>;

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct GlobalState {
    pub admin_canister_id: RefCell<String>,
    pub model_name: RefCell<String>,
    pub model_data_fields: RefCell<DataFieldTypeMap>,
    pub instances: RefCell<ModelInstanceMap>,
    pub last_status_update: RefCell<u64>,
    pub telemetry: RefCell<SubCanisterTelemetry>,
}

thread_local! {
    pub static STATE: GlobalState = GlobalState {
        admin_canister_id: RefCell::new("".to_string()),
        model_name: RefCell::new(String::new()),
        model_data_fields: RefCell::new(DataFieldTypeMap::new()),
        instances: RefCell::new(ModelInstanceMap::new()),
        last_status_update: RefCell::new(0),
        telemetry: RefCell::new(SubCanisterTelemetry::default()),
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
        let admin_canister_id = state.admin_canister_id.borrow();
        let model_name = state.model_name.borrow();
        let model_data_fields = state.model_data_fields.borrow();
        let instances = state.instances.borrow();
        let last_status_update = state.last_status_update.borrow();

        let res = storage::stable_save((
            admin_canister_id.clone(),
            model_name.clone(),
            model_data_fields.clone(),
            instances.clone(),
            last_status_update.clone(),
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
            let (admin_canister_id, model_name, model_data_fields, instances) = res.unwrap();
            state.admin_canister_id.replace(admin_canister_id);
            state.model_name.replace(model_name);
            state.model_data_fields.replace(model_data_fields);
            state.instances.replace(instances);
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

    if t - duration > prev {
        let mut is_activated: bool = false;
        STATE.with(|s| {
            ic_cdk::println!("Update sub telemetry");
            let mut last_status_update = s.last_status_update.borrow_mut();
            *last_status_update = t;
            
            let admin_can_id = s.admin_canister_id.borrow();
            if *admin_can_id != "" {
                is_activated = true;
            }
        });
        if is_activated {
            ic_cdk::println!("sub telemetry..");
            ic_cdk::block_on(auto_update_telemetry());
        }
    }
}