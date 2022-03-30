use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use serde::Serialize;

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
//  ##### ###### #      ###### #    # ###### ##### #####  #   #
//    #   #      #      #      ##  ## #        #   #    #  # #
//    #   #####  #      #####  # ## # #####    #   #    #   #
//    #   #      #      #      #    # #        #   #####    #
//    #   #      #      #      #    # #        #   #   #    #
//    #   ###### ###### ###### #    # ######   #   #    #   #

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

//
//  #    #  ####  #####  ###### #       ####
//  ##  ## #    # #    # #      #      #
//  # ## # #    # #    # #####  #       ####
//  #    # #    # #    # #      #           #
//  #    # #    # #    # #      #      #    #
//  #    #  ####  #####  ###### ######  ####

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CanisterId {
    pub canister_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitModel {
    pub model_name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RemoveField {
    pub field_name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Id {
    pub id: String,
}

// Data Types:
// "STRING"
// "NUMBER"
// "BOOLEAN"
// "STRING_ARRAY"
// "NUMBER_ARRAY"

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ModelDataFieldType {
    pub field_name: String,
    pub data_type: String,
    pub default_json_value: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ModelInstanceDataField {
    pub field_name: String,
    pub json_value: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ModelInstance {
    pub id: String,
    pub model_name: String,
    pub data_fields: Vec<ModelInstanceDataField>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ModelInstanceResponse {
    pub ok: Option<ModelInstance>,
    pub err: String,
}

impl Default for ModelInstanceResponse {
    fn default() -> Self {
        ModelInstanceResponse {
            ok: None,
            err: "".to_string(),
        }
    }
}
