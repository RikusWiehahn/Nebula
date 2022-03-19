use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Authentication {
    pub session_id: String,
    pub password_hash: String,
    pub trusted_canister_ids: Vec<String>,
}

impl Default for Authentication {
    fn default() -> Self {
        Authentication {
            session_id: "".to_string(),
            password_hash: "".to_string(),
            trusted_canister_ids: vec![],
        }
    }
}

//
//   ####    ##   #    # #  ####  ##### ###### #####      ####  ###### ##### ##### # #    #  ####   ####
//  #    #  #  #  ##   # # #        #   #      #    #    #      #        #     #   # ##   # #    # #
//  #      #    # # #  # #  ####    #   #####  #    #     ####  #####    #     #   # # #  # #       ####
//  #      ###### #  # # #      #   #   #      #####          # #        #     #   # #  # # #  ###      #
//  #    # #    # #   ## # #    #   #   #      #   #     #    # #        #     #   # #   ## #    # #    #
//   ####  #    # #    # #  ####    #   ###### #    #     ####  ######   #     #   # #    #  ####   ####

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CanisterIdRecord {
    pub canister_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<String>>,
    pub compute_allocation: f64,
    pub memory_allocation: f64,
    pub freezing_threshold: f64,
}

impl Default for CanisterSettings {
    fn default() -> Self {
        CanisterSettings {
            controllers: None,
            compute_allocation: 0.0,
            memory_allocation: 0.0,
            freezing_threshold: 0.0,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct DefiniteCanisterSettings {
    pub controllers: Vec<String>,
    pub compute_allocation: f64,
    pub memory_allocation: f64,
    pub freezing_threshold: f64,
}

impl Default for DefiniteCanisterSettings {
    fn default() -> Self {
        DefiniteCanisterSettings {
            controllers: vec![],
            compute_allocation: 0.0,
            memory_allocation: 0.0,
            freezing_threshold: 0.0,
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
            sub_canisters: vec![],
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct SubCanisterTelemetry {
    pub last_status_check: f64,
    pub id: String,
    pub settings: DefiniteCanisterSettings,
    pub status: String,
    pub module_hash: String,
    pub memory_size: f64,
    pub cycles: f64,
}

impl Default for SubCanisterTelemetry {
    fn default() -> Self {
        SubCanisterTelemetry {
            last_status_check: 0.0,
            id: "".to_string(),
            settings: DefiniteCanisterSettings::default(),
            status: "".to_string(),
            module_hash: "".to_string(),
            memory_size: 0.0,
            cycles: 0.0,
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
