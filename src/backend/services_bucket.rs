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

//
//    ##   #####  #####     ###### # ###### #      #####
//   #  #  #    # #    #    #      # #      #      #    #
//  #    # #    # #    #    #####  # #####  #      #    #
//  ###### #    # #    #    #      # #      #      #    #
//  #    # #    # #    #    #      # #      #      #    #
//  #    # #####  #####     #      # ###### ###### #####

pub async fn add_field_to_sub_canister(
    canister_id: String,
    input: ModelDataFieldType
) -> Result<(), String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "addField", (input,)).await;
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
//  #####  ###### #    #  ####  #    # ######    ###### # ###### #      #####  
//  #    # #      ##  ## #    # #    # #         #      # #      #      #    # 
//  #    # #####  # ## # #    # #    # #####     #####  # #####  #      #    # 
//  #####  #      #    # #    # #    # #         #      # #      #      #    # 
//  #   #  #      #    # #    #  #  #  #         #      # #      #      #    # 
//  #    # ###### #    #  ####    ##   ######    #      # ###### ###### #####  

pub async fn remove_field_from_sub_canister(
    canister_id: String,
    field_name: String,
) -> Result<(), String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let input: RemoveField = RemoveField {
        field_name,
    };

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "removeField", (input,)).await;
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
//  #####  #####    ##   # #    #     ####  #   #  ####  #      ######  ####  
//  #    # #    #  #  #  # ##   #    #    #  # #  #    # #      #      #      
//  #    # #    # #    # # # #  #    #        #   #      #      #####   ####  
//  #    # #####  ###### # #  # #    #        #   #      #      #           # 
//  #    # #   #  #    # # #   ##    #    #   #   #    # #      #      #    # 
//  #####  #    # #    # # #    #     ####    #    ####  ###### ######  ####  

pub async fn drain_sub_canister_cycles(
    canister_id: &str
) -> Result<(), String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let call_res: Result<((),), (RejectionCode, String)> =
        ic_cdk::call(principal, "wallet_drain", ()).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }

    Ok(())
}


//                                                                                   
//  #####  ####  #####     #    # #####      ####  #   #  ####  #      ######  ####  
//    #   #    # #    #    #    # #    #    #    #  # #  #    # #      #      #      
//    #   #    # #    #    #    # #    #    #        #   #      #      #####   ####  
//    #   #    # #####     #    # #####     #        #   #      #      #           # 
//    #   #    # #         #    # #         #    #   #   #    # #      #      #    # 
//    #    ####  #          ####  #          ####    #    ####  ###### ######  ####  

pub async fn top_up_sub_canister_cycles(
    canister_id: String,
    amount: u64,
) -> Result<(), String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let call_res: Result<((),), (RejectionCode, String)> =
        ic_cdk::call(principal, "wallet_receive", (amount,)).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }

    Ok(())
}

