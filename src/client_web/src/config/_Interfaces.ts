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
  compute_allocation: BigInt;
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

//
//  #    #  ####  #####  ###### #       ####
//  ##  ## #    # #    # #      #      #
//  # ## # #    # #    # #####  #       ####
//  #    # #    # #    # #      #           #
//  #    # #    # #    # #      #      #    #
//  #    #  ####  #####  ###### ######  ####

export interface Model {
  model_name: string;
  data_fields: Array<ModelDataFieldType>;
}

export const EMPTY_MODEL = {
  model_name: "",
  data_fields: [],
};
export interface ModelDataFieldType {
  field_name: string;
  data_type: string;
  default_json_value: string;
}

export interface ModelList {
  models: Model[];
}

export const EMPTY_MODEL_LIST = {
  models: [],
};

export const dataFieldTypesList = [
  { label: "String", value: "STRING", default_value: "" },
  { label: "Boolean", value: "BOOLEAN", default_value: "false" },
  { label: "Number", value: "NUMBER", default_value: "0" },
  { label: "String Array", value: "STRING_ARRAY", default_value: "[]" },
  { label: "Number Array", value: "NUMBER_ARRAY", default_value: "[]" },
]