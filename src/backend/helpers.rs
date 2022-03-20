use crate::main::*;
use crate::types::*;
use crate::utilities::*;
use ic_cdk::export::Principal;
use ic_cdk::export::candid::CandidType;
use serde::Deserialize;
use ic_cdk::export::candid::Nat;

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

//                                                                                                
//   ####  #####  ######   ##   ##### ######     ####    ##   #    # #  ####  ##### ###### #####  
//  #    # #    # #       #  #    #   #         #    #  #  #  ##   # # #        #   #      #    # 
//  #      #    # #####  #    #   #   #####     #      #    # # #  # #  ####    #   #####  #    # 
//  #      #####  #      ######   #   #         #      ###### #  # # #      #   #   #      #####  
//  #    # #   #  #      #    #   #   #         #    # #    # #   ## # #    #   #   #      #   #  
//   ####  #    # ###### #    #   #   ######     ####  #    # #    # #  ####    #   ###### #    # 

pub async fn create_and_install(
    wasm_module: Vec<u8>,
    cycles_to_use: u64,
) -> Result<String, String> {
    let create_res = create_canister(cycles_to_use).await;
    

    match install(canister_id, wasm_module, wasm_arg).await {
        Err(error) => Err(CreateAndInstallError::InstallFailed((error, canister_id))),
        Ok(_) => Ok(canister_id),
    }
}

pub async fn create_canister(cycles_to_use: u64) -> Result<Principal, String> {
    #[derive(CandidType, Clone, Deserialize)]
    struct CanisterSettings {
        controller: Option<Principal>,
        compute_allocation: Option<Nat>,
        memory_allocation: Option<Nat>,
        freezing_threshold: Option<Nat>,
    }

    #[derive(CandidType)]
    struct In {
        settings: Option<CanisterSettings>,
    }

    #[derive(CandidType, Deserialize)]
    struct CreateResult {
        canister_id: Principal,
    }

    let in_arg = In {
        settings: Some(CanisterSettings {
            controller: Some(ic_cdk::id()),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        }),
    };

    let (create_result,): (CreateResult,) = match api::call::call_with_payment(
        Principal::management_canister(),
        "create_canister",
        (in_arg,),
        cycles_to_use.try_into().unwrap(),
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            let code = code as u8;
            error!(
                error_code = code,
                error_message = msg.as_str(),
                "Error calling create_canister"
            );

            return Err(canister::Error { code, msg });
        }
    };

    Ok(create_result.canister_id)
}

pub async fn install(canister_id: CanisterId, wasm_module: Vec<u8>, wasm_arg: Vec<u8>) -> Result<(), canister::Error> {
    #[derive(CandidType, Deserialize)]
    enum InstallMode {
        #[serde(rename = "install")]
        Install,
        #[serde(rename = "reinstall")]
        Reinstall,
        #[serde(rename = "upgrade")]
        Upgrade,
    }

    #[derive(CandidType, Deserialize)]
    struct CanisterInstall {
        mode: InstallMode,
        canister_id: Principal,
        #[serde(with = "serde_bytes")]
        wasm_module: Vec<u8>,
        #[serde(with = "serde_bytes")]
        arg: Vec<u8>,
    }

    let install_config = CanisterInstall {
        mode: InstallMode::Install,
        canister_id,
        wasm_module,
        arg: wasm_arg,
    };

    let (_,): ((),) = match api::call::call(Principal::management_canister(), "install_code", (install_config,)).await {
        Ok(x) => x,
        Err((code, msg)) => {
            let code = code as u8;
            error!(error_code = code, error_message = msg.as_str(), "Error calling install_code");
            return Err(canister::Error { code, msg });
        }
    };

    Ok(())
}
