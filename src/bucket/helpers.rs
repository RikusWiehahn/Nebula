use crate::main::*;
use crate::types::*;
use ic_cdk::caller;
use serde_json::Value;

//
//  #  ####       ##   #####  #    # # #    #
//  # #          #  #  #    # ##  ## # ##   #
//  #  ####     #    # #    # # ## # # # #  #
//  #      #    ###### #    # #    # # #  # #
//  # #    #    #    # #    # #    # # #   ##
//  #  ####     #    # #####  #    # # #    #

pub fn caller_is_admin() -> Result<(), String> {
    let caller_id = caller().to_string();
    let admin_canister_id =
        STATE.with(|state: &GlobalState| state.admin_canister_id.borrow().clone());
    if admin_canister_id.is_empty() {
        return Err("Admin canister is not set".to_string());
    } else if caller_id != admin_canister_id {
        return Err("Caller is not the admin canister".to_string());
    }
    return Ok(());
}

//
//  ###### # #    # #####     #####    ##   #####   ##      ###### # ###### #      #####
//  #      # ##   # #    #    #    #  #  #    #    #  #     #      # #      #      #    #
//  #####  # # #  # #    #    #    # #    #   #   #    #    #####  # #####  #      #    #
//  #      # #  # # #    #    #    # ######   #   ######    #      # #      #      #    #
//  #      # #   ## #    #    #    # #    #   #   #    #    #      # #      #      #    #
//  #      # #    # #####     #####  #    #   #   #    #    #      # ###### ###### #####

pub fn find_data_field(field_name: &str) -> Result<ModelDataFieldType, String> {
    let mut data_field_opt: Option<ModelDataFieldType> = None;
    STATE.with(|state: &GlobalState| {
        let data_fields = state.model_data_fields.borrow();
        if let Some(data_field_found) = data_fields.get(field_name) {
            let field_to_return = data_field_found.clone();
            data_field_opt = Some(field_to_return);
        }
    });
    if data_field_opt.is_none() {
        return Err(format!("Data field not found: {}", field_name));
    }
    Ok(data_field_opt.unwrap())
}

//
//  #    #   ##   #      # #####    ##   ##### ######    ###### # ###### #      #####     ##### #   # #####  ######
//  #    #  #  #  #      # #    #  #  #    #   #         #      # #      #      #    #      #    # #  #    # #
//  #    # #    # #      # #    # #    #   #   #####     #####  # #####  #      #    #      #     #   #    # #####
//  #    # ###### #      # #    # ######   #   #         #      # #      #      #    #      #     #   #####  #
//   #  #  #    # #      # #    # #    #   #   #         #      # #      #      #    #      #     #   #      #
//    ##   #    # ###### # #####  #    #   #   ######    #      # ###### ###### #####       #     #   #      ######

pub fn validate_data_field_type(data_type: &str) -> Result<(), String> {
    // validate data field type
    if data_type != "BOOLEAN".to_string()
        && data_type != "STRING".to_string()
        && data_type != "NUMBER".to_string()
        && data_type != "NUMBER_ARRAY".to_string()
        && data_type != "STRING_ARRAY".to_string()
    {
        return Err("Data field type not valid".to_string());
    }
    return Ok(());
}

//
//  #    #   ##   #      # #####    ##   ##### ######    #    #   ##   #      #    # ######
//  #    #  #  #  #      # #    #  #  #    #   #         #    #  #  #  #      #    # #
//  #    # #    # #      # #    # #    #   #   #####     #    # #    # #      #    # #####
//  #    # ###### #      # #    # ######   #   #         #    # ###### #      #    # #
//   #  #  #    # #      # #    # #    #   #   #          #  #  #    # #      #    # #
//    ##   #    # ###### # #####  #    #   #   ######      ##   #    # ######  ####  ######

pub fn validate_json_field_value(json_value: Value, data_type: &str) -> Result<(), String> {
    if data_type == "BOOLEAN".to_string() {
        if !json_value.is_boolean() {
            return Err("Provided JSON value is not a boolean".to_string());
        }
    } else if data_type == "STRING".to_string() {
        if !json_value.is_string() {
            return Err("Provided JSON value is not a string".to_string());
        }
    } else if data_type == "NUMBER".to_string() {
        if !json_value.is_number() {
            return Err("Provided JSON value is not a number".to_string());
        }
    } else if data_type == "NUMBER_ARRAY".to_string() {
        if !json_value.is_array() {
            return Err("Provided JSON value is not an array".to_string());
        }
        if json_value
            .as_array()
            .unwrap()
            .iter()
            .any(|x| !x.is_number())
        {
            return Err("Provided JSON value is not an array of numbers".to_string());
        }
    } else if data_type == "STRING_ARRAY".to_string() {
        if !json_value.is_array() {
            return Err("Provided JSON value is not an array".to_string());
        }
        if json_value
            .as_array()
            .unwrap()
            .iter()
            .any(|x| !x.is_string())
        {
            return Err("Provided JSON value is not an array of strings".to_string());
        }
    }
    return Ok(());
}


//                                                                                           
//   ####  ###### #####    #    #  ####  #####  ###### #         #    #   ##   #    # ###### 
//  #    # #        #      ##  ## #    # #    # #      #         ##   #  #  #  ##  ## #      
//  #      #####    #      # ## # #    # #    # #####  #         # #  # #    # # ## # #####  
//  #  ### #        #      #    # #    # #    # #      #         #  # # ###### #    # #      
//  #    # #        #      #    # #    # #    # #      #         #   ## #    # #    # #      
//   ####  ######   #      #    #  ####  #####  ###### ######    #    # #    # #    # ###### 

pub fn find_model_name() -> Result<String, String> {
    let bucket_model_name = STATE.with(|state: &GlobalState| {
        let name = state.model_name.borrow();
        name.clone()
    });
    if bucket_model_name.is_empty() {
        return Err("Model name not found".to_string());
    }
    Ok(bucket_model_name)
}

//                                                                      
//  ###### # #    # #####     #####  ######  ####   ####  #####  #####  
//  #      # ##   # #    #    #    # #      #    # #    # #    # #    # 
//  #####  # # #  # #    #    #    # #####  #      #    # #    # #    # 
//  #      # #  # # #    #    #####  #      #      #    # #####  #    # 
//  #      # #   ## #    #    #   #  #      #    # #    # #   #  #    # 
//  #      # #    # #####     #    # ######  ####   ####  #    # #####  

pub fn find_record(record_id: &str) -> Result<Record, String> {
    let mut record_opt: Option<Record> = None;
    STATE.with(|state: &GlobalState| {
        let records = state.records.borrow();
        if let Some(record_found) = records.get(record_id) {
            let record_to_return = record_found.clone();
            record_opt = Some(record_to_return);
        }
    });
    if record_opt.is_none() {
        return Err("Record not found".to_string());
    }
    Ok(record_opt.unwrap())
}
