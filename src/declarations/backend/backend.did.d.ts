import type { Principal } from '@dfinity/principal';
export interface BasicResponse { 'ok' : [] | [string], 'err' : string }
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export interface Model {
  'model_name' : string,
  'data_fields' : Array<ModelDataFieldType>,
}
export interface ModelDataFieldType {
  'field_name' : string,
  'data_type' : string,
  'default_json_value' : string,
}
export interface ModelInstanceResponse {
  'err' : string,
  'json' : [] | [string],
}
export interface ModelListResponse { 'ok' : Array<Model>, 'err' : string }
export interface ModelResponse { 'ok' : [] | [Model], 'err' : string }
export interface SubCanisterTelemetry {
  'id' : string,
  'status' : string,
  'last_status_check' : number,
  'memory_size' : number,
  'cycles' : number,
  'settings' : DefiniteCanisterSettings,
  'module_hash' : string,
}
export interface Telemetry {
  'last_status_check' : number,
  'main_id' : string,
  'sub_canisters' : Array<SubCanisterTelemetry>,
  'main_memory_size' : number,
  'main_memory_used' : number,
  'main_cycles' : number,
}
export interface TelemetryResponse { 'ok' : [] | [Telemetry], 'err' : string }
export interface TrustedCanistersResponse {
  'ok' : Array<string>,
  'err' : string,
}
export interface _SERVICE {
  'addModelField' : (
      arg_0: { 'token' : string, 'model_name' : string },
      arg_1: ModelDataFieldType,
    ) => Promise<ModelResponse>,
  'addTrustedCanister' : (
      arg_0: { 'token' : string, 'canister_id' : string },
    ) => Promise<TrustedCanistersResponse>,
  'changePassword' : (
      arg_0: {
        'password' : string,
        'old_password' : string,
        'password_check' : string,
      },
    ) => Promise<BasicResponse>,
  'checkSession' : (arg_0: { 'token' : string }) => Promise<BasicResponse>,
  'createModel' : (
      arg_0: { 'token' : string, 'model_name' : string },
    ) => Promise<BasicResponse>,
  'createModelInstance' : (
      arg_0: { 'token' : string, 'json' : string },
    ) => Promise<ModelInstanceResponse>,
  'deleteModel' : (
      arg_0: { 'token' : string, 'model_name' : string },
    ) => Promise<BasicResponse>,
  'deleteModelInstance' : (
      arg_0: { 'id' : string, 'token' : string },
    ) => Promise<BasicResponse>,
  'getModel' : (arg_0: { 'token' : string, 'model_name' : string }) => Promise<
      ModelResponse
    >,
  'getModelInstance' : (arg_0: { 'id' : string, 'token' : string }) => Promise<
      ModelInstanceResponse
    >,
  'getModels' : (arg_0: { 'token' : string }) => Promise<ModelListResponse>,
  'getSystemTelemetry' : (arg_0: { 'token' : string }) => Promise<
      TelemetryResponse
    >,
  'getTrustedCanisters' : (arg_0: { 'token' : string }) => Promise<
      TrustedCanistersResponse
    >,
  'isAuthSet' : () => Promise<boolean>,
  'removeModelField' : (
      arg_0: { 'token' : string, 'field_name' : string, 'model_name' : string },
    ) => Promise<ModelResponse>,
  'removeTrustedCanister' : (
      arg_0: { 'token' : string, 'canister_id' : string },
    ) => Promise<TrustedCanistersResponse>,
  'setAuth' : (
      arg_0: { 'password' : string, 'password_check' : string },
    ) => Promise<BasicResponse>,
  'signIn' : (arg_0: { 'password' : string }) => Promise<BasicResponse>,
  'updateModelInstance' : (
      arg_0: { 'id' : string, 'token' : string, 'json' : string },
    ) => Promise<ModelInstanceResponse>,
  'wallet_receive' : (arg_0: bigint) => Promise<undefined>,
}
