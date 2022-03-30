use crate::helpers::*;
use crate::helpers_bucket::*;
use crate::main::*;
use crate::types::*;
use crate::utilities::*;
use ic_cdk_macros::update;
use serde_json;
use serde_json::{Result, Value};

//
//   ####  #####  ######   ##   ##### ######    #    #  ####  #####  ###### #
//  #    # #    # #       #  #    #   #         ##  ## #    # #    # #      #
//  #      #    # #####  #    #   #   #####     # ## # #    # #    # #####  #
//  #      #####  #      ######   #   #         #    # #    # #    # #      #
//  #    # #   #  #      #    #   #   #         #    # #    # #    # #      #
//   ####  #    # ###### #    #   #   ######    #    #  ####  #####  ###### ######

#[update]
pub async fn create_model(
    CreateOrGetModelRequest { token, model_name }: CreateOrGetModelRequest,
) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    if model_name == "" {
        res.err = "No model name provided".to_string();
        return res;
    }
    let auth_res = validate_auth_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }
    // check if model already exists
    let exists_res = find_model(&model_name);
    if exists_res.is_ok() {
        res.err = "Model with that name already exists".to_string();
        return res;
    }

    // TODO: create first model bucket canister

    // get wasm file
    let get_wasm_res = get_canister_wasm();
    if get_wasm_res.is_err() {
        res.err = get_wasm_res.err().unwrap();
        return res;
    }
    let wasm = get_wasm_res.ok().unwrap();
    // create canister
    const TEN_TRILLION: u64 = 5_000_000_000_000;
    let create_canister_res = create_canister(TEN_TRILLION).await;
    if create_canister_res.is_err() {
        res.err = create_canister_res.err().unwrap();
        return res;
    }
    let new_canister_id = create_canister_res.ok().unwrap();
    // install code
    let install_res = install_wasm(new_canister_id.clone(), wasm.module, vec![]).await;
    if install_res.is_err() {
        res.err = install_res.err().unwrap();
        return res;
    }

    // set this canister as the admin canister
    let set_admin_res = set_admin_canister(new_canister_id.clone()).await;
    if set_admin_res.is_err() {
        res.err = set_admin_res.err().unwrap();
        return res;
    }

    // initialize model
    let init_model_res =
        initialize_sub_canister_model(new_canister_id.clone(), model_name.clone()).await;
    if init_model_res.is_err() {
        res.err = init_model_res.err().unwrap();
        return res;
    }

    // save model to models
    STATE.with(|state: &GlobalState| {
        let mut models = state.models.borrow_mut();
        models.insert(
            model_name.clone(),
            Model {
                model_name: model_name.clone(),
                data_fields: vec![],
                canisters: vec![new_canister_id.clone().to_string()],
            },
        );
    });

    res.ok = Some("Model created".to_string());
    return res;
}

//
//   ####  ###### #####    #    #  ####  #####  ###### #       ####
//  #    # #        #      ##  ## #    # #    # #      #      #
//  #      #####    #      # ## # #    # #    # #####  #       ####
//  #  ### #        #      #    # #    # #    # #      #           #
//  #    # #        #      #    # #    # #    # #      #      #    #
//   ####  ######   #      #    #  ####  #####  ###### ######  ####

#[update]
pub async fn get_models(TokenRequest { token }: TokenRequest) -> ModelListResponse {
    let mut res: ModelListResponse = ModelListResponse::default();
    let auth_res = validate_auth_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }

    let mut models_to_return: Vec<Model> = vec![];
    STATE.with(|state: &GlobalState| {
        let models = state.models.borrow();
        for (_, model) in models.iter() {
            models_to_return.push(model.clone());
        }
    });

    res.ok = models_to_return;
    return res;
}

//
//   ####  ###### #####    #    #  ####  #####  ###### #
//  #    # #        #      ##  ## #    # #    # #      #
//  #      #####    #      # ## # #    # #    # #####  #
//  #  ### #        #      #    # #    # #    # #      #
//  #    # #        #      #    # #    # #    # #      #
//   ####  ######   #      #    #  ####  #####  ###### ######

