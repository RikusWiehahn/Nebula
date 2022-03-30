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
  const TrustedCanister = IDL.Record({
    'name' : IDL.Text,
    'canister_id' : IDL.Text,
  });
  const TrustedCanistersResponse = IDL.Record({
    'ok' : IDL.Vec(TrustedCanister),
    'err' : IDL.Text,
  });
  const RecordJsonResponse = IDL.Record({
    'ok' : IDL.Opt(IDL.Text),
    'err' : IDL.Text,
  });
  const ModelListResponse = IDL.Record({
    'ok' : IDL.Vec(Model),
    'err' : IDL.Text,
  });
  const RecordListJsonResponse = IDL.Record({
    'ok' : IDL.Vec(IDL.Text),
    'err' : IDL.Text,
    'page_size' : IDL.Float64,
    'page' : IDL.Float64,
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
    'add_model_field' : IDL.Func(
        [
          IDL.Record({ 'token' : IDL.Text, 'model_name' : IDL.Text }),
          ModelDataFieldType,
        ],
        [ModelResponse],
        [],
      ),
    'add_trusted_canister' : IDL.Func(
        [
          IDL.Record({
            'token' : IDL.Text,
            'name' : IDL.Text,
            'canister_id' : IDL.Text,
          }),
        ],
        [TrustedCanistersResponse],
        [],
      ),
    'change_password' : IDL.Func(
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
    'check_session' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'create_model' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'model_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'create_record' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'json' : IDL.Text })],
        [RecordJsonResponse],
        [],
      ),
    'delete_model' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'model_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'delete_record' : IDL.Func(
        [
          IDL.Record({
            'id' : IDL.Text,
            'token' : IDL.Text,
            'model_name' : IDL.Text,
          }),
        ],
        [BasicResponse],
        [],
      ),
    'get_model' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'model_name' : IDL.Text })],
        [ModelResponse],
        [],
      ),
    'get_models' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text })],
        [ModelListResponse],
        [],
      ),
    'get_record' : IDL.Func(
        [
          IDL.Record({
            'id' : IDL.Text,
            'token' : IDL.Text,
            'model_name' : IDL.Text,
          }),
        ],
        [RecordJsonResponse],
        [],
      ),
    'get_record_list' : IDL.Func(
        [
          IDL.Record({
            'page_size' : IDL.Float64,
            'token' : IDL.Text,
            'model_name' : IDL.Text,
            'page' : IDL.Float64,
          }),
        ],
        [RecordListJsonResponse],
        [],
      ),
    'get_telemetry' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text })],
        [TelemetryResponse],
        [],
      ),
    'get_trusted_canisters' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text })],
        [TrustedCanistersResponse],
        [],
      ),
    'is_activated' : IDL.Func([], [IDL.Bool], []),
    'remove_model_field' : IDL.Func(
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
    'remove_trusted_canister' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'canister_id' : IDL.Text })],
        [TrustedCanistersResponse],
        [],
      ),
    'sign_in' : IDL.Func(
        [IDL.Record({ 'password' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'update_record' : IDL.Func(
        [IDL.Record({ 'token' : IDL.Text, 'json' : IDL.Text })],
        [RecordJsonResponse],
        [],
      ),
    'wallet_receive' : IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
