use crate::helpers::*;
use crate::main::*;
use crate::services_telemetry::auto_update_telemetry;
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

#[update]
pub async fn is_activated() -> bool {
    let auth_info_res = find_auth_info();
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

#[update]
pub async fn activate(
    ActivationRequest {
        password,
        password_check,
        bucket_wasm,
    }: ActivationRequest,
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

    // init telemetry
    let _ = auto_update_telemetry().await;

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

#[update]
pub async fn sign_in(SignInRequest { password }: SignInRequest) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_info_res = find_auth_info();
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

#[update]
pub async fn change_password(
    ChangePasswordRequest {
        old_password,
        password,
        password_check,
    }: ChangePasswordRequest,
) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_info_res = find_auth_info();
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

#[update]
pub async fn check_session(TokenRequest { token }: TokenRequest) -> BasicResponse {
    let mut res: BasicResponse = BasicResponse::default();
    let auth_res = validate_auth_token(&token);
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

#[update]
pub async fn get_trusted_canisters(
    TokenRequest { token }: TokenRequest,
) -> TrustedCanistersResponse {
    let mut res: TrustedCanistersResponse = TrustedCanistersResponse::default();
    let auth_res = validate_auth_token(&token);
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
//    ##   #####  #####     ##### #####  #    #  ####  ##### ###### #####      ####    ##   #    # #  ####  ##### ###### #####
//   #  #  #    # #    #      #   #    # #    # #        #   #      #    #    #    #  #  #  ##   # # #        #   #      #    #
//  #    # #    # #    #      #   #    # #    #  ####    #   #####  #    #    #      #    # # #  # #  ####    #   #####  #    #
//  ###### #    # #    #      #   #####  #    #      #   #   #      #    #    #      ###### #  # # #      #   #   #      #####
//  #    # #    # #    #      #   #   #  #    # #    #   #   #      #    #    #    # #    # #   ## # #    #   #   #      #   #
//  #    # #####  #####       #   #    #  ####   ####    #   ###### #####      ####  #    # #    # #  ####    #   ###### #    #

#[update]
pub async fn add_trusted_canister(
    AddTrustedCanisterRequest {
        token,
        name,
        canister_id,
    }: AddTrustedCanisterRequest,
) -> TrustedCanistersResponse {
    let mut res: TrustedCanistersResponse = TrustedCanistersResponse::default();
    let auth_res = validate_auth_token(&token);
    if auth_res.is_err() {
        res.err = auth_res.err().unwrap();
        return res;
    }

    if name.is_empty() {
        res.err = "A canister name is required".to_string();
        return res;
    }
    if canister_id.is_empty() {
        res.err = "Canister ID is required".to_string();
        return res;
    }

    let to_check_res = find_trusted_canisters();
    if to_check_res.is_err() {
        res.err = to_check_res.err().unwrap();
        return res;
    }
    let to_check = to_check_res.unwrap();
    let already_exists = to_check.iter().any(|x| x.canister_id == canister_id);
    if already_exists {
        res.err = "Canister ID already added".to_string();
        return res;
    }

    STATE.with(|state: &GlobalState| {
        let mut auth = state.auth.borrow_mut();
        auth.trusted_canisters.push(TrustedCanister {
            name: name.clone(),
            canister_id: canister_id.clone(),
        });
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

#[update]
pub async fn remove_trusted_canister(
    RemoveTrustedCanisterRequest { token, canister_id }: RemoveTrustedCanisterRequest,
) -> TrustedCanistersResponse {
    let mut res: TrustedCanistersResponse = TrustedCanistersResponse::default();
    let auth_res = validate_auth_token(&token);
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
        for existing_canister in auth.trusted_canisters.iter() {
            if existing_canister.canister_id == canister_id {
                found = true;
                break;
            }
        }
        auth.trusted_canisters
            .retain(|canister| canister.canister_id != canister_id);
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
