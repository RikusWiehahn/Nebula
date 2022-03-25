use ic_cdk::export::{
    candid::{CandidType, Deserialize, Nat},
    Principal,
};
use serde::Serialize;

//
//  #####    ##    ####  #  ####
//  #    #  #  #  #      # #    #
//  #####  #    #  ####  # #
//  #    # ######      # # #
//  #    # #    # #    # # #    #
//  #####  #    #  ####  #  ####

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BasicResponse {
    pub ok: Option<String>,
    pub err: String,
}

impl Default for BasicResponse {
    fn default() -> Self {
        BasicResponse {
            ok: None,
            err: "".to_string(),
        }
    }
}

//
//    ##   #    # ##### #    #
//   #  #  #    #   #   #    #
//  #    # #    #   #   ######
//  ###### #    #   #   #    #
//  #    # #    #   #   #    #
//  #    #  ####    #   #    #

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    pub id: String,
    pub exp: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Authentication {
    pub session_id: String,
    pub password_hash: String,
    pub trusted_canisters: Vec<TrustedCanister>,
}

impl Default for Authentication {
    fn default() -> Self {
        Authentication {
            session_id: "".to_string(),
            password_hash: "".to_string(),
            trusted_canisters: vec![],
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Activate {
    pub password: String,
    pub password_check: String,
    pub bucket_wasm: Vec<u8>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct SignIn {
    pub password: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ChangePassword {
    pub old_password: String,
    pub password: String,
    pub password_check: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TokenRecord {
    pub token: String,
}

//
//   ####    ##   #    # #  ####  ##### ###### #####      ####  ###### ##### ##### # #    #  ####   ####
//  #    #  #  #  ##   # # #        #   #      #    #    #      #        #     #   # ##   # #    # #
//  #      #    # # #  # #  ####    #   #####  #    #     ####  #####    #     #   # # #  # #       ####
//  #      ###### #  # # #      #   #   #      #####          # #        #     #   # #  # # #  ###      #
//  #    # #    # #   ## # #    #   #   #      #   #     #    # #        #     #   # #   ## #    # #    #
//   ####  #    # #    # #  ####    #   ###### #    #     ####  ######   #     #   # #    #  ####   ####

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CanisterId {
    pub canister_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<u64>,
    pub memory_allocation: Option<u64>,
    pub freezing_threshold: Option<u64>,
}

impl Default for CanisterSettings {
    fn default() -> Self {
        CanisterSettings {
            controllers: None,
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        }
    }
}

//
//  ##### ###### #      ###### #    # ###### ##### #####  #   #
//    #   #      #      #      ##  ## #        #   #    #  # #
//    #   #####  #      #####  # ## # #####    #   #    #   #
//    #   #      #      #      #    # #        #   #####    #
//    #   #      #      #      #    # #        #   #   #    #
//    #   ###### ###### ###### #    # ######   #   #    #   #
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Telemetry {
    pub last_status_check: f64,
    pub main_id: String,
    pub main_memory_size: f64,
    pub main_memory_used: f64,
    pub main_cycles: f64,
    pub bucket_wasm_size: f64,
    pub sub_canisters: Vec<SubCanisterTelemetry>,
}

impl Default for Telemetry {
    fn default() -> Self {
        Telemetry {
            last_status_check: 0.0,
            main_id: "".to_string(),
            main_memory_size: 0.0,
            main_memory_used: 0.0,
            main_cycles: 0.0,
            bucket_wasm_size: 0.0,
            sub_canisters: vec![],
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct SubCanisterTelemetry {
    pub id: String,
    pub model_name: String,
    pub memory_size: f64,
    pub memory_used: f64,
    pub cycles: f64,
}

impl Default for SubCanisterTelemetry {
    fn default() -> Self {
        SubCanisterTelemetry {
            id: "".to_string(),
            model_name: "".to_string(),
            memory_size: 0.0,
            memory_used: 0.0,
            cycles: 0.0,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct SubCanisterTelemetryResponse {
    pub ok: Option<SubCanisterTelemetry>,
    pub err: String,
}

impl Default for SubCanisterTelemetryResponse {
    fn default() -> Self {
        SubCanisterTelemetryResponse {
            ok: None,
            err: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TelemetryResponse {
    pub ok: Option<Telemetry>,
    pub err: String,
}

impl Default for TelemetryResponse {
    fn default() -> Self {
        TelemetryResponse {
            ok: None,
            err: "".to_string(),
        }
    }
}

//
//  #    #  ####  #####  ###### #       ####
//  ##  ## #    # #    # #      #      #
//  # ## # #    # #    # #####  #       ####
//  #    # #    # #    # #      #           #
//  #    # #    # #    # #      #      #    #
//  #    #  ####  #####  ###### ######  ####

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CreateOrGetModel {
    pub token: String,
    pub model_name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct RemoveModelField {
    pub token: String,
    pub model_name: String,
    pub field_name: String,
}

#[derive(Clone, Debug, CandidType, Default, Deserialize, Serialize)]
pub struct ModelDataFieldType {
    pub field_name: String,
    pub data_type: String,
    pub default_json_value: String,
}

#[derive(Clone, Debug, CandidType, Default, Deserialize, Serialize)]
pub struct Model {
    pub model_name: String,
    pub data_fields: Vec<ModelDataFieldType>,
    pub canisters: Vec<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ModelResponse {
    pub ok: Option<Model>,
    pub err: String,
}

impl Default for ModelResponse {
    fn default() -> Self {
        ModelResponse {
            ok: None,
            err: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ModelListResponse {
    pub ok: Vec<Model>,
    pub err: String,
}

impl Default for ModelListResponse {
    fn default() -> Self {
        ModelListResponse {
            ok: vec![],
            err: "".to_string(),
        }
    }
}

//
//  # #    #  ####  #####   ##   #      #
//  # ##   # #        #    #  #  #      #
//  # # #  #  ####    #   #    # #      #
//  # #  # #      #   #   ###### #      #
//  # #   ## #    #   #   #    # #      #
//  # #    #  ####    #   #    # ###### ######

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CanisterWasm {
    pub module: Vec<u8>,
}

//
//  #####  #    #  ####  #    # ###### #####  ####
//  #    # #    # #    # #   #  #        #   #
//  #####  #    # #      ####   #####    #    ####
//  #    # #    # #      #  #   #        #        #
//  #    # #    # #    # #   #  #        #   #    #
//  #####   ####   ####  #    # ######   #    ####

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct InitModel {
    pub model_name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct RemoveField {
    pub field_name: String,
}

//
//  ##### #####  #    #  ####  ##### ###### #####      ####    ##   #    # #  ####  ##### ###### #####   ####
//    #   #    # #    # #        #   #      #    #    #    #  #  #  ##   # # #        #   #      #    # #
//    #   #    # #    #  ####    #   #####  #    #    #      #    # # #  # #  ####    #   #####  #    #  ####
//    #   #####  #    #      #   #   #      #    #    #      ###### #  # # #      #   #   #      #####       #
//    #   #   #  #    # #    #   #   #      #    #    #    # #    # #   ## # #    #   #   #      #   #  #    #
//    #   #    #  ####   ####    #   ###### #####      ####  #    # #    # #  ####    #   ###### #    #  ####

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct TrustedCanister {
    pub name: String,
    pub canister_id: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TrustedCanistersResponse {
    pub ok: Vec<TrustedCanister>,
    pub err: String,
}

impl Default for TrustedCanistersResponse {
    fn default() -> Self {
        TrustedCanistersResponse {
            ok: vec![],
            err: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AddTrustedCanister {
    pub token: String,
    pub name: String,
    pub canister_id: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct RemoveTrustedCanister {
    pub token: String,
    pub canister_id: String,
}