use crate::helpers::*;
use crate::main::*;
use crate::types::*;
use crate::utilities::*;
use ic_cdk_macros::update;

//
//    ##   #    # ##### #    #    #  ####      ####  ###### #####    #    # #####
//   #  #  #    #   #   #    #    # #         #      #        #      #    # #    #
//  #    # #    #   #   ######    #  ####      ####  #####    #      #    # #    #
//  ###### #    #   #   #    #    #      #         # #        #      #    # #####
//  #    # #    #   #   #    #    # #    #    #    # #        #      #    # #
//  #    #  ####    #   #    #    #  ####      ####  ######   #       ####  #

#[update(name = "isActivated")]
pub async fn is_auth_set() -> bool {
    let auth_info_res = get_auth_info();
    if auth_info_res.is_err() {
        return false;
    }
    return true;
}

//
//   ####  ###### ##### #    # #####
//  #      #        #   #    # #    #
//   ####  #####    #   #    # #    #
//       # #        #   #    # #####
//  #    # #        #   #    # #
//   ####  ######   #    ####  #

#[update(name = "activate")]
pub async fn set_auth(
    Activate {
        password,
        password_check,
        bucket_wasm,
    }: Activate,
) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    if password.len() < 64 {
        res.err = "Password must be at least 64 characters".to_string();
        return res;
    }
    if password != password_check {
        res.err = "Passwords do not match".to_string();
        return res;
    }
    if bucket_wasm.len() == 0 {
        res.err = "Bucket wasm is empty".to_string();
        return res;
    }

    let _ = init_jwt_key().await; // init jwt key if not already done
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
        let mut wasm = state.bucket_wasm.borrow_mut();
        auth.password_hash = hash;
        auth.session_id = session_id.clone();
        *wasm = bucket_wasm;

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

#[update(name = "signIn")]
pub async fn sign_in(SignIn { password }: SignIn) -> BasicResponse {
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
pub async fn change_password(
    ChangePassword {
        old_password,
        password,
        password_check,
    }: ChangePassword,
) -> BasicResponse {
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
//   ####  #    # ######  ####  #    #     ####  ######  ####   ####  #  ####  #    #
//  #    # #    # #      #    # #   #     #      #      #      #      # #    # ##   #
//  #      ###### #####  #      ####       ####  #####   ####   ####  # #    # # #  #
//  #      #    # #      #      #  #           # #           #      # # #    # #  # #
//  #    # #    # #      #    # #   #     #    # #      #    # #    # # #    # #   ##
//   ####  #    # ######  ####  #    #     ####  ######  ####   ####  #  ####  #    #

#[update(name = "checkSession")]
pub async fn check_session(TokenRecord { token }: TokenRecord) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_res = authenticate_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }

    res.ok = Some("Session is valid".to_string());
    return res;
}

//
//   ####  ###### #####    ##### #####  #    #  ####  ##### ###### #####      ####    ##   #    # #  ####  ##### ###### #####   ####
//  #    # #        #        #   #    # #    # #        #   #      #    #    #    #  #  #  ##   # # #        #   #      #    # #
//  #      #####    #        #   #    # #    #  ####    #   #####  #    #    #      #    # # #  # #  ####    #   #####  #    #  ####
//  #  ### #        #        #   #####  #    #      #   #   #      #    #    #      ###### #  # # #      #   #   #      #####       #
//  #    # #        #        #   #   #  #    # #    #   #   #      #    #    #    # #    # #   ## # #    #   #   #      #   #  #    #
//   ####  ######   #        #   #    #  ####   ####    #   ###### #####      ####  #    # #    # #  ####    #   ###### #    #  ####

#[update(name = "getTrustedCanisters")]
pub async fn get_trusted_canisters(TokenRecord { token }: TokenRecord) -> TrustedCanistersResponse {
    let mut res: TrustedCanistersResponse = TrustedCanistersResponse::default();
    let auth_res = authenticate_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }

    let trusted_canisters_res = find_trusted_canisters();
    if trusted_canisters_res.is_err() {
        res.err = trusted_canisters_res.err().unwrap();
        return res;
    }
    res.ok = trusted_canisters_res.unwrap();
    return res;
}

