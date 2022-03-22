export const idlFactory = ({ IDL }) => {
  const BasicResponse = IDL.Record({
    'ok' : IDL.Opt(IDL.Text),
    'err' : IDL.Text,
  });
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
  const ModelInstanceResponse = IDL.Record({
    'err' : IDL.Text,
    'json' : IDL.Opt(IDL.Text),
  });
  const ModelListResponse = IDL.Record({
    'ok' : IDL.Vec(Model),
    'err' : IDL.Text,
  });
  const SubCanisterTelemetry = IDL.Record({
    'id' : IDL.Text,
    'memory_size' : IDL.Float64,
    'memory_used' : IDL.Float64,
    'model_name' : IDL.Text,
    'cycles' : IDL.Float64,
  });
  const Telemetry = IDL.Record({
    'last_status_check' : IDL.Float64,
    'main_id' : IDL.Text,
    'bucket_wasm_size' : IDL.Float64,
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
    'activate' : IDL.Func(
        [
          IDL.Record({
            'password' : IDL.Text,
            'bucket_wasm' : IDL.Vec(IDL.Nat8),
            'password_check' : IDL.Text,
          }),
        ],
        [BasicResponse],
        [],
      ),
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
    'isActivated' : IDL.Func([], [IDL.Bool], []),
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
