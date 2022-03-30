use crate::helpers::*;
use crate::main::*;
use crate::types::*;
use crate::utilities::*;
use ic_cdk_macros::update;
use serde_json;
use serde_json::{Result, Value};

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
//   ####  ###### #####      ##   #####  #    # # #    #
//  #      #        #       #  #  #    # ##  ## # ##   #
//   ####  #####    #      #    # #    # # ## # # # #  #
//       # #        #      ###### #    # #    # # #  # #
//  #    # #        #      #    # #    # #    # # #   ##
//   ####  ######   #      #    # #####  #    # # #    #

#[update]
pub async fn set_admin_canister(CanisterId { canister_id }: CanisterId) -> BasicResponse {
    let mut res = BasicResponse::default();
    let canister_id_str = canister_id.to_string();
    if canister_id_str.is_empty() {
        res.err = "Provided canister ID is empty".to_string();
        return res;
    }

    STATE.with(|state: &GlobalState| {
        let mut admin_canister_id = state.admin_canister_id.borrow_mut();
        if admin_canister_id.is_empty() {
            *admin_canister_id = canister_id_str.clone();
            res.ok = Some("Admin canister successfully set".to_string());
        } else {
            res.err = "Admin canister already set".to_string();
        }
    });
    return res;
}

//
//   ####  #    # ######  ####  #    #    # ######      ##   #####  #    # # #    #
//  #    # #    # #      #    # #   #     # #          #  #  #    # ##  ## # ##   #
//  #      ###### #####  #      ####      # #####     #    # #    # # ## # # # #  #
//  #      #    # #      #      #  #      # #         ###### #    # #    # # #  # #
//  #    # #    # #      #    # #   #     # #         #    # #    # #    # # #   ##
//   ####  #    # ######  ####  #    #    # #         #    # #####  #    # # #    #

#[update]
pub async fn check_if_admin_canister() -> BasicResponse {
    let mut res = BasicResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    res.ok = Some("Caller is the admin canister".to_string());
    return res;
}

//
//  # #    # # #####    #    #  ####  #####  ###### #
//  # ##   # #   #      ##  ## #    # #    # #      #
//  # # #  # #   #      # ## # #    # #    # #####  #
//  # #  # # #   #      #    # #    # #    # #      #
//  # #   ## #   #      #    # #    # #    # #      #
//  # #    # #   #      #    #  ####  #####  ###### ######

#[update]
pub async fn init_model(InitModelRequest { model_name }: InitModelRequest) -> BasicResponse {
    let mut res = BasicResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }

    STATE.with(|state: &GlobalState| {
        let mut saved_model_name = state.model_name.borrow_mut();
        if saved_model_name.is_empty() {
            *saved_model_name = model_name.clone();
            res.ok = Some("Model successfully initialized".to_string());
        } else {
            res.err = "Model already initialized".to_string();
        }
    });

    return res;
}

//
//    ##   #####  #####     ###### # ###### #      #####
//   #  #  #    # #    #    #      # #      #      #    #
//  #    # #    # #    #    #####  # #####  #      #    #
//  ###### #    # #    #    #      # #      #      #    #
//  #    # #    # #    #    #      # #      #      #    #
//  #    # #####  #####     #      # ###### ###### #####

#[update]
pub async fn add_field(input: ModelDataFieldType) -> BasicResponse {
    let mut res = BasicResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if input.field_name == "id" || input.field_name == "model_name" {
        res.err = "Field name cannot be 'id' or 'model_name'".to_string();
        return res;
    }
    if input.field_name.is_empty() {
        res.err = "Provided field name is empty".to_string();
        return res;
    }
    // check that data field is not already in the model
    let data_field_res = find_data_field(&input.field_name);
    if data_field_res.is_ok() {
        res.err = "Data field already exists".to_string();
        return res;
    }

    let data_field_valid_res = validate_data_field_type(&input.data_type);
    if data_field_valid_res.is_err() {
        res.err = data_field_valid_res.err().unwrap();
        return res;
    }

    let json_res: Result<Value> = serde_json::from_str(&input.default_json_value);
    if json_res.is_err() {
        res.err = "Provided default JSON value is not valid".to_string();
        return res;
    }
    let json = json_res.unwrap();

    let valid_res = validate_json_field_value(json, &input.data_type);
    if valid_res.is_err() {
        res.err = valid_res.err().unwrap();
        return res;
    }

    // update model
    STATE.with(|state: &GlobalState| {
        let mut data_fields = state.model_data_fields.borrow_mut();
        data_fields.insert(input.field_name.clone(), input.clone());
    });

    // update every record
    STATE.with(|state: &GlobalState| {
        let mut records = state.records.borrow_mut();
        for record in records.values_mut() {
            let new_field = RecordDataField {
                field_name: input.field_name.clone(),
                json_value: input.default_json_value.clone(),
            };
            record.data_fields.push(new_field);
        }
    });

    res.ok = Some("Data field successfully added".to_string());
    return res;
}