#[update]
pub async fn get_model(
    CreateOrGetModelRequest { token, model_name }: CreateOrGetModelRequest,
) -> ModelResponse {
    let mut res: ModelResponse = ModelResponse::default();
    let auth_res = validate_auth_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }
    if model_name == "" {
        res.err = "No model name provided".to_string();
        return res;
    }

    let mut model_to_return: Option<Model> = None;
    STATE.with(|state: &GlobalState| {
        let models = state.models.borrow();
        for (_, model) in models.iter() {
            if model.model_name == model_name {
                model_to_return = Some(model.clone());
            }
        }
    });

    res.ok = model_to_return;
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
pub async fn add_model_field(
    CreateOrGetModelRequest { token, model_name }: CreateOrGetModelRequest,
    ModelDataFieldType {
        field_name,
        data_type,
        default_json_value,
    }: ModelDataFieldType,
) -> ModelResponse {
    let mut res: ModelResponse = ModelResponse::default();
    let auth_res = validate_auth_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }
    if field_name == "" {
        res.err = "No field name provided".to_string();
        return res;
    }
    if field_name == "id" || field_name == "model_name" {
        res.err = "Provided field name cannot be 'id' or 'model_name'".to_string();
        return res;
    }

    // makes sure model exists
    let model_res = find_model(&model_name);
    if model_res.is_err() {
        res.err = model_res.err().unwrap();
        return res;
    }

    // makes sure field doesn't already exist
    let model = model_res.ok().unwrap();
    for field in model.data_fields.iter() {
        if field.field_name == field_name {
            res.err = "Field with that name already exists".to_string();
            return res;
        }
    }

    let valid_field_res = validate_data_field_type(&data_type);
    if valid_field_res.is_err() {
        res.err = valid_field_res.err().unwrap();
        return res;
    }

    let json_res: Result<Value> = serde_json::from_str(&default_json_value);
    if json_res.is_err() {
        res.err = "Provided default JSON value is not valid".to_string();
        return res;
    }
    let json = json_res.unwrap();
    if !json.is_object() {
        res.err = "Provided default JSON value is not an object".to_string();
        return res;
    }

    let valid_res = validate_json_field_value(json.clone(), data_type.clone());
    if valid_res.is_err() {
        res.err = valid_res.err().unwrap();
        return res;
    }

    let mut model_to_return: Option<Model> = None;
    STATE.with(|state: &GlobalState| {
        let mut models = state.models.borrow_mut();

        if let Some(model_found) = models.get_mut(&model_name) {
            model_found.data_fields.push(ModelDataFieldType {
                field_name: field_name.clone(),
                data_type: data_type.clone(),
                default_json_value: json.clone().to_string(),
            });
            model_to_return = Some(model_found.clone());
        }
    });

    // update model in all it's canisters
    for sub_canister_id in model.canisters {
        let sub_canister_res = add_field_to_sub_canister(
            sub_canister_id,
            ModelDataFieldType {
                field_name: field_name.clone(),
                data_type: data_type.clone(),
                default_json_value: default_json_value.clone(),
            },
        )
        .await;
        if sub_canister_res.is_err() {
            res.err = sub_canister_res.err().unwrap();
            return res;
        }
    }

    res.ok = model_to_return;
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
pub async fn remove_model_field(
    RemoveModelFieldRequest {
        token,
        model_name,
        field_name,
    }: RemoveModelFieldRequest,
) -> ModelResponse {
    let mut res: ModelResponse = ModelResponse::default();
    let auth_res = validate_auth_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }
    if model_name == "" {
        res.err = "No model name provided".to_string();
        return res;
    }
    if field_name == "" {
        res.err = "No field name provided".to_string();
        return res;
    }
    if field_name == "id" || field_name == "model_name" {
        res.err = "Provided field name cannot be 'id' or 'model_name'".to_string();
        return res;
    }

    // makes sure model exists
    let model_res = find_model(&model_name);
    if model_res.is_err() {
        res.err = model_res.err().unwrap();
        return res;
    }

    // makes sure field exists
    let model = model_res.ok().unwrap();
    let mut field_exists = false;
    for field in model.data_fields.iter() {
        if field.field_name == field_name {
            field_exists = true;
        }
    }
    if !field_exists {
        res.err = "Field with that name does not exist".to_string();
        return res;
    }

    let mut model_to_return: Option<Model> = None;
    STATE.with(|state: &GlobalState| {
        let mut models = state.models.borrow_mut();

        if let Some(model_found) = models.get_mut(&model_name) {
            model_found
                .data_fields
                .retain(|field| field.field_name != field_name);
            model_to_return = Some(model_found.clone());
        }
    });

    // update model in all it's sub-canisters
    for sub_canister_id in model.canisters {
        let sub_canister_res =
            remove_field_from_sub_canister(sub_canister_id, field_name.clone()).await;
        if sub_canister_res.is_err() {
            res.err = sub_canister_res.err().unwrap();
            return res;
        }
    }

    res.ok = model_to_return;
    return res;
}

