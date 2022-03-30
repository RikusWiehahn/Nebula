export const idlFactory = ({ IDL }) => {
  const ModelDataFieldType = IDL.Record({
    'field_name' : IDL.Text,
    'data_type' : IDL.Text,
    'default_json_value' : IDL.Text,
  });
  const BasicResponse = IDL.Record({
    'ok' : IDL.Opt(IDL.Text),
    'err' : IDL.Text,
  });
  const ModelDataField = IDL.Record({
    'field_name' : IDL.Text,
    'json_value' : IDL.Text,
    'data_type' : IDL.Text,
  });
  const ModelInstance = IDL.Record({
    'id' : IDL.Text,
    'model_name' : IDL.Text,
    'data_fields' : IDL.Vec(ModelDataField),
  });
  const ModelInstanceResponse = IDL.Record({
    'ok' : IDL.Opt(ModelInstance),
    'err' : IDL.Text,
  });
  const ModelInstanceListResponse = IDL.Record({
    'err' : IDL.Text,
    'list' : IDL.Vec(ModelInstance),
    'end_index' : IDL.Float64,
    'total_count' : IDL.Float64,
    'start_index' : IDL.Float64,
  });
  const SubCanisterTelemetry = IDL.Record({
    'id' : IDL.Text,
    'memory_size' : IDL.Float64,
    'memory_used' : IDL.Float64,
    'model_name' : IDL.Text,
    'cycles' : IDL.Float64,
  });
  const SubCanisterTelemetryResponse = IDL.Record({
    'ok' : IDL.Opt(SubCanisterTelemetry),
    'err' : IDL.Text,
  });
  return IDL.Service({
    'add_field' : IDL.Func([ModelDataFieldType], [BasicResponse], []),
    'check_if_admin_canister' : IDL.Func([], [BasicResponse], []),
    'create_instance' : IDL.Func([ModelInstance], [ModelInstanceResponse], []),
    'delete_instance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'get_instance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text })],
        [ModelInstanceResponse],
        [],
      ),
    'get_instance_list' : IDL.Func(
        [
          IDL.Record({
            'end_index' : IDL.Float64,
            'start_index' : IDL.Float64,
          }),
        ],
        [ModelInstanceListResponse],
        [],
      ),
    'get_telemetry' : IDL.Func([], [SubCanisterTelemetryResponse], []),
    'init_model' : IDL.Func(
        [IDL.Record({ 'model_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'remove_field' : IDL.Func(
        [IDL.Record({ 'field_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'set_admin_canister' : IDL.Func(
        [IDL.Record({ 'canister_id' : IDL.Principal })],
        [BasicResponse],
        [],
      ),
    'update_instance' : IDL.Func([ModelInstance], [ModelInstanceResponse], []),
    'wallet_receive' : IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
