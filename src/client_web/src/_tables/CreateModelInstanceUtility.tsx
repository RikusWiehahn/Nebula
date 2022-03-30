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
  EMPTY_MODEL_INSTANCE,
  Model,
  ModelInstance,
} from "../config/_Interfaces";

interface Props {
  model: Model;
}

export const CreateModelInstanceUtility = (props: Props) => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const model_table = useSelector((s: StoreState) => s.model_table);
  const [instance, setInstance] = useState<ModelInstance>({
    ...EMPTY_MODEL_INSTANCE,
  });

  useEffect(() => {
    setInstance({
      ...EMPTY_MODEL_INSTANCE,
      model_name: props.model.model_name,
      data_fields: props.model.data_fields.map((f) => ({
        field_name: f.field_name,
        data_type: f.data_type,
        json_value: f.default_json_value,
      })),
    });
  }, []);

  const createDataModelInstance = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const json = Object.fromEntries([
        ["id", instance.id],
        ["model_name", instance.model_name],
        ...instance.data_fields.map((f) => [f.field_name, f.json_value]),
      ]);

      const create_res = await backend.create_model_instance({
        token,
        json: JSON.stringify(json),
      });
      if (create_res.err) throw new Error(create_res.err);
      if (!create_res.json) throw new Error("Failed to create model instance.");
      SuccessToast("Model instance created.");
      setLoading(false);
      setShowModal(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderFields = () => {
    const list = props.model.data_fields.map((f) => {
      if (f.data_type === DataFieldType.STRING) {
        const value = instance.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_instance = { ...instance };
                const field = new_instance.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setInstance(new_instance);
              }}
              type="text"
              autoComplete="off"
              placeholder={`""`}
            />
          </div>
        );
      }
      if (f.data_type === DataFieldType.BOOLEAN) {
        const value = instance.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_instance = { ...instance };
                const field = new_instance.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setInstance(new_instance);
              }}
              type="text"
              autoComplete="off"
              placeholder={`false`}
            />
          </div>
        );
      }
      if (f.data_type === DataFieldType.NUMBER) {
        const value = instance.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_instance = { ...instance };
                const field = new_instance.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setInstance(new_instance);
              }}
              type="text"
              autoComplete="off"
              placeholder={`0`}
            />
          </div>
        );
      }
      if (f.data_type === DataFieldType.STRING_ARRAY) {
        const value = instance.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_instance = { ...instance };
                const field = new_instance.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setInstance(new_instance);
              }}
              type="text"
              autoComplete="off"
              placeholder={`[]`}
            />
          </div>
        );
      }
      if (f.data_type === DataFieldType.NUMBER_ARRAY) {
        const value = instance.data_fields.find(
          (d) => d.field_name === f.field_name
        )?.json_value;

        return (
          <div key={f.field_name}>
            <label>{f.field_name}</label>
            <input
              className="input-primary"
              value={value || ""}
              onChange={({ target }) => {
                const new_instance = { ...instance };
                const field = new_instance.data_fields.find(
                  (d) => d.field_name === f.field_name
                );
                if (!field) return;
                field.json_value = target.value;
                setInstance(new_instance);
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
          onClick={createDataModelInstance}
        >
          Create new item
        </button>
      </div>
    );
  };

  return (
    <ModalWrapper
      headerLabel="Create a new item"
      showModal={showModal}
      isLoading={loading}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button onClick={() => setShowModal(true)} className="btn-list">
          <RiAddLine className="mr-4" />
          Create new item
        </button>
      }
    >
      {renderForm()}
    </ModalWrapper>
  );
};
