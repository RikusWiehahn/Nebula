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
  const RecordDataField = IDL.Record({
    'field_name' : IDL.Text,
    'json_value' : IDL.Text,
    'data_type' : IDL.Text,
  });
  const Record = IDL.Record({
    'id' : IDL.Text,
    'model_name' : IDL.Text,
    'data_fields' : IDL.Vec(RecordDataField),
  });
  const RecordResponse = IDL.Record({
    'ok' : IDL.Opt(Record),
    'err' : IDL.Text,
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
    'delete_record' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'get_record' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text })],
        [RecordResponse],
        [],
      ),
    'get_telemetry' : IDL.Func([], [SubCanisterTelemetryResponse], []),
    'init_model' : IDL.Func(
        [IDL.Record({ 'model_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'insert_record' : IDL.Func([Record], [RecordResponse], []),
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
    'update_record' : IDL.Func([Record], [RecordResponse], []),
    'wallet_receive' : IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