//
//  #####  ###### #      ###### ##### ######    #    #  ####  #####  ###### #
//  #    # #      #      #        #   #         ##  ## #    # #    # #      #
//  #    # #####  #      #####    #   #####     # ## # #    # #    # #####  #
//  #    # #      #      #        #   #         #    # #    # #    # #      #
//  #    # #      #      #        #   #         #    # #    # #    # #      #
//  #####  ###### ###### ######   #   ######    #    #  ####  #####  ###### ######

#[update]
pub async fn delete_model(
    CreateOrGetModelRequest { token, model_name }: CreateOrGetModelRequest,
) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_res = validate_auth_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }
    if model_name == "" {
        res.err = "No model name provided".to_string();
        return res;
    }

    // makes sure model exists
    let model_res = find_model(&model_name);
    if model_res.is_err() {
        res.err = model_res.err().unwrap();
        return res;
    }
    let model = model_res.ok().unwrap();

    // TODO - delete all of this model's canisters
    for sub_canister_id in model.canisters {
        // destroy canister
        let destroy_res = destroy_sub_canister(&sub_canister_id).await;
        if destroy_res.is_err() {
            res.err = destroy_res.err().unwrap();
            return res;
        }
        delay();
    }

    STATE.with(|state: &GlobalState| {
        let mut models = state.models.borrow_mut();
        models.retain(|key, _| key != &model_name);
    });

    res.ok = Some("Model deleted".to_string());
    return res;
}

//
//   ####  #####  ######   ##   ##### ######    #####  ######  ####   ####  #####  #####
//  #    # #    # #       #  #    #   #         #    # #      #    # #    # #    # #    #
//  #      #    # #####  #    #   #   #####     #    # #####  #      #    # #    # #    #
//  #      #####  #      ######   #   #         #####  #      #      #    # #####  #    #
//  #    # #   #  #      #    #   #   #         #   #  #      #    # #    # #   #  #    #
//   ####  #    # ###### #    #   #   ######    #    # ######  ####   ####  #    # #####

