use crate::main::*;
use crate::types::*;
use crate::utilities::*;
use ic_cdk::export::candid::{CandidType, Deserialize};

//
//   ####  ###### #####      ##   #    # ##### #    #
//  #    # #        #       #  #  #    #   #   #    #
//  #      #####    #      #    # #    #   #   ######
//  #  ### #        #      ###### #    #   #   #    #
//  #    # #        #      #    # #    #   #   #    #
//   ####  ######   #      #    #  ####    #   #    #

pub fn get_auth_info() -> Result<Authentication, String> {
    let mut auth_info: Authentication = Authentication::default();
    STATE.with(|state: &GlobalState| {
        let auth = state.auth.borrow_mut();
        auth_info = auth.clone();
    });
    if auth_info.password_hash.is_empty() {
        return Err("Authentication has not been set up.".to_string());
    }
    return Ok(auth_info);
}

//                                                                                                               
//  #    #   ##   #      # #####    ##   ##### ######     ####  ######  ####   ####  #  ####  #    #    # #####  
//  #    #  #  #  #      # #    #  #  #    #   #         #      #      #      #      # #    # ##   #    # #    # 
//  #    # #    # #      # #    # #    #   #   #####      ####  #####   ####   ####  # #    # # #  #    # #    # 
//  #    # ###### #      # #    # ######   #   #              # #           #      # # #    # #  # #    # #    # 
//   #  #  #    # #      # #    # #    #   #   #         #    # #      #    # #    # # #    # #   ##    # #    # 
//    ##   #    # ###### # #####  #    #   #   ######     ####  ######  ####   ####  #  ####  #    #    # #####  

pub fn validate_session_id(session_id: &str) -> Result<(), String> {
    let auth_info_res = get_auth_info();
    if auth_info_res.is_err() {
        return Err(auth_info_res.err().unwrap());
    }
    let auth_info = auth_info_res.unwrap();

    if auth_info.session_id == session_id {
        return Ok(());
    }
    return Err(String::from("Invalid token"));
}


//                                                                  
//  #####  ####  #    # ###### #    #      ##   #    # ##### #    # 
//    #   #    # #   #  #      ##   #     #  #  #    #   #   #    # 
//    #   #    # ####   #####  # #  #    #    # #    #   #   ###### 
//    #   #    # #  #   #      #  # #    ###### #    #   #   #    # 
//    #   #    # #   #  #      #   ##    #    # #    #   #   #    # 
//    #    ####  #    # ###### #    #    #    #  ####    #   #    # 

pub fn authenticate_token(token: &str) -> Result<(), String> {
  let auth_info_res = get_auth_info();
  if auth_info_res.is_err() {
    return Err(auth_info_res.err().unwrap());
  }

  let token_res = decode_id_from_token(&token);
  if token_res.is_err() {
    return Err(token_res.err().unwrap());
  }
  let session_id = token_res.unwrap();

  let session_res = validate_session_id(&session_id);
  if session_res.is_err() {
    return Err(session_res.err().unwrap());
  }
  return Ok(());
}