//
//    ##   #####  #####  ###### #####     ##### #####  #    #  ####  ##### ###### #####      ####    ##   #    # #  ####  ##### ###### #####
//   #  #  #    # #    # #      #    #      #   #    # #    # #        #   #      #    #    #    #  #  #  ##   # # #        #   #      #    #
//  #    # #    # #    # #####  #    #      #   #    # #    #  ####    #   #####  #    #    #      #    # # #  # #  ####    #   #####  #    #
//  ###### #    # #    # #      #    #      #   #####  #    #      #   #   #      #    #    #      ###### #  # # #      #   #   #      #####
//  #    # #    # #    # #      #    #      #   #   #  #    # #    #   #   #      #    #    #    # #    # #   ## # #    #   #   #      #   #
//  #    # #####  #####  ###### #####       #   #    #  ####   ####    #   ###### #####      ####  #    # #    # #  ####    #   ###### #    #

#[update(name = "addTrustedCanisterId")]
pub async fn add_trusted_canister_id(
    AddOrRemoveTrustedCanister { token, canister_id }: AddOrRemoveTrustedCanister,
) -> TrustedCanistersResponse {
    let mut res: TrustedCanistersResponse = TrustedCanistersResponse::default();
    let auth_res = authenticate_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }

    if canister_id.is_empty() {
        res.err = "Canister ID is required".to_string();
        return res;
    }

    STATE.with(|state: &GlobalState| {
        let mut auth = state.auth.borrow_mut();
        auth.trusted_canister_ids.push(canister_id);
    });

    let trusted_canisters_res = find_trusted_canisters();
    if trusted_canisters_res.is_err() {
        res.err = trusted_canisters_res.err().unwrap();
        return res;
    }
    res.ok = trusted_canisters_res.unwrap();
    return res;
}

//
//  #####  ###### #    #  ####  #    # ######    ##### #####  #    #  ####  ##### ###### #####      ####    ##   #    # #  ####  ##### ###### #####
//  #    # #      ##  ## #    # #    # #           #   #    # #    # #        #   #      #    #    #    #  #  #  ##   # # #        #   #      #    #
//  #    # #####  # ## # #    # #    # #####       #   #    # #    #  ####    #   #####  #    #    #      #    # # #  # #  ####    #   #####  #    #
//  #####  #      #    # #    # #    # #           #   #####  #    #      #   #   #      #    #    #      ###### #  # # #      #   #   #      #####
//  #   #  #      #    # #    #  #  #  #           #   #   #  #    # #    #   #   #      #    #    #    # #    # #   ## # #    #   #   #      #   #
//  #    # ###### #    #  ####    ##   ######      #   #    #  ####   ####    #   ###### #####      ####  #    # #    # #  ####    #   ###### #    #

#[update(name = "removeTrustedCanisterId")]
pub async fn remove_trusted_canister_id(
    AddOrRemoveTrustedCanister { token, canister_id }: AddOrRemoveTrustedCanister,
) -> TrustedCanistersResponse {
    let mut res: TrustedCanistersResponse = TrustedCanistersResponse::default();
    let auth_res = authenticate_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }

    if canister_id.is_empty() {
        res.err = "Canister ID is required".to_string();
        return res;
    }

    let mut found = false;
    STATE.with(|state: &GlobalState| {
        let mut auth = state.auth.borrow_mut();
        for existing_canister_id in auth.trusted_canister_ids.iter() {
            if existing_canister_id == &canister_id {
                found = true;
                break;
            }
        }
        auth.trusted_canister_ids.retain(|id| id != &canister_id);
    });

    if !found {
        res.err = "Canister ID not found".to_string();
        return res;
    }

    let trusted_canisters_res = find_trusted_canisters();
    if trusted_canisters_res.is_err() {
        res.err = trusted_canisters_res.err().unwrap();
        return res;
    }
    res.ok = trusted_canisters_res.unwrap();
    return res;
}
