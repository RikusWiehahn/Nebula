use ic_cdk::export::candid::{CandidType, Deserialize};

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
//  #    #  ####  #####  ###### #       ####
//  ##  ## #    # #    # #      #      #
//  # ## # #    # #    # #####  #       ####
//  #    # #    # #    # #      #           #
//  #    # #    # #    # #      #      #    #
//  #    #  ####  #####  ###### ######  ####

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CanisterId {
    pub canister_id: String,
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

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct CreateOrUpdateInstance {
    pub id: String,
    pub json: String,
}

// Data Types:
// "STRING"
// "NUMBER"
// "BOOLEAN"
// "STRING_ARRAY"
// "NUMBER_ARRAY"

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ModelDataFieldType {
    pub field_name: String,
    pub data_type: String,
    pub default_json_value: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ModelDataField {
    pub field_name: String,
    pub json_value: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ModelInstance {
    pub id: String,
    pub model_name: String,
    pub data_fields: Vec<ModelDataField>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct ModelInstanceResponse {
    pub json: Option<String>,
    pub err: String,
}

impl Default for ModelInstanceResponse {
    fn default() -> Self {
        ModelInstanceResponse {
            json: None,
            err: "".to_string(),
        }
    }
}
