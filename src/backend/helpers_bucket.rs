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
    let input = CanisterIdRecord {
        canister_id: main_canister,
    };

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(target_canister_id, "set_admin_canister", (input,)).await;
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

pub async fn initialize_sub_canister_model(
    target_canister_id: Principal,
    model_name: String,
) -> Result<(), String> {
    let input = InitBucketModelRequest {
        model_name: model_name,
    };

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(target_canister_id, "init_model", (input,)).await;
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
    input: ModelDataFieldType,
) -> Result<(), String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "add_field", (input,)).await;
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

    let input: RemoveBucketFieldRequest = RemoveBucketFieldRequest { field_name };

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "remove_field", (input,)).await;
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
//  #####  ####  #####     #    # #####      ####  #   #  ####  #      ######  ####
//    #   #    # #    #    #    # #    #    #    #  # #  #    # #      #      #
//    #   #    # #    #    #    # #    #    #        #   #      #      #####   ####
//    #   #    # #####     #    # #####     #        #   #      #      #           #
//    #   #    # #         #    # #         #    #   #   #    # #      #      #    #
//    #    ####  #          ####  #          ####    #    ####  ###### ######  ####

pub async fn top_up_sub_canister_cycles(canister_id: String, amount: u64) -> Result<(), String> {
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

//                                                                                   
//  # #    #  ####  ###### #####  #####    #####  ######  ####   ####  #####  #####  
//  # ##   # #      #      #    #   #      #    # #      #    # #    # #    # #    # 
//  # # #  #  ####  #####  #    #   #      #    # #####  #      #    # #    # #    # 
//  # #  # #      # #      #####    #      #####  #      #      #    # #####  #    # 
//  # #   ## #    # #      #   #    #      #   #  #      #    # #    # #   #  #    # 
//  # #    #  ####  ###### #    #   #      #    # ######  ####   ####  #    # #####  

pub async fn insert_record_into_sub_canister(
    canister_id: String,
    input: Record,
) -> Result<Record, String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let call_res: Result<(RecordResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "insert_record", (input,)).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }
    
    let (record_res,) = call_res.unwrap();
    if !record_res.err.is_empty() {
        return Err(format!("{:?}", record_res.err));
    }
    if record_res.ok.is_none() {
        return Err(format!("{:?}", "No record returned"));
    }

    Ok(record_res.ok.unwrap())
}

//                                                                                        
//  #    # #####  #####    ##   ##### ######    #####  ######  ####   ####  #####  #####  
//  #    # #    # #    #  #  #    #   #         #    # #      #    # #    # #    # #    # 
//  #    # #    # #    # #    #   #   #####     #    # #####  #      #    # #    # #    # 
//  #    # #####  #    # ######   #   #         #####  #      #      #    # #####  #    # 
//  #    # #      #    # #    #   #   #         #   #  #      #    # #    # #   #  #    # 
//   ####  #      #####  #    #   #   ######    #    # ######  ####   ####  #    # #####  

pub async fn update_record_in_sub_canister(
    canister_id: String,
    input: Record,
) -> Result<Record, String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let call_res: Result<(RecordResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "update_record", (input,)).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }

    let (record_res,) = call_res.unwrap();
    if !record_res.err.is_empty() {
        return Err(format!("{:?}", record_res.err));
    }
    if record_res.ok.is_none() {
        return Err(format!("{:?}", "No record returned"));
    }

    Ok(record_res.ok.unwrap())
}

//                                                                      
//  ###### # #    # #####     #####  ######  ####   ####  #####  #####  
//  #      # ##   # #    #    #    # #      #    # #    # #    # #    # 
//  #####  # # #  # #    #    #    # #####  #      #    # #    # #    # 
//  #      # #  # # #    #    #####  #      #      #    # #####  #    # 
//  #      # #   ## #    #    #   #  #      #    # #    # #   #  #    # 
//  #      # #    # #####     #    # ######  ####   ####  #    # #####  

pub async fn find_record_in_sub_canister(
    canister_id: String,
    id: String,
) -> Result<Record, String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let input = BucketRecordRequest { id: id };

    let call_res: Result<(RecordResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "get_record", (input,)).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }

    let (record_res,) = call_res.unwrap();
    if !record_res.err.is_empty() {
        return Err(format!("{:?}", record_res.err));
    }
    if record_res.ok.is_none() {
        return Err(format!("{:?}", "No record returned"));
    }

    Ok(record_res.ok.unwrap())
}


//                                                                            
//  #####  ######  ####   ####  #####  #####   ####     #      #  ####  ##### 
//  #    # #      #    # #    # #    # #    # #         #      # #        #   
//  #    # #####  #      #    # #    # #    #  ####     #      #  ####    #   
//  #####  #      #      #    # #####  #    #      #    #      #      #   #   
//  #   #  #      #    # #    # #   #  #    # #    #    #      # #    #   #   
//  #    # ######  ####   ####  #    # #####   ####     ###### #  ####    #   

pub async fn find_sub_canister_records_list(
    canister_id: String,
    page: f64,
    page_size: f64,
) -> Result<RecordListResponse, String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let input = BucketRecordListRequest {
        page: page,
        page_size: page_size,
    };

    let call_res: Result<(RecordListResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "get_record", (input,)).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }

    let (record_res,) = call_res.unwrap();
    if !record_res.err.is_empty() {
        return Err(format!("{:?}", record_res.err));
    }

    Ok(record_res)
}

//                                                                                        
//  #####  ###### #      ###### ##### ######    #####  ######  ####   ####  #####  #####  
//  #    # #      #      #        #   #         #    # #      #    # #    # #    # #    # 
//  #    # #####  #      #####    #   #####     #    # #####  #      #    # #    # #    # 
//  #    # #      #      #        #   #         #####  #      #      #    # #####  #    # 
//  #    # #      #      #        #   #         #   #  #      #    # #    # #   #  #    # 
//  #####  ###### ###### ######   #   ######    #    # ######  ####   ####  #    # #####  

pub async fn delete_record_in_sub_canister(
    canister_id: String,
    id: String,
) -> Result<(), String> {
    let principal_res = Principal::from_text(canister_id);
    if principal_res.is_err() {
        return Err(format!("{:?}", principal_res.err().unwrap()));
    }
    let principal = principal_res.unwrap();

    let input = BucketRecordRequest { id: id };

    let call_res: Result<(BasicResponse,), (RejectionCode, String)> =
        ic_cdk::call(principal, "delete_record", (input,)).await;
    if call_res.is_err() {
        return Err(format!("{:?}", call_res.err().unwrap()));
    }

    let (record_res,) = call_res.unwrap();
    if !record_res.err.is_empty() {
        return Err(format!("{:?}", record_res.err));
    }
    Ok(())
}