//
//  #####  ###### #    #  ####  #    # ######    ###### # ###### #      #####
//  #    # #      ##  ## #    # #    # #         #      # #      #      #    #
//  #    # #####  # ## # #    # #    # #####     #####  # #####  #      #    #
//  #####  #      #    # #    # #    # #         #      # #      #      #    #
//  #   #  #      #    # #    #  #  #  #         #      # #      #      #    #
//  #    # ###### #    #  ####    ##   ######    #      # ###### ###### #####

#[update]
pub async fn remove_field(RemoveFieldRequest { field_name }: RemoveFieldRequest) -> BasicResponse {
    let mut res = BasicResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if field_name.is_empty() {
        res.err = "Provided field name is empty".to_string();
        return res;
    }
    if field_name == "id" || field_name == "model_name" {
        res.err = "Provided field name cannot be 'id' or 'model_name'".to_string();
        return res;
    }
    // check that data field is in the model
    let data_field_res = find_data_field(&field_name);
    if data_field_res.is_err() {
        res.err = "Data field does not exist".to_string();
        return res;
    }

    // update model
    STATE.with(|state: &GlobalState| {
        let mut data_fields = state.model_data_fields.borrow_mut();
        data_fields.retain(|key, _value| key != &field_name);
    });

    // update every record
    STATE.with(|state: &GlobalState| {
        let mut records = state.records.borrow_mut();
        for record in records.values_mut() {
            record
                .data_fields
                .retain(|field| field.field_name != field_name);
        }
    });

    res.ok = Some("Data field successfully removed".to_string());
    return res;
}

//                                                                                   
//  # #    #  ####  ###### #####  #####    #####  ######  ####   ####  #####  #####  
//  # ##   # #      #      #    #   #      #    # #      #    # #    # #    # #    # 
//  # # #  #  ####  #####  #    #   #      #    # #####  #      #    # #    # #    # 
//  # #  # #      # #      #####    #      #####  #      #      #    # #####  #    # 
//  # #   ## #    # #      #   #    #      #   #  #      #    # #    # #   #  #    # 
//  # #    #  ####  ###### #    #   #      #    # ######  ####   ####  #    # #####  

#[update]
pub async fn insert_record(
    Record {
        id,
        model_name,
        data_fields,
    }: Record,
) -> RecordResponse {
    let mut res = RecordResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if id.is_empty() {
        res.err = "Provided ID is empty".to_string();
        return res;
    }
    // get model name
    let bucket_model_name_res = find_model_name();
    if bucket_model_name_res.is_err() {
        res.err = bucket_model_name_res.err().unwrap();
        return res;
    }
    let bucket_model_name = bucket_model_name_res.unwrap();
    if bucket_model_name != model_name {
        res.err = "Provided model name does not match the bucket model name".to_string();
        return res;
    }
    // get data fields
    let model_data_fields = STATE.with(|state: &GlobalState| {
        let fields = state.model_data_fields.borrow();
        fields.clone()
    });

    let already_exists_res = find_record(&id);
    if already_exists_res.is_err() {
        res.err = already_exists_res.err().unwrap();
        return res;
    }

    // create record
    let mut new_record = Record {
        id: id.clone(),
        model_name: model_name,
        data_fields: vec![],
    };

    // insert data fields
    for data_field in data_fields {
        let model_data_field_opt = model_data_fields.get(&data_field.field_name);
        if model_data_field_opt.is_none() {
            continue;
        }
        let model_field = model_data_field_opt.unwrap();
        let valid_field_type_res = validate_data_field_type(&model_field.data_type);
        if valid_field_type_res.is_err() {
            res.err = valid_field_type_res.err().unwrap();
            return res;
        }

        let json_res: Result<Value> = serde_json::from_str(&data_field.json_value);
        if json_res.is_err() {
            res.err = "Provided JSON value is not valid".to_string();
            return res;
        }
        let json_value = json_res.unwrap();
        let valid_json_res = validate_json_field_value(json_value, &model_field.data_type);
        if valid_json_res.is_err() {
            res.err = valid_json_res.err().unwrap();
            return res;
        }

        new_record.data_fields.push(RecordDataField {
            field_name: data_field.field_name,
            json_value: data_field.json_value,
        });
    }

    // insert missed data fields
    for (_, data_field) in model_data_fields {
        let mut found = false;
        for record_data_field in new_record.data_fields.clone() {
            if record_data_field.field_name == data_field.field_name {
                found = true;
                break;
            }
        }
        if !found {
            new_record.data_fields.push(RecordDataField {
                field_name: data_field.field_name,
                json_value: data_field.default_json_value,
            });
        }
    }

    // insert record
    STATE.with(|state: &GlobalState| {
        let mut records = state.records.borrow_mut();
        records.insert(new_record.id.clone(), new_record.clone());
    });

    res.ok = Some(new_record);
    return res;
}

