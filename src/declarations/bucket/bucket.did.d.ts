import type { Principal } from '@dfinity/principal';
export interface BasicResponse { 'ok' : [] | [string], 'err' : string }
export interface ModelDataField {
  'field_name' : string,
  'json_value' : string,
  'data_type' : string,
}
export interface ModelDataFieldType {
  'field_name' : string,
  'data_type' : string,
  'default_json_value' : string,
}
export interface ModelInstance {
  'id' : string,
  'model_name' : string,
  'data_fields' : Array<ModelDataField>,
}
export interface ModelInstanceListResponse {
  'err' : string,
  'list' : Array<ModelInstance>,
  'end_index' : number,
  'total_count' : number,
  'start_index' : number,
}
export interface ModelInstanceResponse {
  'ok' : [] | [ModelInstance],
  'err' : string,
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
  'add_field' : (arg_0: ModelDataFieldType) => Promise<BasicResponse>,
  'check_if_admin_canister' : () => Promise<BasicResponse>,
  'create_instance' : (arg_0: ModelInstance) => Promise<ModelInstanceResponse>,
  'delete_instance' : (arg_0: { 'id' : string }) => Promise<BasicResponse>,
  'get_instance' : (arg_0: { 'id' : string }) => Promise<ModelInstanceResponse>,
  'get_instance_list' : (
      arg_0: { 'end_index' : number, 'start_index' : number },
    ) => Promise<ModelInstanceListResponse>,
  'get_telemetry' : () => Promise<SubCanisterTelemetryResponse>,
  'init_model' : (arg_0: { 'model_name' : string }) => Promise<BasicResponse>,
  'remove_field' : (arg_0: { 'field_name' : string }) => Promise<BasicResponse>,
  'set_admin_canister' : (arg_0: { 'canister_id' : Principal }) => Promise<
      BasicResponse
    >,
  'update_instance' : (arg_0: ModelInstance) => Promise<ModelInstanceResponse>,
  'wallet_receive' : (arg_0: bigint) => Promise<undefined>,
}
