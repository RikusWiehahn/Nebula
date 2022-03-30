import type { Principal } from '@dfinity/principal';
export interface BasicResponse { 'ok' : [] | [string], 'err' : string }
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
  'memory_size' : number,
  'memory_used' : number,
  'model_name' : string,
  'cycles' : number,
}
export interface Telemetry {
  'last_status_check' : number,
  'main_id' : string,
  'bucket_wasm_size' : number,
  'sub_canisters' : Array<SubCanisterTelemetry>,
  'main_memory_size' : number,
  'main_memory_used' : number,
  'main_cycles' : number,
}
export interface TelemetryResponse { 'ok' : [] | [Telemetry], 'err' : string }
export interface TrustedCanister { 'name' : string, 'canister_id' : string }
export interface TrustedCanistersResponse {
  'ok' : Array<TrustedCanister>,
  'err' : string,
}
export interface _SERVICE {
  'activate' : (
      arg_0: {
        'password' : string,
        'bucket_wasm' : Array<number>,
        'password_check' : string,
      },
    ) => Promise<BasicResponse>,
  'add_model_field' : (
      arg_0: { 'token' : string, 'model_name' : string },
      arg_1: ModelDataFieldType,
    ) => Promise<ModelResponse>,
  'add_trusted_canister' : (
      arg_0: { 'token' : string, 'name' : string, 'canister_id' : string },
    ) => Promise<TrustedCanistersResponse>,
  'change_password' : (
      arg_0: {
        'password' : string,
        'old_password' : string,
        'password_check' : string,
      },
    ) => Promise<BasicResponse>,
  'check_session' : (arg_0: { 'token' : string }) => Promise<BasicResponse>,
  'create_model' : (
      arg_0: { 'token' : string, 'model_name' : string },
    ) => Promise<BasicResponse>,
  'create_model_instance' : (
      arg_0: { 'token' : string, 'json' : string },
    ) => Promise<ModelInstanceResponse>,
  'delete_model' : (
      arg_0: { 'token' : string, 'model_name' : string },
    ) => Promise<BasicResponse>,
  'delete_model_instance' : (
      arg_0: { 'id' : string, 'token' : string },
    ) => Promise<BasicResponse>,
  'get_model' : (arg_0: { 'token' : string, 'model_name' : string }) => Promise<
      ModelResponse
    >,
  'get_model_instance' : (
      arg_0: { 'id' : string, 'token' : string },
    ) => Promise<ModelInstanceResponse>,
  'get_models' : (arg_0: { 'token' : string }) => Promise<ModelListResponse>,
  'get_telemetry' : (arg_0: { 'token' : string }) => Promise<TelemetryResponse>,
  'get_trusted_canisters' : (arg_0: { 'token' : string }) => Promise<
      TrustedCanistersResponse
    >,
  'is_activated' : () => Promise<boolean>,
  'remove_model_field' : (
      arg_0: { 'token' : string, 'field_name' : string, 'model_name' : string },
    ) => Promise<ModelResponse>,
  'remove_trusted_canister' : (
      arg_0: { 'token' : string, 'canister_id' : string },
    ) => Promise<TrustedCanistersResponse>,
  'sign_in' : (arg_0: { 'password' : string }) => Promise<BasicResponse>,
  'update_model_instance' : (
      arg_0: { 'id' : string, 'token' : string, 'json' : string },
    ) => Promise<ModelInstanceResponse>,
  'wallet_receive' : (arg_0: bigint) => Promise<undefined>,
}