//                                                                   
//   ####  ###### #####    #####  ######  ####   ####  #####  #####  
//  #    # #        #      #    # #      #    # #    # #    # #    # 
//  #      #####    #      #    # #####  #      #    # #    # #    # 
//  #  ### #        #      #####  #      #      #    # #####  #    # 
//  #    # #        #      #   #  #      #    # #    # #   #  #    # 
//   ####  ######   #      #    # ######  ####   ####  #    # #####  

#[update]
pub async fn get_record(IdRequest { id }: IdRequest) -> RecordResponse {
    let mut res = RecordResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if id.is_empty() {
        res.err = "Provided ID is empty".to_string();
        return res;
    }

    let record_res = find_record(&id);
    if record_res.is_err() {
        res.err = record_res.err().unwrap();
        return res;
    }

    res.ok = Some(record_res.unwrap());
    return res;
}

//                                                                                        
//  #    # #####  #####    ##   ##### ######    #####  ######  ####   ####  #####  #####  
//  #    # #    # #    #  #  #    #   #         #    # #      #    # #    # #    # #    # 
//  #    # #    # #    # #    #   #   #####     #    # #####  #      #    # #    # #    # 
//  #    # #####  #    # ######   #   #         #####  #      #      #    # #####  #    # 
//  #    # #      #    # #    #   #   #         #   #  #      #    # #    # #   #  #    # 
//   ####  #      #####  #    #   #   ######    #    # ######  ####   ####  #    # #####  

#[update]
pub async fn update_record(
    Record {
        id,
        model_name,
        data_fields,
    }: Record,
) -> RecordResponse {
    let mut res = RecordResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if id.is_empty() {
        res.err = "Provided ID is empty".to_string();
        return res;
    }
    // get model name
    let bucket_model_name_res = find_model_name();
    if bucket_model_name_res.is_err() {
        res.err = bucket_model_name_res.err().unwrap();
        return res;
    }
    let bucket_model_name = bucket_model_name_res.unwrap();
    if bucket_model_name != model_name {
        res.err = "Provided model name does not match the bucket model name".to_string();
        return res;
    }

    let record_res = find_record(&id);
    if record_res.is_err() {
        res.err = record_res.err().unwrap();
        return res;
    }
    let mut record_to_update = record_res.unwrap();

    // get data fields
    let model_data_fields = STATE.with(|state: &GlobalState| {
        let fields = state.model_data_fields.borrow();
        fields.clone()
    });

    // update data fields
    for data_field in data_fields {
        let model_data_field_opt = model_data_fields.get(&data_field.field_name);
        if model_data_field_opt.is_none() {
            continue;
        }
        let model_field = model_data_field_opt.unwrap();
        let valid_field_type_res = validate_data_field_type(&model_field.data_type);
        if valid_field_type_res.is_err() {
            res.err = valid_field_type_res.err().unwrap();
            return res;
        }

        let json_res: Result<Value> = serde_json::from_str(&data_field.json_value);
        if json_res.is_err() {
            res.err = "Provided JSON value is not valid".to_string();
            return res;
        }
        let json_value = json_res.unwrap();
        let valid_json_res = validate_json_field_value(json_value, &model_field.data_type);
        if valid_json_res.is_err() {
            res.err = valid_json_res.err().unwrap();
            return res;
        }

        for field_to_update in record_to_update.data_fields.iter_mut() {
            if field_to_update.field_name == data_field.field_name {
                field_to_update.json_value = data_field.json_value.clone();
                break;
            }
        }
    }

    // insert missed data fields
    for (_, model_data_field) in model_data_fields {
        let mut found = false;
        for record_data_field in record_to_update.data_fields.clone() {
            if record_data_field.field_name == model_data_field.field_name {
                found = true;
                break;
            }
        }
        if !found {
            record_to_update.data_fields.push(RecordDataField {
                field_name: model_data_field.field_name,
                json_value: model_data_field.default_json_value,
            });
        }
    }

    // insert record
    STATE.with(|state: &GlobalState| {
        let mut records = state.records.borrow_mut();
        if let Some(record_found) = records.get_mut(&id) {
            *record_found = record_to_update;
        }
    });

    return res;
}


