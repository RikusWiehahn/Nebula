use crate::helpers::*;
use crate::main::*;
use crate::types::*;
use crate::utilities::generate_uuid;
use ic_cdk_macros::update;
use serde_json;
use serde_json::{Result, Value};

//
//   ####  ###### #####      ##   #####  #    # # #    #
//  #      #        #       #  #  #    # ##  ## # ##   #
//   ####  #####    #      #    # #    # # ## # # # #  #
//       # #        #      ###### #    # #    # # #  # #
//  #    # #        #      #    # #    # #    # # #   ##
//   ####  ######   #      #    # #####  #    # # #    #

#[update(name = "setAdminCanister")]
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

#[update(name = "checkIfAdminCanister")]
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

#[update(name = "initModel")]
pub async fn init_model(InitModel { model_name }: InitModel) -> BasicResponse {
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

#[update(name = "addField")]
pub async fn add_data_field(input: ModelDataFieldType) -> BasicResponse {
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

    let valid_res = validate_json_field_value(json, input.data_type.clone());
    if valid_res.is_err() {
        res.err = valid_res.err().unwrap();
        return res;
    }

    // update model
    STATE.with(|state: &GlobalState| {
        let mut data_fields = state.model_data_fields.borrow_mut();
        data_fields.insert(input.field_name.clone(), input.clone());
    });

    // update every model instance
    STATE.with(|state: &GlobalState| {
        let mut instances = state.instances.borrow_mut();
        for instance in instances.values_mut() {
            let new_field = ModelDataField {
                field_name: input.field_name.clone(),
                json_value: input.default_json_value.clone(),
            };
            instance.data_fields.push(new_field);
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

#[update(name = "removeField")]
pub async fn remove_data_field(input: RemoveField) -> BasicResponse {
    let mut res = BasicResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if input.field_name.is_empty() {
        res.err = "Provided field name is empty".to_string();
        return res;
    }
    if input.field_name == "id" || input.field_name == "model_name" {
        res.err = "Provided field name cannot be 'id' or 'model_name'".to_string();
        return res;
    }
    // check that data field is in the model
    let data_field_res = find_data_field(&input.field_name);
    if data_field_res.is_err() {
        res.err = "Data field does not exist".to_string();
        return res;
    }

    // update model
    STATE.with(|state: &GlobalState| {
        let mut data_fields = state.model_data_fields.borrow_mut();
        data_fields.retain(|key, _value| key != &input.field_name);
    });

    // update every model instance
    STATE.with(|state: &GlobalState| {
        let mut instances = state.instances.borrow_mut();
        for instance in instances.values_mut() {
            instance
                .data_fields
                .retain(|field| field.field_name != input.field_name);
        }
    });

    res.ok = Some("Data field successfully removed".to_string());
    return res;
}

//
//   ####  #####  ######   ##   ##### ######    # #    #  ####  #####   ##   #    #  ####  ######
//  #    # #    # #       #  #    #   #         # ##   # #        #    #  #  ##   # #    # #
//  #      #    # #####  #    #   #   #####     # # #  #  ####    #   #    # # #  # #      #####
//  #      #####  #      ######   #   #         # #  # #      #   #   ###### #  # # #      #
//  #    # #   #  #      #    #   #   #         # #   ## #    #   #   #    # #   ## #    # #
//   ####  #    # ###### #    #   #   ######    # #    #  ####    #   #    # #    #  ####  ######

#[update(name = "createInstance")]
pub async fn create_instance(input: CreateOrUpdateInstance) -> ModelInstanceResponse {
    let mut res = ModelInstanceResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if input.id.is_empty() {
        res.err = "Provided ID is empty".to_string();
        return res;
    }
    if input.json.is_empty() {
        res.err = "Provided JSON field is empty".to_string();
        return res;
    }
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

    // validate json
    let json_res: Result<Value> = serde_json::from_str(&input.json);
    if json_res.is_err() {
        res.err = "Provided default JSON value is not valid".to_string();
        return res;
    }
    let json = json_res.unwrap();
    if !json.is_object() {
        res.err = "Provided default JSON value is not an object".to_string();
        return res;
    }

    let uuid_res = generate_uuid().await;
    if uuid_res.is_err() {
        res.err = uuid_res.err().unwrap();
        return res;
    }

    // create instance
    let mut new_instance = ModelInstance {
        id: uuid_res.unwrap(),
        model_name: model_name,
        data_fields: vec![],
    };

    json.clone()
        .as_object()
        .unwrap()
        .iter()
        .for_each(|(key, value)| {
            // validate every key in the object
            let mut data_field_opt: Option<ModelDataFieldType> = None;
            if let Some(data_field_found) = data_fields.clone().get(key) {
                data_field_opt = Some(data_field_found.clone());
            }
            if data_field_opt.is_none() {
                res.err = format!("Provided JSON value has an invalid key: {}", key);
                return;
            }
            let data_field = data_field_opt.unwrap();
            // validate every value in the object
            let valid_res = validate_json_field_value(value.clone(), data_field.data_type);
            if valid_res.is_err() {
                res.err = valid_res.err().unwrap();
                return;
            }
            // add data field to the instance
            new_instance.data_fields.push(ModelDataField {
                field_name: key.to_string(),
                json_value: value.to_string(),
            });
        });

    // insert any missing data fields
    for data_field in data_fields.clone().into_values() {
        let mut found = false;
        for instance_data_field in new_instance.data_fields.clone() {
            if instance_data_field.field_name == data_field.field_name {
                found = true;
                break;
            }
        }
        if !found {
            new_instance.data_fields.push(ModelDataField {
                field_name: data_field.field_name,
                json_value: data_field.default_json_value,
            });
        }
    }

    // insert instance
    STATE.with(|state: &GlobalState| {
        let mut instances = state.instances.borrow_mut();
        instances.insert(new_instance.id.clone(), new_instance.clone());
    });

    let json_res = get_instance_as_json(&new_instance.id);
    if json_res.is_err() {
        res.err = json_res.err().unwrap();
        return res;
    }

    res.json = Some(json_res.unwrap());
    return res;
}

//
//   ####  ###### #####    # #    #  ####  #####   ##   #    #  ####  ######
//  #    # #        #      # ##   # #        #    #  #  ##   # #    # #
//  #      #####    #      # # #  #  ####    #   #    # # #  # #      #####
//  #  ### #        #      # #  # #      #   #   ###### #  # # #      #
//  #    # #        #      # #   ## #    #   #   #    # #   ## #    # #
//   ####  ######   #      # #    #  ####    #   #    # #    #  ####  ######

#[update(name = "getInstance")]
pub async fn get_instance(input: Id) -> ModelInstanceResponse {
    let mut res = ModelInstanceResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if input.id.is_empty() {
        res.err = "Provided ID is empty".to_string();
        return res;
    }

    let json_res = get_instance_as_json(&input.id);
    if json_res.is_err() {
        res.err = json_res.err().unwrap();
        return res;
    }

    res.json = Some(json_res.unwrap());
    return res;
}

//
//  #    # #####  #####    ##   ##### ######    # #    #  ####  #####   ##   #    #  ####  ######
//  #    # #    # #    #  #  #    #   #         # ##   # #        #    #  #  ##   # #    # #
//  #    # #    # #    # #    #   #   #####     # # #  #  ####    #   #    # # #  # #      #####
//  #    # #####  #    # ######   #   #         # #  # #      #   #   ###### #  # # #      #
//  #    # #      #    # #    #   #   #         # #   ## #    #   #   #    # #   ## #    # #
//   ####  #      #####  #    #   #   ######    # #    #  ####    #   #    # #    #  ####  ######

#[update(name = "updateInstance")]
pub async fn update_instance(input: CreateOrUpdateInstance) -> ModelInstanceResponse {
    let mut res = ModelInstanceResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if input.id.is_empty() {
        res.err = "Provided ID is empty".to_string();
        return res;
    }
    if input.json.is_empty() {
        res.err = "Provided JSON field is empty".to_string();
        return res;
    }
    let instance_res = find_model_instance(&input.id);
    if instance_res.is_err() {
        res.err = instance_res.err().unwrap();
        return res;
    }
    let mut instance_to_update = instance_res.unwrap();

    // get data fields
    let data_fields = STATE.with(|state: &GlobalState| {
        let fields = state.model_data_fields.borrow();
        fields.clone()
    });

    // validate json
    let json_res: Result<Value> = serde_json::from_str(&input.json);
    if json_res.is_err() {
        res.err = "Provided default JSON value is not valid".to_string();
        return res;
    }
    let json = json_res.unwrap();
    if !json.is_object() {
        res.err = "Provided default JSON value is not an object".to_string();
        return res;
    }

    json.clone()
        .as_object()
        .unwrap()
        .iter()
        .for_each(|(key, value)| {
            // validate every key in the object
            let mut data_field_opt: Option<ModelDataFieldType> = None;
            if let Some(data_field_found) = data_fields.clone().get(key) {
                data_field_opt = Some(data_field_found.clone());
            }
            if data_field_opt.is_none() {
                res.err = format!("Provided JSON value has an invalid key: {}", key);
                return;
            }
            let data_field = data_field_opt.unwrap();
            // validate every value in the object
            let valid_res = validate_json_field_value(value.clone(), data_field.data_type);
            if valid_res.is_err() {
                res.err = valid_res.err().unwrap();
                return;
            }

            for instance_data_field in instance_to_update.data_fields.iter_mut() {
                if instance_data_field.field_name == data_field.field_name {
                    instance_data_field.json_value = value.clone().to_string();
                    break;
                }
            }
        });

    // update instance
    STATE.with(|state: &GlobalState| {
        let mut instances = state.instances.borrow_mut();
        if let Some(instance_found) = instances.get_mut(&input.id) {
            *instance_found = instance_to_update.clone();
        }
    });

    let json_res = get_instance_as_json(&input.id);
    if json_res.is_err() {
        res.err = json_res.err().unwrap();
        return res;
    }

    res.json = Some(json_res.unwrap());
    return res;
}

//
//  #####  ###### #      ###### ##### ######    # #    #  ####  #####   ##   #    #  ####  ######
//  #    # #      #      #        #   #         # ##   # #        #    #  #  ##   # #    # #
//  #    # #####  #      #####    #   #####     # # #  #  ####    #   #    # # #  # #      #####
//  #    # #      #      #        #   #         # #  # #      #   #   ###### #  # # #      #
//  #    # #      #      #        #   #         # #   ## #    #   #   #    # #   ## #    # #
//  #####  ###### ###### ######   #   ######    # #    #  ####    #   #    # #    #  ####  ######

#[update(name = "deleteInstance")]
pub async fn delete_instance(input: Id) -> BasicResponse {
    let mut res = BasicResponse::default();
    let is_admin_res = caller_is_admin();
    if is_admin_res.is_err() {
        res.err = is_admin_res.err().unwrap();
        return res;
    }
    if input.id.is_empty() {
        res.err = "Provided ID is empty".to_string();
        return res;
    }
    let instance_res = find_model_instance(&input.id);
    if instance_res.is_err() {
        res.err = instance_res.err().unwrap();
        return res;
    }

    // update every model instance
    STATE.with(|state: &GlobalState| {
        let mut instances = state.instances.borrow_mut();
        instances.retain(|key, _| key != &input.id);
    });

    res.ok = Some("Instance deleted".to_string());
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
    let mut model_name = "".to_string();
    STATE.with(|state: &GlobalState| {
        let saved_model_name = state.model_name.borrow();
        model_name = saved_model_name.clone();
    });
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

#[update(name = "getTelemetry")]
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
