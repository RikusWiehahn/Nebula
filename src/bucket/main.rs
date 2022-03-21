use crate::types::*;
use crate::utilities::accept_cycles;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use ic_cdk_macros::*;
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

pub type ModelInstanceMap = HashMap<String, ModelInstance>;
pub type DataFieldTypeMap = HashMap<String, ModelDataFieldType>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct GlobalState {
    pub admin_canister_id: RefCell<String>,
    pub model_name: RefCell<String>,
    pub model_data_fields: RefCell<DataFieldTypeMap>,
    pub instances: RefCell<ModelInstanceMap>,
}

thread_local! {
    pub static STATE: GlobalState = GlobalState {
        admin_canister_id: RefCell::new(String::new()),
        model_name: RefCell::new(String::new()),
        model_data_fields: RefCell::new(DataFieldTypeMap::new()),
        instances: RefCell::new(ModelInstanceMap::new()),
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
        let admin_canister_id = state.admin_canister_id.borrow();
        let model_name = state.model_name.borrow();
        let model_data_fields = state.model_data_fields.borrow();
        let instances = state.instances.borrow();

        let res = storage::stable_save((
            admin_canister_id.clone(),
            model_name.clone(),
            model_data_fields.clone(),
            instances.clone(),
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