//                                                                                        
//  #####  ###### #      ###### ##### ######    #####  ######  ####   ####  #####  #####  
//  #    # #      #      #        #   #         #    # #      #    # #    # #    # #    # 
//  #    # #####  #      #####    #   #####     #    # #####  #      #    # #    # #    # 
//  #    # #      #      #        #   #         #####  #      #      #    # #####  #    # 
//  #    # #      #      #        #   #         #   #  #      #    # #    # #   #  #    # 
//  #####  ###### ###### ######   #   ######    #    # ######  ####   ####  #    # #####  

#[update]
pub async fn delete_record(IdRequest { id }: IdRequest) -> BasicResponse {
    let mut res = BasicResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if id.is_empty() {
        res.err = "Provided ID is empty".to_string();
        return res;
    }
    let record_res = find_record(&id);
    if record_res.is_err() {
        res.err = record_res.err().unwrap();
        return res;
    }

    // update every model record
    STATE.with(|state: &GlobalState| {
        let mut records = state.records.borrow_mut();
        records.retain(|key, _| key != &id);
    });

    res.ok = Some("Record deleted".to_string());
    return res;
}

//
//    ##   #    # #####  ####     ##### ###### #      ###### #    # ###### ##### #####  #   #
//   #  #  #    #   #   #    #      #   #      #      #      ##  ## #        #   #    #  # #
//  #    # #    #   #   #    #      #   #####  #      #####  # ## # #####    #   #    #   #
//  ###### #    #   #   #    #      #   #      #      #      #    # #        #   #####    #
//  #    # #    #   #   #    #      #   #      #      #      #    # #        #   #   #    #
//  #    #  ####    #    ####       #   ###### ###### ###### #    # ######   #   #    #   #

pub async fn auto_update_telemetry() {
    let model_name_res = find_model_name();
    if model_name_res.is_err() {
        return;
    }
    let model_name = model_name_res.unwrap();
    ic_cdk::println!("{:?}", model_name);

    let cycles = ic_cdk::api::canister_balance();

    let new_telemetry = SubCanisterTelemetry {
        id: ic_cdk::id().to_string(),
        model_name: model_name.clone(),
        memory_size: 0.0,
        memory_used: get_heap_size() as f64,
        cycles: cycles as f64,
    };

    ic_cdk::println!("{:?}", new_telemetry);

    STATE.with(|state: &GlobalState| {
        let mut telemetry = state.telemetry.borrow_mut();
        *telemetry = new_telemetry;
    });
}

//
//   ####  ###### #####    ##### ###### #      ###### #    # ###### ##### #####  #   #
//  #    # #        #        #   #      #      #      ##  ## #        #   #    #  # #
//  #      #####    #        #   #####  #      #####  # ## # #####    #   #    #   #
//  #  ### #        #        #   #      #      #      #    # #        #   #####    #
//  #    # #        #        #   #      #      #      #    # #        #   #   #    #
//   ####  ######   #        #   ###### ###### ###### #    # ######   #   #    #   #

#[update]
pub async fn get_telemetry() -> SubCanisterTelemetryResponse {
    let mut res = SubCanisterTelemetryResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }

    let telemetry = STATE.with(|s: &GlobalState| s.telemetry.borrow().clone());

    ic_cdk::println!("{:?}", telemetry);

    res.ok = Some(telemetry);
    return res;
}
