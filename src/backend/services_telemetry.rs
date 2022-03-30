use crate::helpers::*;
use crate::main::*;
use crate::types::*;
use crate::utilities::get_canister_wasm;
use ic_cdk::api::time;
use ic_cdk::export::Principal;
use ic_cdk_macros::update;

//
//    ##   #    # #####  ####     #    # #####  #####    ##   ##### ######
//   #  #  #    #   #   #    #    #    # #    # #    #  #  #    #   #
//  #    # #    #   #   #    #    #    # #    # #    # #    #   #   #####
//  ###### #    #   #   #    #    #    # #####  #    # ######   #   #
//  #    # #    #   #   #    #    #    # #      #    # #    #   #   #
//  #    #  ####    #    ####      ####  #      #####  #    #   #   ######

pub async fn auto_update_telemetry() {
    // get own identity
    let main_canister = ic_cdk::id();
    let cycles_available = ic_cdk::api::canister_balance();

    let bucket_wasm_res = get_canister_wasm();
    let mut bucket_wasm_size: f64 = 0.0;
    if bucket_wasm_res.is_ok() {
        let bucket_wasm = bucket_wasm_res.unwrap();
        bucket_wasm_size = bucket_wasm.module.len() as f64;
    }

    let new_telemetry = Telemetry {
        last_status_check: time() as f64,
        main_id: main_canister.to_string(),
        main_memory_size: 0.0,
        main_memory_used: get_heap_size() as f64,
        main_cycles: cycles_available as f64,
        bucket_wasm_size: bucket_wasm_size,
        sub_canisters: vec![],
    };

    STATE.with(|state: &GlobalState| {
        let mut telemetry = state.telemetry.borrow_mut();
        *telemetry = new_telemetry;
    });
}

//
//   ####  ###### #####     ####  #####   ##   ##### #    #  ####
//  #    # #        #      #        #    #  #    #   #    # #
//  #      #####    #       ####    #   #    #   #   #    #  ####
//  #  ### #        #           #   #   ######   #   #    #      #
//  #    # #        #      #    #   #   #    #   #   #    # #    #
//   ####  ######   #       ####    #   #    #   #    ####   ####

#[update]
pub async fn get_telemetry(TokenRecord { token }: TokenRecord) -> TelemetryResponse {
    let mut res = TelemetryResponse::default();
    let auth_res = validate_auth_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }
    let mut telemetry = STATE.with(|s: &GlobalState| s.telemetry.borrow().clone());

    // get models
    let mut models: Vec<Model> = Vec::new();
    STATE.with(|state: &GlobalState| {
        let data_models = state.models.borrow();
        for (_, model) in data_models.iter() {
            models.push(model.clone());
        }
    });

    // get telemetry for each model sub canister
    let mut all_sub_canisters_telemetry: Vec<SubCanisterTelemetry> = Vec::new();
    for model in models.iter() {
        let sub_canister_ids = model.canisters.clone();
        for sub_canister_id in sub_canister_ids.iter() {
            let tel_res = get_sub_canister_telemetry(&sub_canister_id).await;
            if tel_res.is_err() {
                res.err = tel_res.err().unwrap();
                return res;
            }
            let sub_canister_telemetry = tel_res.unwrap();
            all_sub_canisters_telemetry.push(sub_canister_telemetry);
        }
    }
    telemetry.sub_canisters = all_sub_canisters_telemetry;

    res.ok = Some(telemetry);
    return res;
}

//
//   ####  #    # #####     ##### ###### #      ###### #    # ###### ##### #####  #   #
//  #      #    # #    #      #   #      #      #      ##  ## #        #   #    #  # #
//   ####  #    # #####       #   #####  #      #####  # ## # #####    #   #    #   #
//       # #    # #    #      #   #      #      #      #    # #        #   #####    #
//  #    # #    # #    #      #   #      #      #      #    # #        #   #   #    #
//   ####   ####  #####       #   ###### ###### ###### #    # ######   #   #    #   #

pub async fn get_sub_canister_telemetry(canister_id: &str) -> Result<SubCanisterTelemetry, String> {
    if canister_id.is_empty() {
        return Err("Canister ID is empty".to_string());
    }
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err("Canister principal not found".to_string());
    }
    let principal = principal_res.unwrap();

    ic_cdk::println!("Getting telemetry for canister {}", canister_id);

    let call_res: Result<(SubCanisterTelemetryResponse,), _> =
        ic_cdk::call(principal, "get_telemetry", ()).await;
    if call_res.is_err() {
        return Err(format!(
            "Error getting telemetry for canister {:?}",
            call_res.err().unwrap()
        ));
    }
    let call = call_res.unwrap();
    let tele_res = call.0;
    if !tele_res.err.is_empty() {
        return Err(tele_res.err);
    }
    if tele_res.ok.is_none() {
        return Err("Telemetry not found".to_string());
    }
    let telemetry = tele_res.ok.unwrap();

    return Ok(telemetry);
}