#[update]
pub async fn create_record(
    CreateOrUpdateRecordJson { token, json }: CreateOrUpdateRecordJson,
) -> RecordJsonResponse {
    let mut res: RecordJsonResponse = RecordJsonResponse::default();
    let auth_res = validate_auth_token(&token);
    let trusted_res = is_call_from_trusted_canister();
    if auth_res.is_err() && trusted_res.is_err() {
        res.err = auth_res.err().unwrap() + " " + &trusted_res.err().unwrap();
        return res;
    }

    let record_res = convert_json_to_record(json);
    if record_res.is_err() {
        res.err = record_res.err().unwrap();
        return res;
    }
    let mut record = record_res.ok().unwrap();

    // generate uuid
    let uuid_res = generate_uuid().await;
    if uuid_res.is_err() {
        res.err = uuid_res.err().unwrap();
        return res;
    }
    record.id = uuid_res.unwrap();

    // get model
    let model_res = find_model(&record.model_name);
    if model_res.is_err() {
        res.err = model_res.err().unwrap();
        return res;
    }
    let model = model_res.unwrap();
    if model.canisters.len() == 0 {
        res.err = "No canisters associated with this model".to_string();
        return res;
    }
    let canister_id = model.canisters[0].clone(); // TODO - pick canister based on memory usage

    let insert_res = insert_record_into_sub_canister(canister_id, record.clone()).await;
    if insert_res.is_err() {
        res.err = insert_res.err().unwrap();
        return res;
    }

    let record_as_json_res = convert_record_to_json(record.clone());
    if record_as_json_res.is_err() {
        res.err = record_as_json_res.err().unwrap();
        return res;
    }
    let record_as_json = record_as_json_res.unwrap();
    let record_as_string_res = serde_json::to_string(&record_as_json);
    if record_as_string_res.is_err() {
        res.err = "Failed to convert record to json".to_string();
        return res;
    }

    res.json = Some(record_as_string_res.unwrap());
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
    CreateOrUpdateRecordJson { token, json }: CreateOrUpdateRecordJson,
) -> RecordJsonResponse {
    let mut res: RecordJsonResponse = RecordJsonResponse::default();
    let auth_res = validate_auth_token(&token);
    let trusted_res = is_call_from_trusted_canister();
    if auth_res.is_err() && trusted_res.is_err() {
        res.err = auth_res.err().unwrap() + " " + &trusted_res.err().unwrap();
        return res;
    }
    let record_res = convert_json_to_record(json);
    if record_res.is_err() {
        res.err = record_res.err().unwrap();
        return res;
    }
    let record = record_res.ok().unwrap();

    // make sure model exists
    let model_res = find_model(&record.model_name);
    if model_res.is_err() {
        res.err = model_res.err().unwrap();
        return res;
    }
    let model = model_res.ok().unwrap();
    if model.canisters.len() == 0 {
        res.err = "No canisters associated with this model".to_string();
        return res;
    }
    let canister_id = model.canisters[0].clone(); // TODO - pick canister based on memory usage

    // make sure record exists
    let record_exists_res =
        find_record_in_sub_canister(canister_id.clone(), record.id.clone()).await;
    if record_exists_res.is_err() {
        res.err = record_exists_res.err().unwrap();
        return res;
    }

    // update record
    let insert_res = update_record_in_sub_canister(canister_id, record.clone()).await;
    if insert_res.is_err() {
        res.err = insert_res.err().unwrap();
        return res;
    }

    let record_as_json_res = convert_record_to_json(record.clone());
    if record_as_json_res.is_err() {
        res.err = record_as_json_res.err().unwrap();
        return res;
    }
    let record_as_json = record_as_json_res.unwrap();
    let record_as_string_res = serde_json::to_string(&record_as_json);
    if record_as_string_res.is_err() {
        res.err = "Failed to convert record to json".to_string();
        return res;
    }

    res.json = Some(record_as_string_res.unwrap());
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
pub async fn get_record(
    RecordRequest {
        token,
        id,
        model_name,
    }: RecordRequest,
) -> RecordJsonResponse {
    let mut res: RecordJsonResponse = RecordJsonResponse::default();
    let auth_res = validate_auth_token(&token);
    let trusted_res = is_call_from_trusted_canister();
    if auth_res.is_err() && trusted_res.is_err() {
        res.err = auth_res.err().unwrap() + " " + &trusted_res.err().unwrap();
        return res;
    }
    // make sure model exists
    let model_res = find_model(&model_name);
    if model_res.is_err() {
        res.err = model_res.err().unwrap();
        return res;
    }
    let model = model_res.ok().unwrap();
    if model.canisters.len() == 0 {
        res.err = "No canisters associated with this model".to_string();
        return res;
    }
    let canister_id = model.canisters[0].clone(); // TODO - pick canister based on memory usage

    // make sure record exists
    let record_res = find_record_in_sub_canister(canister_id.clone(), id.clone()).await;
    if record_res.is_err() {
        res.err = record_res.err().unwrap();
        return res;
    }
    let record = record_res.ok();
    if record.is_none() {
        res.err = "record not found".to_string();
        return res;
    }

    let record_as_json_res = convert_record_to_json(record.unwrap());
    if record_as_json_res.is_err() {
        res.err = record_as_json_res.err().unwrap();
        return res;
    }
    let record_as_json = record_as_json_res.unwrap();
    let record_as_string_res = serde_json::to_string(&record_as_json);
    if record_as_string_res.is_err() {
        res.err = "Failed to convert record to json".to_string();
        return res;
    }

    res.json = Some(record_as_string_res.unwrap());
    return res;
}

//
//  #####  ######  ####   ####  #####  #####     #      #  ####  #####
//  #    # #      #    # #    # #    # #    #    #      # #        #
//  #    # #####  #      #    # #    # #    #    #      #  ####    #
//  #####  #      #      #    # #####  #    #    #      #      #   #
//  #   #  #      #    # #    # #   #  #    #    #      # #    #   #
//  #    # ######  ####   ####  #    # #####     ###### #  ####    #

#[update]
pub async fn get_record_list(
    RecordListRequest {
        token,
        model_name,
        page,
        page_size,
    }: RecordListRequest,
) -> RecordListJsonResponse {
    let mut res: RecordListJsonResponse = RecordListJsonResponse::default();
    let auth_res = validate_auth_token(&token);
    let trusted_res = is_call_from_trusted_canister();
    if auth_res.is_err() && trusted_res.is_err() {
        res.err = auth_res.err().unwrap() + " " + &trusted_res.err().unwrap();
        return res;
    }
    // make sure model exists
    let model_res = find_model(&model_name);
    if model_res.is_err() {
        res.err = model_res.err().unwrap();
        return res;
    }
    let model = model_res.ok().unwrap();
    if model.canisters.len() == 0 {
        res.err = "No canisters associated with this model".to_string();
        return res;
    }
    let canister_id = model.canisters[0].clone(); // TODO - pick canister based on memory usage

    let records_res = find_sub_canister_records_list(canister_id, page, page_size).await;
    if records_res.is_err() {
        res.err = records_res.err().unwrap();
        return res;
    }
    let records = records_res.unwrap();
    res.page = records.page;
    res.page_size = records.page_size;

    let mut records_to_return: Vec<String> = Vec::new();
    for record in records.ok {
        let json_res = convert_record_to_json(record);
        if json_res.is_err() {
            res.err = json_res.err().unwrap();
            return res;
        }
        let json = json_res.unwrap();
        let string_res = serde_json::to_string(&json);
        if string_res.is_err() {
            res.err = "Failed to convert record to json".to_string();
            return res;
        }
        records_to_return.push(string_res.unwrap());
    }
    res.ok = records_to_return;

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
pub async fn delete_record(
    RecordRequest {
        token,
        id,
        model_name,
    }: RecordRequest,
) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_res = validate_auth_token(&token);
    let trusted_res = is_call_from_trusted_canister();
    if auth_res.is_err() && trusted_res.is_err() {
        res.err = auth_res.err().unwrap() + " " + &trusted_res.err().unwrap();
        return res;
    }
    // make sure model exists
    let model_res = find_model(&model_name);
    if model_res.is_err() {
        res.err = model_res.err().unwrap();
        return res;
    }
    let model = model_res.ok().unwrap();
    if model.canisters.len() == 0 {
        res.err = "No canisters associated with this model".to_string();
        return res;
    }
    let canister_id = model.canisters[0].clone(); // TODO - pick canister based on memory usage

    // make sure record exists
    let record_res = delete_record_in_sub_canister(canister_id.clone(), id.clone()).await;
    if record_res.is_err() {
        res.err = record_res.err().unwrap();
        return res;
    }

    res.ok = Some("record deleted".to_string());
    return res;
}
