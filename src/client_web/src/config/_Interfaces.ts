//
//   ####   ####  #    # ###### #  ####
//  #    # #    # ##   # #      # #    #
//  #      #    # # #  # #####  # #
//  #      #    # #  # # #      # #  ###
//  #    # #    # #   ## #      # #    #
//   ####   ####  #    # #      #  ####

import { Principal } from "@dfinity/principal";
import * as Backend from "../../../declarations/backend/backend.did";

export interface ConfigState {
  uiMode: "light" | "dark";
  uiModeWasManuallySet: boolean;
}

export const EMPTY_CONFIG_STATE: ConfigState = {
  uiMode: "light",
  uiModeWasManuallySet: false,
};

//
//    ##   #    # ##### #    #
//   #  #  #    #   #   #    #
//  #    # #    #   #   ######
//  ###### #    #   #   #    #
//  #    # #    #   #   #    #
//  #    #  ####    #   #    #
export interface AuthSession {
  token: string;
}

export const EMPTY_AUTH_SESSION: AuthSession = {
  token: "",
};

//                                                              
//  ##### ###### #      ###### #    # ###### ##### #####  #   # 
//    #   #      #      #      ##  ## #        #   #    #  # #  
//    #   #####  #      #####  # ## # #####    #   #    #   #   
//    #   #      #      #      #    # #        #   #####    #   
//    #   #      #      #      #    # #        #   #   #    #   
//    #   ###### ###### ###### #    # ######   #   #    #   #   

export interface DefiniteCanisterSettings {
  controllers: Principal[];
  compute_allocation:BigInt;
  memory_allocation: BigInt;
  freezing_threshold: BigInt;
}

export interface Telemetry {
  last_status_check: number;
  main_id: String;
  main_memory_size: number;
  main_memory_used: number;
  main_cycles: number;
  sub_canisters: SubCanisterTelemetry[];
}

export interface SubCanisterTelemetry {
  last_status_check: number;
  id: String;
  settings: DefiniteCanisterSettings;
  status: String;
  module_hash: String;
  memory_size: number;
  cycles: number;
}

export const EMPTY_TELEMETRY = {
  last_status_check: 0,
  main_id: "",
  main_memory_size: 0,
  main_memory_used: 0,
  main_cycles: 0,
  sub_canisters: [],
};
