use crate::main::*;
use crate::types::*;
use crate::utilities::*;
use serde_json;
use serde_json::{Value};


//
//   ####  ###### #####      ##   #    # ##### #    #
//  #    # #        #       #  #  #    #   #   #    #
//  #      #####    #      #    # #    #   #   ######
//  #  ### #        #      ###### #    #   #   #    #
//  #    # #        #      #    # #    #   #   #    #
//   ####  ######   #      #    #  ####    #   #    #

pub fn get_auth_info() -> Result<Authentication, String> {
    let mut auth_info: Authentication = Authentication::default();
    STATE.with(|state: &GlobalState| {
        let auth = state.auth.borrow_mut();
        auth_info = auth.clone();
    });
    if auth_info.password_hash.is_empty() {
        return Err("Authentication has not been set up.".to_string());
    }
    return Ok(auth_info);
}

//
//  #    #   ##   #      # #####    ##   ##### ######     ####  ######  ####   ####  #  ####  #    #    # #####
//  #    #  #  #  #      # #    #  #  #    #   #         #      #      #      #      # #    # ##   #    # #    #
//  #    # #    # #      # #    # #    #   #   #####      ####  #####   ####   ####  # #    # # #  #    # #    #
//  #    # ###### #      # #    # ######   #   #              # #           #      # # #    # #  # #    # #    #
//   #  #  #    # #      # #    # #    #   #   #         #    # #      #    # #    # # #    # #   ##    # #    #
//    ##   #    # ###### # #####  #    #   #   ######     ####  ######  ####   ####  #  ####  #    #    # #####

pub fn validate_session_id(session_id: &str) -> Result<(), String> {
    let auth_info_res = get_auth_info();
    if auth_info_res.is_err() {
        return Err(auth_info_res.err().unwrap());
    }
    let auth_info = auth_info_res.unwrap();

    if auth_info.session_id == session_id {
        return Ok(());
    }
    return Err(String::from("Invalid token"));
}

//
//  #####  ####  #    # ###### #    #      ##   #    # ##### #    #
//    #   #    # #   #  #      ##   #     #  #  #    #   #   #    #
//    #   #    # ####   #####  # #  #    #    # #    #   #   ######
//    #   #    # #  #   #      #  # #    ###### #    #   #   #    #
//    #   #    # #   #  #      #   ##    #    # #    #   #   #    #
//    #    ####  #    # ###### #    #    #    #  ####    #   #    #

pub fn authenticate_token(token: &str) -> Result<(), String> {
    let auth_info_res = get_auth_info();
    if auth_info_res.is_err() {
        return Err(auth_info_res.err().unwrap());
    }
    if token == "" {
        return Err("No token provided".to_string());
    }
    let token_res = decode_id_from_token(&token);
    if token_res.is_err() {
        return Err(token_res.err().unwrap());
    }
    let session_id = token_res.unwrap();

    let session_res = validate_session_id(&session_id);
    if session_res.is_err() {
        return Err(session_res.err().unwrap());
    }
    return Ok(());
}


//                                                                                                                                    
//   ####  ###### #####    ##### #####  #    #  ####  ##### ###### #####      ####    ##   #    # #  ####  ##### ###### #####   ####  
//  #    # #        #        #   #    # #    # #        #   #      #    #    #    #  #  #  ##   # # #        #   #      #    # #      
//  #      #####    #        #   #    # #    #  ####    #   #####  #    #    #      #    # # #  # #  ####    #   #####  #    #  ####  
//  #  ### #        #        #   #####  #    #      #   #   #      #    #    #      ###### #  # # #      #   #   #      #####       # 
//  #    # #        #        #   #   #  #    # #    #   #   #      #    #    #    # #    # #   ## # #    #   #   #      #   #  #    # 
//   ####  ######   #        #   #    #  ####   ####    #   ###### #####      ####  #    # #    # #  ####    #   ###### #    #  ####  

pub fn find_trusted_canisters() -> Result<Vec<String>, String> {
    let mut trusted_canisters: Vec<String> = Vec::new();
    STATE.with(|state: &GlobalState| {
        let auth = state.auth.borrow();
        trusted_canisters = auth.trusted_canister_ids.clone();
    });
    Ok(trusted_canisters)
}

//                                                               
//  ###### # #    # #####     #    #  ####  #####  ###### #      
//  #      # ##   # #    #    ##  ## #    # #    # #      #      
//  #####  # # #  # #    #    # ## # #    # #    # #####  #      
//  #      # #  # # #    #    #    # #    # #    # #      #      
//  #      # #   ## #    #    #    # #    # #    # #      #      
//  #      # #    # #####     #    #  ####  #####  ###### ###### 

pub fn find_model(model_name: &str) -> Result<Model, String> {
    let mut model_opt: Option<Model> = None;
    STATE.with(|state: &GlobalState| {
        let models = state.models.borrow();
        if let Some(model_found) = models.get(model_name) {
            let model_to_return = model_found.clone();
            model_opt = Some(model_to_return);
        }
    });
    if model_opt.is_none() {
        return Err("Data model not found".to_string());
    }
    Ok(model_opt.unwrap())
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