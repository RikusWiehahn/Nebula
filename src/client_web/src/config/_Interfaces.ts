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
  trusted_canisters: TrustedCanister[];
}

export const EMPTY_AUTH_SESSION: AuthSession = {
  token: "",
  trusted_canisters: [],
};

export interface TrustedCanister {
  name: string;
  canister_id: string;
}

//
//  ##### ###### #      ###### #    # ###### ##### #####  #   #
//    #   #      #      #      ##  ## #        #   #    #  # #
//    #   #####  #      #####  # ## # #####    #   #    #   #
//    #   #      #      #      #    # #        #   #####    #
//    #   #      #      #      #    # #        #   #   #    #
//    #   ###### ###### ###### #    # ######   #   #    #   #

export interface Telemetry {
  last_status_check: number;
  main_id: string;
  main_memory_size: number;
  main_memory_used: number;
  main_cycles: number;
  bucket_wasm_size: number;
  sub_canisters: SubCanisterTelemetry[];
}

export interface SubCanisterTelemetry {
  id: string;
  memory_size: number;
  memory_used: number;
  model_name: string;
  cycles: number;
}

export const EMPTY_TELEMETRY = {
  last_status_check: 0,
  main_id: "",
  main_memory_size: 0,
  main_memory_used: 0,
  main_cycles: 0,
  bucket_wasm_size: 0,
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

export enum DataFieldType {
  STRING = "STRING",
  NUMBER = "NUMBER",
  BOOLEAN = "BOOLEAN",
  STRING_ARRAY = "STRING_ARRAY",
  NUMBER_ARRAY = "NUMBER_ARRAY",
}

export const dataFieldTypesList = [
  { label: "String", value: DataFieldType.STRING, default_value: `` },
  { label: "Boolean", value: DataFieldType.BOOLEAN, default_value: `false` },
  { label: "Number", value: DataFieldType.NUMBER, default_value: `0` },
  {
    label: "String Array",
    value: DataFieldType.STRING_ARRAY,
    default_value: `[]`,
  },
  {
    label: "Number Array",
    value: DataFieldType.NUMBER_ARRAY,
    default_value: `[]`,
  },
];

//                                                   
//  #####  ######  ####   ####  #####  #####   ####  
//  #    # #      #    # #    # #    # #    # #      
//  #    # #####  #      #    # #    # #    #  ####  
//  #####  #      #      #    # #####  #    #      # 
//  #   #  #      #    # #    # #   #  #    # #    # 
//  #    # ######  ####   ####  #    # #####   ####  

export interface RecordDataField {
  field_name: string;
  json_value: string;
  data_type: string;
}
export interface Record {
  id: string;
  model_name: string;
  data_fields: Array<RecordDataField>;
}

export const EMPTY_RECORD = {
  id: "",
  model_name: "",
  data_fields: [],
};

export interface RecordTable {
  model_name: string;
  data_fields: Array<ModelDataFieldType>;
  records: Array<any>;
  page: number;
  page_size: number;
}

export const EMPTY_RECORD_TABLE = {
  model_name: "",
  data_fields: [],
  records: [],
  page: 1,
  page_size: 100,
};
