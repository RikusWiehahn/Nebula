import type { Principal } from '@dfinity/principal';
export interface BasicResponse { 'ok' : [] | [string], 'err' : string }
export interface ModelDataFieldType {
  'field_name' : string,
  'data_type' : string,
  'default_json_value' : string,
}
export interface Record {
  'id' : string,
  'model_name' : string,
  'data_fields' : Array<RecordDataField>,
}
export interface RecordDataField {
  'field_name' : string,
  'json_value' : string,
  'data_type' : string,
}
export interface RecordListResponse {
  'ok' : Array<Record>,
  'err' : string,
  'page_size' : number,
  'page' : number,
}
export interface RecordResponse { 'ok' : [] | [Record], 'err' : string }
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
  'delete_record' : (arg_0: { 'id' : string }) => Promise<BasicResponse>,
  'get_record' : (arg_0: { 'id' : string }) => Promise<RecordResponse>,
  'get_record_list' : (
      arg_0: { 'page_size' : number, 'page' : number },
    ) => Promise<RecordListResponse>,
  'get_telemetry' : () => Promise<SubCanisterTelemetryResponse>,
  'init_model' : (arg_0: { 'model_name' : string }) => Promise<BasicResponse>,
  'insert_record' : (arg_0: Record) => Promise<RecordResponse>,
  'remove_field' : (arg_0: { 'field_name' : string }) => Promise<BasicResponse>,
  'set_admin_canister' : (arg_0: { 'canister_id' : Principal }) => Promise<
      BasicResponse
    >,
  'update_record' : (arg_0: Record) => Promise<RecordResponse>,
  'wallet_receive' : (arg_0: bigint) => Promise<undefined>,
}
