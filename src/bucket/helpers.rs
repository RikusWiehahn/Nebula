use crate::main::*;
use crate::types::*;
use ic_cdk::caller;
use serde_json::json;
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
        return Err("Data field not found".to_string());
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

pub fn validate_json_field_value(json_value: Value, data_type: String) -> Result<(), String> {
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
//  ###### # #    # #####     # #    #  ####  #####   ##   #    #  ####  ######
//  #      # ##   # #    #    # ##   # #        #    #  #  ##   # #    # #
//  #####  # # #  # #    #    # # #  #  ####    #   #    # # #  # #      #####
//  #      # #  # # #    #    # #  # #      #   #   ###### #  # # #      #
//  #      # #   ## #    #    # #   ## #    #   #   #    # #   ## #    # #
//  #      # #    # #####     # #    #  ####    #   #    # #    #  ####  ######

pub fn find_model_instance(instance_id: &str) -> Result<ModelInstance, String> {
    let mut instance_opt: Option<ModelInstance> = None;
    STATE.with(|state: &GlobalState| {
        let instances = state.instances.borrow();
        if let Some(instance_found) = instances.get(instance_id) {
            let instance_to_return = instance_found.clone();
            instance_opt = Some(instance_to_return);
        }
    });
    if instance_opt.is_none() {
        return Err("Data field not found".to_string());
    }
    Ok(instance_opt.unwrap())
}

//
//   ####  ###### #####    # #    #  ####  #####   ##   #    #  ####  ######      ##    ####          #  ####   ####  #    #
//  #    # #        #      # ##   # #        #    #  #  ##   # #    # #          #  #  #              # #      #    # ##   #
//  #      #####    #      # # #  #  ####    #   #    # # #  # #      #####     #    #  ####          #  ####  #    # # #  #
//  #  ### #        #      # #  # #      #   #   ###### #  # # #      #         ######      #         #      # #    # #  # #
//  #    # #        #      # #   ## #    #   #   #    # #   ## #    # #         #    # #    #    #    # #    # #    # #   ##
//   ####  ######   #      # #    #  ####    #   #    # #    #  ####  ######    #    #  ####      ####   ####   ####  #    #

pub fn get_instance_as_json(instance_id: &str) -> Result<String, String> {
    let instance_res = find_model_instance(instance_id);
    if instance_res.is_err() {
        return Err(instance_res.err().unwrap());
    }
    let instance = instance_res.unwrap();

    // get model name
    let model_name = STATE.with(|state: &GlobalState| {
        let name = state.model_name.borrow();
        name.clone()
    });
    // get data fields
    let data_fields = STATE.with(|state: &GlobalState| {
        let fields = state.model_data_fields.borrow();
        fields.clone()
    });

    let mut instance_json = json!({
      "id": instance_id,
      "model_name": model_name,
    });

    for data_field in data_fields.values() {
        let data_field_name = data_field.field_name.clone();
        let mut data_field_opt: Option<ModelDataField> = None;
        for instance_data_field_value in instance.data_fields.iter() {
            if instance_data_field_value.field_name == data_field_name {
                data_field_opt = Some(instance_data_field_value.clone());
            }
        }

        if data_field_opt.is_some() {
            let data_field = data_field_opt.unwrap();
            let data_field_value: serde_json::Result<Value> =
                serde_json::from_str(&data_field.json_value);
            if data_field_value.is_ok() {
                instance_json[data_field_name.as_str()] = data_field_value.unwrap();
            }
        }
    }

    return Ok(instance_json.to_string());
}
