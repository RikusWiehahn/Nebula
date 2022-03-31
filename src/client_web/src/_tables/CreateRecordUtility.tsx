import React, { useEffect, useState } from "react";
import { RiAddLine } from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { backend } from "../../../declarations/backend";
import { ModalWrapper } from "../components/ModalWrapper";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateModelListState } from "../config/_Actions";
import {
  DataFieldType,
  EMPTY_RECORD,
  Model,
  Record,
} from "../config/_Interfaces";

interface Props {
  model: Model;
}

export const CreateRecordUtility = (props: Props) => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const record_table = useSelector((s: StoreState) => s.record_table);
  const [record, setRecord] = useState<Record>({
    ...EMPTY_RECORD,
  });

  useEffect(() => {
    setRecord({
      ...EMPTY_RECORD,
      model_name: props.model.model_name,
      data_fields: props.model.data_fields.map((f) => ({
        field_name: f.field_name,
        data_type: f.data_type,
        json_value: f.default_json_value,
      })),
    });
  }, []);

  const createRecord = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const json = Object.fromEntries([
        ["id", record.id],
        ["model_name", record.model_name],
        ...record.data_fields.map((f) => [f.field_name, f.json_value]),
      ]);

      console.log(json);
      const create_res = await backend.create_record({
        token,
        json: JSON.stringify(json),
      });

      if (create_res.err) throw new Error(create_res.err);
      if (!create_res?.ok[0]) throw new Error("Failed to create model record.");
      SuccessToast("Model record created.");
      setLoading(false);
      setShowModal(false);
    } catch (e: any) {
      console.log({e});
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderFields = () => {
    const list = props.model.data_fields.map((f) => {
      if (f.data_type === DataFieldType.STRING) {
        const value = record.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_record = { ...record };
                const field = new_record.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setRecord(new_record);
              }}
              type="text"
              autoComplete="off"
              placeholder={`""`}
            />
          </div>
        );
      }
      if (f.data_type === DataFieldType.BOOLEAN) {
        const value = record.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_record = { ...record };
                const field = new_record.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setRecord(new_record);
              }}
              type="text"
              autoComplete="off"
              placeholder={`false`}
            />
          </div>
        );
      }
      if (f.data_type === DataFieldType.NUMBER) {
        const value = record.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_record = { ...record };
                const field = new_record.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setRecord(new_record);
              }}
              type="text"
              autoComplete="off"
              placeholder={`0`}
            />
          </div>
        );
      }
      if (f.data_type === DataFieldType.STRING_ARRAY) {
        const value = record.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_record = { ...record };
                const field = new_record.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setRecord(new_record);
              }}
              type="text"
              autoComplete="off"
              placeholder={`[]`}
            />
          </div>
        );
      }
      if (f.data_type === DataFieldType.NUMBER_ARRAY) {
        const value = record.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_record = { ...record };
                const field = new_record.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setRecord(new_record);
              }}
              type="text"
              autoComplete="off"
              placeholder={`[]`}
            />
          </div>
        );
      }
    });

    return list;
  };

  const renderForm = () => {
    return (
      <div>
        {renderFields()}
        <button
          className="btn-primary mt-4 w-full"
          onClick={createRecord}
        >
          Create new record
        </button>
      </div>
    );
  };

  return (
    <ModalWrapper
      headerLabel="Create a new record"
      showModal={showModal}
      isLoading={loading}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button onClick={() => setShowModal(true)} className="btn-list py-2">
          <RiAddLine className="mr-4" />
          Create new record
        </button>
      }
    >
      {renderForm()}
    </ModalWrapper>
  );
};
