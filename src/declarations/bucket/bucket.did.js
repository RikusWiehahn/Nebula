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
  const ModelInstanceResponse = IDL.Record({
    'err' : IDL.Text,
    'json' : IDL.Opt(IDL.Text),
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
    'addField' : IDL.Func([ModelDataFieldType], [BasicResponse], []),
    'checkIfAdminCanister' : IDL.Func([], [BasicResponse], []),
    'createInstance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text, 'json' : IDL.Text })],
        [ModelInstanceResponse],
        [],
      ),
    'deleteInstance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'getInstance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text })],
        [ModelInstanceResponse],
        [],
      ),
    'getTelemetry' : IDL.Func([], [SubCanisterTelemetryResponse], []),
    'initModel' : IDL.Func(
        [IDL.Record({ 'model_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'removeField' : IDL.Func(
        [IDL.Record({ 'field_name' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'setAdminCanister' : IDL.Func(
        [IDL.Record({ 'canister_id' : IDL.Principal })],
        [BasicResponse],
        [],
      ),
    'updateInstance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text, 'json' : IDL.Text })],
        [ModelInstanceResponse],
        [],
      ),
    'wallet_drain' : IDL.Func([], [], []),
    'wallet_receive' : IDL.Func([IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
