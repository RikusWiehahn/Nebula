
use crate::helpers::*;
use crate::main::*;
use crate::types::*;
use crate::utilities::*;
use ic_cdk_macros::query;
use ic_cdk_macros::update;


//
//   ####  ###### ##### #    # #####
//  #      #        #   #    # #    #
//   ####  #####    #   #    # #    #
//       # #        #   #    # #####
//  #    # #        #   #    # #
//   ####  ######   #    ####  #

#[update(name = "setupAuthentication")]
pub async fn setup_authentication(password: String, password_check: String) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    if password.len() < 64 {
        res.err = "Password must be at least 64 characters".to_string();
        return res;
    }
    if password != password_check {
        res.err = "Passwords do not match".to_string();
        return res;
    }

    let mut auth_info: Authentication = Authentication::default();
    STATE.with(|state: &GlobalState| {
        let auth = state.auth.borrow();
        auth_info = auth.clone();
    });
    if auth_info.password_hash.len() > 0 {
        res.err = "Authentication already setup".to_string();
        return res;
    }

    let uuid_res = generate_uuid().await;
    if uuid_res.is_err() {
        res.err = "Error generating UUID".to_string();
        return res;
    }
    let session_id = uuid_res.unwrap();

    // hash the password
    let hash_res = hash_password(&password).await;
    if hash_res.is_err() {
        res.err = "Error hashing password".to_string();
        return res;
    }
    let hash = hash_res.unwrap();

    STATE.with(|state: &GlobalState| {
        let mut auth = state.auth.borrow_mut();
        auth.password_hash = hash;
        auth.session_id = session_id.clone();
    });

    // generate a token
    let jwt_res = generate_token_from_id(&session_id);
    if jwt_res.is_err() {
        res.err = "Error generating JWT".to_string();
        return res;
    }

    res.ok = Some(jwt_res.unwrap());
    return res;
}

//
//   ####  #  ####  #    #    # #    #
//  #      # #    # ##   #    # ##   #
//   ####  # #      # #  #    # # #  #
//       # # #  ### #  # #    # #  # #
//  #    # # #    # #   ##    # #   ##
//   ####  #  ####  #    #    # #    #

#[query(name = "signIn")]
pub async fn sign_in(password: String) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_info_res = get_auth_info();
    if auth_info_res.is_err() {
        res.err = auth_info_res.err().unwrap();
        return res;
    }
    let auth_info = auth_info_res.unwrap();
    
    if password.is_empty() {
        res.err = "Password is required".to_string();
        return res;
    }

    // verify password
    let verify_res = verify_password(&password, &auth_info.password_hash).await;
    if verify_res.is_err() {
        res.err = verify_res.err().unwrap();
        return res;
    };

    // generate a new session id
    let uuid_res = generate_uuid().await;
    if uuid_res.is_err() {
        res.err = "Error generating UUID".to_string();
        return res;
    }
    let session_id = uuid_res.unwrap();
    STATE.with(|state: &GlobalState| {
        let mut auth = state.auth.borrow_mut();
        auth.session_id = session_id.clone();
    });

    // generate a token
    let jwt_res = generate_token_from_id(&session_id);
    if jwt_res.is_err() {
        res.err = "Error generating JWT".to_string();
        return res;
    }

    res.ok = Some(jwt_res.unwrap());
    return res;
}

//
//   ####  #    #   ##   #    #  ####  ######    #####    ##    ####   ####  #    #  ####  #####  #####
//  #    # #    #  #  #  ##   # #    # #         #    #  #  #  #      #      #    # #    # #    # #    #
//  #      ###### #    # # #  # #      #####     #    # #    #  ####   ####  #    # #    # #    # #    #
//  #      #    # ###### #  # # #  ### #         #####  ######      #      # # ## # #    # #####  #    #
//  #    # #    # #    # #   ## #    # #         #      #    # #    # #    # ##  ## #    # #   #  #    #
//   ####  #    # #    # #    #  ####  ######    #      #    #  ####   ####  #    #  ####  #    # #####

