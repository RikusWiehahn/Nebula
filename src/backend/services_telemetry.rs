use crate::main::*;
use crate::types::*;
use ic_cdk::api::*;
use ic_cdk_macros::update;


// // get management canister
// let management_canister = Principal::from_text("aaaaa-aa").unwrap();
//   let canister_id_record = CanisterIdRecord {
//     canister_id: main_canister.clone(),
//   };
//   let self_status_res: Result<(String,), _> =
//       call::call(management_canister, "canister_status", (canister_id_record,)).await;
//   let self_res_string = format!("{:?}", self_status_res);
//   ic_cdk::println!("{}", self_res_string);

//                                                                         
//    ##   #    # #####  ####     #    # #####  #####    ##   ##### ###### 
//   #  #  #    #   #   #    #    #    # #    # #    #  #  #    #   #      
//  #    # #    #   #   #    #    #    # #    # #    # #    #   #   #####  
//  ###### #    #   #   #    #    #    # #####  #    # ######   #   #      
//  #    # #    #   #   #    #    #    # #      #    # #    #   #   #      
//  #    #  ####    #    ####      ####  #      #####  #    #   #   ###### 

pub async fn auto_update_telemetry() {
    // get own identity
    let main_canister = ic_cdk::id();
    let cycles_available = ic_cdk::api::canister_balance();
    let mem_size = ic_cdk::api::stable::stable_bytes();
    let mem_buff: &mut [u8] = &mut [0; 64];
    ic_cdk::api::stable::stable64_read(0, mem_buff);
    let memory_used = mem_buff.len() as u64;
    let memory_size = mem_size.len() as u64;

    let new_telemetry = Telemetry {
        last_status_check: time() as f64,
        main_id: main_canister.to_string(),
        main_memory_size: memory_size as f64,
        main_memory_used: memory_used as f64,
        main_cycles: cycles_available as f64,
        sub_canisters: vec![],
    };

    STATE.with(|state: &GlobalState| {
        let mut telemetry = state.telemetry.borrow_mut();
        *telemetry = new_telemetry;
    });
}


//                                                                 
//   ####  ###### #####     ####  #####   ##   ##### #    #  ####  
//  #    # #        #      #        #    #  #    #   #    # #      
//  #      #####    #       ####    #   #    #   #   #    #  ####  
//  #  ### #        #           #   #   ######   #   #    #      # 
//  #    # #        #      #    #   #   #    #   #   #    # #    # 
//   ####  ######   #       ####    #   #    #   #    ####   ####  


#[update(name = "getSystemTelemetry")]
pub async fn get_telemetry() -> TelemetryResponse {
    let mut res = TelemetryResponse::default();
    let telemetry = STATE.with(|s: &GlobalState| s.telemetry.borrow().clone());

    res.ok = Some(telemetry);
    return res;
}
