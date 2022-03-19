use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct Authentication {
    pub session_id: String,
    pub password_hash: String,
    pub trusted_caller_ids: Vec<String>,
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