#[update(name = "changePassword")]
pub async fn change_password(old_password: String, password: String, password_check: String) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_info_res = get_auth_info();
    if auth_info_res.is_err() {
        res.err = auth_info_res.err().unwrap();
        return res;
    }
    let auth_info = auth_info_res.unwrap();

    if old_password.is_empty() {
        res.err = "Current password is required".to_string();
        return res;
    }
    if password.len() < 64 {
        res.err = "New password must be at least 64 characters".to_string();
        return res;
    }
    if password != password_check {
        res.err = "New passwords do not match".to_string();
        return res;
    }

    // verify current password
    let verify_res = verify_password(&old_password, &auth_info.password_hash).await;
    if verify_res.is_err() {
        res.err = verify_res.err().unwrap();
        return res;
    };

    let uuid_res = generate_uuid().await;
    if uuid_res.is_err() {
        res.err = "Error generating UUID".to_string();
        return res;
    }
    let session_id = uuid_res.unwrap();

    // hash the password
    let hash_res = hash_password(&password).await;
    if hash_res.is_err() {
        res.err = "Error hashing password".to_string();
        return res;
    }
    let hash = hash_res.unwrap();

    // save the new password and roll the session id, they will have to sign in again
    STATE.with(|state: &GlobalState| {
        let mut auth = state.auth.borrow_mut();
        auth.password_hash = hash;
        auth.session_id = session_id.clone();
    });

    res.ok = Some("Password changed".to_string());
    return res;
}


//                                                                                
//    ##   #####  #####      ####    ##   #      #      ###### #####     # #####  
//   #  #  #    # #    #    #    #  #  #  #      #      #      #    #    # #    # 
//  #    # #    # #    #    #      #    # #      #      #####  #    #    # #    # 
//  ###### #    # #    #    #      ###### #      #      #      #####     # #    # 
//  #    # #    # #    #    #    # #    # #      #      #      #   #     # #    # 
//  #    # #####  #####      ####  #    # ###### ###### ###### #    #    # #####  

#[update(name = "addTrustedCallerId")]
pub async fn add_trusted_caller_id(token: String, caller_id: String) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_res = authenticate_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }

    if caller_id.is_empty() {
        res.err = "Caller ID is required".to_string();
        return res;
    }
    
    STATE.with(|state: &GlobalState| {
        let mut auth = state.auth.borrow_mut();
        auth.trusted_caller_ids.push(caller_id);
    });
    
    res.ok = Some("Caller ID added".to_string());
    return res;
}

//                                                                                                     
//  #####  ###### #    #  ####  #    # ######     ####    ##   #      #      ###### #####     # #####  
//  #    # #      ##  ## #    # #    # #         #    #  #  #  #      #      #      #    #    # #    # 
//  #    # #####  # ## # #    # #    # #####     #      #    # #      #      #####  #    #    # #    # 
//  #####  #      #    # #    # #    # #         #      ###### #      #      #      #####     # #    # 
//  #   #  #      #    # #    #  #  #  #         #    # #    # #      #      #      #   #     # #    # 
//  #    # ###### #    #  ####    ##   ######     ####  #    # ###### ###### ###### #    #    # #####  

#[update(name = "removeTrustedCallerId")]
pub async fn remove_trusted_caller_id(token: String, caller_id: String) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_res = authenticate_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }

    if caller_id.is_empty() {
        res.err = "Caller ID is required".to_string();
        return res;
    }
    
    let mut found = false;
    STATE.with(|state: &GlobalState| {
        let mut auth = state.auth.borrow_mut();
        for existing_caller_id in auth.trusted_caller_ids.iter() {
            if existing_caller_id == &caller_id {
                found = true;
                break;
            }
        }   
        auth.trusted_caller_ids.retain(|id| id != &caller_id);
    });

    if !found {
        res.err = "Caller ID not found".to_string();
        return res;
    }
    
    res.ok = Some("Caller ID removed".to_string());
    return res;
}