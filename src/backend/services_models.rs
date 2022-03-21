use crate::helpers::*;
use crate::main::*;
use crate::services_bucket::initialize_canister_model;
use crate::services_bucket::set_admin_canister;
use crate::types::*;
use crate::utilities::create_canister;
use crate::utilities::get_canister_wasm;
use crate::utilities::install_wasm;
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

#[update(name = "createModel")]
pub async fn create_model(
    CreateOrGetModel { token, model_name }: CreateOrGetModel,
) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    if model_name == "" {
        res.err = "No model name provided".to_string();
        return res;
    }
    let auth_res = authenticate_token(&token);
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

    // create canister
    const TEN_TRILLION: u64 = 10_000_000_000_000;
    let create_canister_res = create_canister(TEN_TRILLION).await;
    if create_canister_res.is_err() {
        res.err = create_canister_res.err().unwrap();
        return res;
    }
    let new_canister_id = create_canister_res.ok().unwrap();
    // install code
    let get_wasm_res = get_canister_wasm();
    if get_wasm_res.is_err() {
        res.err = get_wasm_res.err().unwrap();
        return res;
    }
    let wasm = get_wasm_res.ok().unwrap();
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
        initialize_canister_model(new_canister_id.clone(), model_name.clone()).await;
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

#[update(name = "getModels")]
pub async fn get_models(TokenRecord { token }: TokenRecord) -> ModelListResponse {
    let mut res: ModelListResponse = ModelListResponse::default();
    let auth_res = authenticate_token(&token);
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

#[update(name = "getModel")]
pub async fn get_model(CreateOrGetModel { token, model_name }: CreateOrGetModel) -> ModelResponse {
    let mut res: ModelResponse = ModelResponse::default();
    let auth_res = authenticate_token(&token);
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

#[update(name = "addModelField")]
pub async fn add_model_field(
    CreateOrGetModel { token, model_name }: CreateOrGetModel,
    ModelDataFieldType {
        field_name,
        data_type,
        default_json_value,
    }: ModelDataFieldType,
) -> ModelResponse {
    let mut res: ModelResponse = ModelResponse::default();
    let auth_res = authenticate_token(&token);
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

    // TODO - update model in all it's canisters

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

#[update(name = "removeModelField")]
pub async fn remove_model_field(
    RemoveModelField {
        token,
        model_name,
        field_name,
    }: RemoveModelField,
) -> ModelResponse {
    let mut res: ModelResponse = ModelResponse::default();
    let auth_res = authenticate_token(&token);
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

    // TODO - update model in all it's sub-canisters

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

// "deleteModel": (record { token: text; model_name: text }) -> (BasicResponse);
#[update(name = "deleteModel")]
pub async fn delete_model(
    CreateOrGetModel { token, model_name }: CreateOrGetModel,
) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_res = authenticate_token(&token);
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

    STATE.with(|state: &GlobalState| {
        let mut models = state.models.borrow_mut();
        models.retain(|key, _| key != &model_name);
    });

    // TODO - delete all of this model's canisters

    res.ok = Some("Model deleted".to_string());
    return res;
}

//
//   ####  #####  ######   ##   ##### ######    # #    #  ####  #####   ##   #    #  ####  ######
//  #    # #    # #       #  #    #   #         # ##   # #        #    #  #  ##   # #    # #
//  #      #    # #####  #    #   #   #####     # # #  #  ####    #   #    # # #  # #      #####
//  #      #####  #      ######   #   #         # #  # #      #   #   ###### #  # # #      #
//  #    # #   #  #      #    #   #   #         # #   ## #    #   #   #    # #   ## #    # #
//   ####  #    # ###### #    #   #   ######    # #    #  ####    #   #    # #    #  ####  ######

// "createModelInstance": (record { token: text; json: text; }) -> (ModelInstanceResponse);

//
//  #    # #####  #####    ##   ##### ######    # #    #  ####  #####   ##   #    #  ####  ######
//  #    # #    # #    #  #  #    #   #         # ##   # #        #    #  #  ##   # #    # #
//  #    # #    # #    # #    #   #   #####     # # #  #  ####    #   #    # # #  # #      #####
//  #    # #####  #    # ######   #   #         # #  # #      #   #   ###### #  # # #      #
//  #    # #      #    # #    #   #   #         # #   ## #    #   #   #    # #   ## #    # #
//   ####  #      #####  #    #   #   ######    # #    #  ####    #   #    # #    #  ####  ######

// "updateModelInstance": (record { token: text; id: text; json: text; }) -> (ModelInstanceResponse);

//
//   ####  ###### #####    # #    #  ####  #####   ##   #    #  ####  ######
//  #    # #        #      # ##   # #        #    #  #  ##   # #    # #
//  #      #####    #      # # #  #  ####    #   #    # # #  # #      #####
//  #  ### #        #      # #  # #      #   #   ###### #  # # #      #
//  #    # #        #      # #   ## #    #   #   #    # #   ## #    # #
//   ####  ######   #      # #    #  ####    #   #    # #    #  ####  ######

// "getModelInstance": (record { token: text; id: text }) -> (ModelInstanceResponse);

//
//  #####  ###### #      ###### ##### ######    # #    #  ####  #####   ##   #    #  ####  ######
//  #    # #      #      #        #   #         # ##   # #        #    #  #  ##   # #    # #
//  #    # #####  #      #####    #   #####     # # #  #  ####    #   #    # # #  # #      #####
//  #    # #      #      #        #   #         # #  # #      #   #   ###### #  # # #      #
//  #    # #      #      #        #   #         # #   ## #    #   #   #    # #   ## #    # #
//  #####  ###### ###### ######   #   ######    # #    #  ####    #   #    # #    #  ####  ######

// "deleteModelInstance": (record { token: text; id: text }) -> (BasicResponse);
