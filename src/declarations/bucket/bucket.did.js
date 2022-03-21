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
        [IDL.Record({ 'canister_id' : IDL.Text })],
        [BasicResponse],
        [],
      ),
    'updateInstance' : IDL.Func(
        [IDL.Record({ 'id' : IDL.Text, 'json' : IDL.Text })],
        [ModelInstanceResponse],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
