use crate::main::*;
use crate::types::*;
use crate::utilities::*;
use serde_json;
use serde_json::json;
use serde_json::Value;

//
//  ###### # #    # #####       ##   #    # ##### #    #    # #    # ######  ####
//  #      # ##   # #    #     #  #  #    #   #   #    #    # ##   # #      #    #
//  #####  # # #  # #    #    #    # #    #   #   ######    # # #  # #####  #    #
//  #      # #  # # #    #    ###### #    #   #   #    #    # #  # # #      #    #
//  #      # #   ## #    #    #    # #    #   #   #    #    # #   ## #      #    #
//  #      # #    # #####     #    #  ####    #   #    #    # #    # #       ####

pub fn find_auth_info() -> Result<Authentication, String> {
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
    let auth_info_res = find_auth_info();
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
//  #    #   ##   #      # #####       ##   #    # ##### #    #    #####  ####  #    # ###### #    #
//  #    #  #  #  #      # #    #     #  #  #    #   #   #    #      #   #    # #   #  #      ##   #
//  #    # #    # #      # #    #    #    # #    #   #   ######      #   #    # ####   #####  # #  #
//  #    # ###### #      # #    #    ###### #    #   #   #    #      #   #    # #  #   #      #  # #
//   #  #  #    # #      # #    #    #    # #    #   #   #    #      #   #    # #   #  #      #   ##
//    ##   #    # ###### # #####     #    #  ####    #   #    #      #    ####  #    # ###### #    #

pub fn validate_auth_token(token: &str) -> Result<(), String> {
    let auth_info_res = find_auth_info();
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
//  ###### # #    # #####     ##### #####  #    #  ####  ##### ###### #####      ####    ##   #    #  ####
//  #      # ##   # #    #      #   #    # #    # #        #   #      #    #    #    #  #  #  ##   # #
//  #####  # # #  # #    #      #   #    # #    #  ####    #   #####  #    #    #      #    # # #  #  ####
//  #      # #  # # #    #      #   #####  #    #      #   #   #      #    #    #      ###### #  # #      #
//  #      # #   ## #    #      #   #   #  #    # #    #   #   #      #    #    #    # #    # #   ## #    #
//  #      # #    # #####       #   #    #  ####   ####    #   ###### #####      ####  #    # #    #  ####

pub fn find_trusted_canisters() -> Result<Vec<TrustedCanister>, String> {
    let mut trusted_canisters: Vec<TrustedCanister> = Vec::new();
    STATE.with(|state: &GlobalState| {
        let auth = state.auth.borrow();
        trusted_canisters = auth.trusted_canisters.clone();
    });
    Ok(trusted_canisters)
}

//
//  #    #   ##   #      # #####    ##   ##### ######     ####    ##   #      #
//  #    #  #  #  #      # #    #  #  #    #   #         #    #  #  #  #      #
//  #    # #    # #      # #    # #    #   #   #####     #      #    # #      #
//  #    # ###### #      # #    # ######   #   #         #      ###### #      #
//   #  #  #    # #      # #    # #    #   #   #         #    # #    # #      #
//    ##   #    # ###### # #####  #    #   #   ######     ####  #    # ###### ######

pub fn is_call_from_trusted_canister() -> Result<(), String> {
    let trusted_canisters_res = find_trusted_canisters();
    if trusted_canisters_res.is_err() {
        return Err(trusted_canisters_res.err().unwrap());
    }
    let trusted_canisters = trusted_canisters_res.unwrap();
    if trusted_canisters.len() == 0 {
        return Err("No trusted canisters".to_string());
    }
    let caller_id = ic_cdk::caller().to_string();
    for trusted_canister in trusted_canisters {
        if trusted_canister.canister_id == caller_id {
            return Ok(());
        }
    }
    return Err("Caller is not a trusted canister".to_string());
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
//  ###### # #    # #####     ###### # ###### #      #####     ##### #   # #####  ######
//  #      # ##   # #    #    #      # #      #      #    #      #    # #  #    # #
//  #####  # # #  # #    #    #####  # #####  #      #    #      #     #   #    # #####
//  #      # #  # # #    #    #      # #      #      #    #      #     #   #####  #
//  #      # #   ## #    #    #      # #      #      #    #      #     #   #      #
//  #      # #    # #####     #      # ###### ###### #####       #     #   #      ######

pub fn find_model_data_field_type(
    model_name: &str,
    field_name: &str,
) -> Result<ModelDataFieldType, String> {
    let model_res = find_model(model_name);
    if model_res.is_err() {
        return Err(model_res.err().unwrap());
    }
    let model = model_res.unwrap();
    let mut field_opt: Option<ModelDataFieldType> = None;
    for field in model.data_fields {
        if field.field_name == field_name {
            let field_to_return = field.clone();
            field_opt = Some(field_to_return);
        }
    }
    if field_opt.is_none() {
        return Err("Data field not found".to_string());
    }
    Ok(field_opt.unwrap())
}

//
//  #    #   ##   #      # #####     ###### # ###### #      #####     ##### #   # #####  ######
//  #    #  #  #  #      # #    #    #      # #      #      #    #      #    # #  #    # #
//  #    # #    # #      # #    #    #####  # #####  #      #    #      #     #   #    # #####
//  #    # ###### #      # #    #    #      # #      #      #    #      #     #   #####  #
//   #  #  #    # #      # #    #    #      # #      #      #    #      #     #   #      #
//    ##   #    # ###### # #####     #      # ###### ###### #####       #     #   #      ######

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
//  #    #   ##   #      # #####     ###### # ###### #      #####     #    #   ##   #      #    # ######
//  #    #  #  #  #      # #    #    #      # #      #      #    #    #    #  #  #  #      #    # #
//  #    # #    # #      # #    #    #####  # #####  #      #    #    #    # #    # #      #    # #####
//  #    # ###### #      # #    #    #      # #      #      #    #    #    # ###### #      #    # #
//   #  #  #    # #      # #    #    #      # #      #      #    #     #  #  #    # #      #    # #
//    ##   #    # ###### # #####     #      # ###### ###### #####       ##   #    # ######  ####  ######

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


//                                       #                                                 
//       #  ####   ####  #    #           #      #####  ######  ####   ####  #####  #####  
//       # #      #    # ##   #            #     #    # #      #    # #    # #    # #    # 
//       #  ####  #    # # #  #    #####    #    #    # #####  #      #    # #    # #    # 
//       #      # #    # #  # #            #     #####  #      #      #    # #####  #    # 
//  #    # #    # #    # #   ##           #      #   #  #      #    # #    # #   #  #    # 
//   ####   ####   ####  #    #          #       #    # ######  ####   ####  #    # #####  

pub fn convert_json_to_record(json: String) -> Result<Record, String> {
    let json_res: serde_json::Result<Value> = serde_json::from_str(&json);
    if json_res.is_err() {
        return Err("Provided default JSON value is not valid".to_string());
    }
    let json_value = json_res.unwrap();
    if !json_value.is_object() {
        return Err("Provided default JSON value is not an object".to_string());
    }

    // make sure model exists
    let model_name_opt = json_value["model"].as_str();
    if model_name_opt.is_none() {
        return Err("No model name provided".to_string());
    }
    let model_name = model_name_opt.unwrap().to_string();
    let model_res = find_model(&model_name);
    if model_res.is_err() {
        return Err(format!("Model {} not found", model_name));
    }
    let model = model_res.ok().unwrap();

    // get default fields
    let mut new_data_fields: Vec<RecordDataField> = Vec::new();
    for default_field in model.data_fields.iter() {
        let new_data_field = RecordDataField {
            field_name: default_field.field_name.clone(),
            data_type: default_field.data_type.clone(),
            json_value: default_field.default_json_value.clone(),
        };
        new_data_fields.push(new_data_field);
    }

    // loop through and overwrite default fields
    for field in new_data_fields.iter_mut() {
        let field_name = field.field_name.clone();
        let field_is_present = json_value[&field_name].as_str();
        if field_is_present != None {
            continue;
        }
        let field_data_type_res = find_model_data_field_type(&model_name, &field_name);
        if field_data_type_res.is_err() {
            return Err(field_data_type_res.err().unwrap());
        }
        let field_data_type = field_data_type_res.unwrap();

        let json_value = json_value[&field_name].clone();
        let valid_json_value_res =
            validate_json_field_value(json_value.clone(), field_data_type.data_type.clone());
        if valid_json_value_res.is_err() {
            return Err(valid_json_value_res.err().unwrap());
        }
        let new_json_value = serde_json::to_string(&json_value);
        if new_json_value.is_err() {
            return Err("Provided JSON value is not valid".to_string());
        }
        field.json_value = new_json_value.unwrap();
    }

    let record = Record {
        id: json_value["id"].as_str().unwrap_or("").to_string(),
        model_name: model_name.clone(),
        data_fields: new_data_fields.clone(),
    };
    return Ok(record);
}

//                                                     #                                   
//  #####  ######  ####   ####  #####  #####            #           #  ####   ####  #    # 
//  #    # #      #    # #    # #    # #    #            #          # #      #    # ##   # 
//  #    # #####  #      #    # #    # #    #    #####    #         #  ####  #    # # #  # 
//  #####  #      #      #    # #####  #    #            #          #      # #    # #  # # 
//  #   #  #      #    # #    # #   #  #    #           #      #    # #    # #    # #   ## 
//  #    # ######  ####   ####  #    # #####           #        ####   ####   ####  #    # 

pub fn convert_record_to_json(record: Record) -> Result<Value, String> {
    let mut json_value = json!({
        "id": record.id,
        "model": record.model_name,
    });

    // make sure model exists
    let model_res = find_model(&record.model_name);
    if model_res.is_err() {
        return Err(format!("Model {} not found", record.model_name));
    }

    for data_field in record.data_fields.iter() {
        let field_name = data_field.field_name.clone();
        let field_value = data_field.json_value.clone();
        let field_json_value_res = serde_json::from_str(&field_value);
        if field_json_value_res.is_err() {
            return Err("Provided JSON value is not valid".to_string());
        }
        let field_json_value: Value = field_json_value_res.unwrap();

        let field_type = data_field.data_type.clone();
        let valid_type_res = validate_data_field_type(&field_type);
        if valid_type_res.is_err() {
            return Err(valid_type_res.err().unwrap());
        }
        let valid_res = validate_json_field_value(field_json_value.clone(), field_type);
        if valid_res.is_err() {
            return Err(valid_res.err().unwrap());
        }
        json_value[&field_name] = field_json_value;
    }

    return Ok(json_value);
}
