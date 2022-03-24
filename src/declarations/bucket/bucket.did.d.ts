import type { Principal } from '@dfinity/principal';
export interface BasicResponse { 'ok' : [] | [string], 'err' : string }
export interface ModelDataFieldType {
  'field_name' : string,
  'data_type' : string,
  'default_json_value' : string,
}
export interface ModelInstanceResponse {
  'err' : string,
  'json' : [] | [string],
}
export interface SubCanisterTelemetry {
  'id' : string,
  'memory_size' : number,
  'memory_used' : number,
  'model_name' : string,
  'cycles' : number,
}
export interface SubCanisterTelemetryResponse {
  'ok' : [] | [SubCanisterTelemetry],
  'err' : string,
}
export interface _SERVICE {
  'addField' : (arg_0: ModelDataFieldType) => Promise<BasicResponse>,
  'checkIfAdminCanister' : () => Promise<BasicResponse>,
  'createInstance' : (arg_0: { 'id' : string, 'json' : string }) => Promise<
      ModelInstanceResponse
    >,
  'deleteInstance' : (arg_0: { 'id' : string }) => Promise<BasicResponse>,
  'getInstance' : (arg_0: { 'id' : string }) => Promise<ModelInstanceResponse>,
  'getTelemetry' : () => Promise<SubCanisterTelemetryResponse>,
  'initModel' : (arg_0: { 'model_name' : string }) => Promise<BasicResponse>,
  'removeField' : (arg_0: { 'field_name' : string }) => Promise<BasicResponse>,
  'setAdminCanister' : (arg_0: { 'canister_id' : Principal }) => Promise<
      BasicResponse
    >,
  'updateInstance' : (arg_0: { 'id' : string, 'json' : string }) => Promise<
      ModelInstanceResponse
    >,
  'wallet_drain' : () => Promise<undefined>,
  'wallet_receive' : (arg_0: bigint) => Promise<undefined>,
}
