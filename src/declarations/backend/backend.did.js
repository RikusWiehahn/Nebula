export const idlFactory = ({ IDL }) => {
  const ModelDataFieldType = IDL.Record({
    'field_name' : IDL.Text,
    'data_type' : IDL.Text,
    'default_json_value' : IDL.Text,
  });
  const Model = IDL.Record({
    'model_name' : IDL.Text,
    'data_fields' : IDL.Vec(ModelDataFieldType),
  });
  const ModelResponse = IDL.Record({ 'ok' : IDL.Opt(Model), 'err' : IDL.Text });
  const TrustedCanistersResponse = IDL.Record({
    'ok' : IDL.Vec(IDL.Text),
    'err' : IDL.Text,
  });
  const BasicResponse = IDL.Record({
    'ok' : IDL.Opt(IDL.Text),
    'err' : IDL.Text,
  });
  const ModelInstanceResponse = IDL.Record({
    'err' : IDL.Text,
    'json' : IDL.Opt(IDL.Text),
  });
  const ModelListResponse = IDL.Record({
    'ok' : IDL.Vec(Model),
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
    'addModelField' : IDL.Func(
        [
          IDL.Record({ 'token' : IDL.Text, 'model_name' : IDL.Text }),
          ModelDataFieldType,
        ],
        [ModelResponse],
        [],
      ),
    'addTrustedCanister' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'canister_id' : IDL.Text })],
        [TrustedCanistersResponse],
        [],
      ),
    'changePassword' : IDL.Func(
        [
          IDL.Record({
            'password' : IDL.Text,
            'old_password' : IDL.Text,
            'password_check' : IDL.Text,
          }),
        ],
        [BasicResponse],
        [],
      ),
    'checkSession' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'createModel' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'model_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'createModelInstance' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'json' : IDL.Text })],
        [ModelInstanceResponse],
        [],
      ),
    'deleteModel' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'model_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'deleteModelInstance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text, 'token' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'getModel' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'model_name' : IDL.Text })],
        [ModelResponse],
        [],
      ),
    'getModelInstance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text, 'token' : IDL.Text })],
        [ModelInstanceResponse],
        [],
      ),
    'getModels' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text })],
        [ModelListResponse],
        [],
      ),
    'getSystemTelemetry' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text })],
        [TelemetryResponse],
        [],
      ),
    'getTrustedCanisters' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text })],
        [TrustedCanistersResponse],
        [],
      ),
    'isAuthSet' : IDL.Func([], [IDL.Bool], []),
    'removeModelField' : IDL.Func(
        [
          IDL.Record({
            'token' : IDL.Text,
            'field_name' : IDL.Text,
            'model_name' : IDL.Text,
          }),
        ],
        [ModelResponse],
        [],
      ),
    'removeTrustedCanister' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'canister_id' : IDL.Text })],
        [TrustedCanistersResponse],
        [],
      ),
    'setAuth' : IDL.Func(
        [IDL.Record({ 'password' : IDL.Text, 'password_check' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'signIn' : IDL.Func(
        [IDL.Record({ 'password' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'updateModelInstance' : IDL.Func(
        [
          IDL.Record({
            'id' : IDL.Text,
            'token' : IDL.Text,
            'json' : IDL.Text,
          }),
        ],
        [ModelInstanceResponse],
        [],
      ),
    'wallet_receive' : IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
