use crate::types::*;
use ic_cdk;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::Principal;


//
//   ####  ###### #####      ##   #####  #    # # #    #
//  #      #        #       #  #  #    # ##  ## # ##   #
//   ####  #####    #      #    # #    # # ## # # # #  #
//       # #        #      ###### #    # #    # # #  # #
//  #    # #        #      #    # #    # #    # # #   ##
//   ####  ######   #      #    # #####  #    # # #    #

pub async fn set_admin_canister(target_canister_id: Principal) -> Result<(), String> {
    let main_canister = ic_cdk::id();
    let input = CanisterId {
        canister_id: main_canister,
    };

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(target_canister_id, "setAdminCanister", (input,)).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }
    let (basic_res,) = call_res.unwrap();
    if !basic_res.err.is_empty() {
        return Err(format!("{:?}", basic_res.err));
    }

    Ok(())
}

//
//  # #    # # #####    #    #  ####  #####  ###### #
//  # ##   # #   #      ##  ## #    # #    # #      #
//  # # #  # #   #      # ## # #    # #    # #####  #
//  # #  # # #   #      #    # #    # #    # #      #
//  # #   ## #   #      #    # #    # #    # #      #
//  # #    # #   #      #    #  ####  #####  ###### ######

pub async fn initialize_canister_model(
    target_canister_id: Principal,
    model_name: String,
) -> Result<(), String> {
    let input = InitModel {
        model_name: model_name,
    };

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(target_canister_id, "initModel", (input,)).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }

    let (basic_res,) = call_res.unwrap();
    if !basic_res.err.is_empty() {
        return Err(format!("{:?}", basic_res.err));
    }

    Ok(())
}
