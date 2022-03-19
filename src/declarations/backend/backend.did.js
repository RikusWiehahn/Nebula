export const idlFactory = ({ IDL }) => {
  const BasicResponse = IDL.Record({
    'ok' : IDL.Opt(IDL.Text),
    'err' : IDL.Text,
  });
  const DefiniteCanisterSettings = IDL.Record({
    'freezing_threshold' : IDL.Nat,
    'controllers' : IDL.Vec(IDL.Principal),
    'memory_allocation' : IDL.Nat,
    'compute_allocation' : IDL.Nat,
  });
  const SubCanisterTelemetry = IDL.Record({
    'id' : IDL.Text,
    'status' : IDL.Text,
    'last_status_check' : IDL.Float64,
    'memory_size' : IDL.Float64,
    'cycles' : IDL.Float64,
    'settings' : DefiniteCanisterSettings,
    'module_hash' : IDL.Text,
  });
  const Telemetry = IDL.Record({
    'last_status_check' : IDL.Float64,
    'main_id' : IDL.Text,
    'sub_canisters' : IDL.Vec(SubCanisterTelemetry),
    'main_memory_size' : IDL.Float64,
    'main_memory_used' : IDL.Float64,
    'main_cycles' : IDL.Float64,
  });
  const TelemetryResponse = IDL.Record({
    'ok' : IDL.Opt(Telemetry),
    'err' : IDL.Text,
  });
  return IDL.Service({
    'changePassword' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text],
        [BasicResponse],
        [],
      ),
    'checkSession' : IDL.Func([IDL.Text], [BasicResponse], []),
    'getSystemTelemetry' : IDL.Func([IDL.Text], [TelemetryResponse], []),
    'isAuthSet' : IDL.Func([], [IDL.Bool], []),
    'setAuth' : IDL.Func([IDL.Text, IDL.Text], [BasicResponse], []),
    'signIn' : IDL.Func([IDL.Text], [BasicResponse], []),
  });
};
export const init = ({ IDL }) => { return []; };
