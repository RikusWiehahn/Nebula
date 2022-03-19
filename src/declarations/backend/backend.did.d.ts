import type { Principal } from '@dfinity/principal';
export interface BasicResponse { 'ok' : [] | [string], 'err' : string }
export interface DefiniteCanisterSettings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
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
export interface _SERVICE {
  'changePassword' : (arg_0: string, arg_1: string, arg_2: string) => Promise<
      BasicResponse
    >,
  'checkSession' : (arg_0: string) => Promise<BasicResponse>,
  'getSystemTelemetry' : (arg_0: string) => Promise<TelemetryResponse>,
  'isAuthSet' : () => Promise<boolean>,
  'setAuth' : (arg_0: string, arg_1: string) => Promise<BasicResponse>,
  'signIn' : (arg_0: string) => Promise<BasicResponse>,
}
