import React, { useState } from "react";
import { RiAddLine } from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { backend } from "../../../declarations/backend";
import { ModalWrapper } from "../components/ModalWrapper";
import { SelectDropDown } from "../components/SelectDropDown";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateModelListState } from "../config/_Actions";
import { dataFieldTypesList, Model, ModelDataFieldType } from "../config/_Interfaces";

export const AddDataFieldUtility = (props: { model: Model }) => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [state, setState] = useState<ModelDataFieldType>({
    field_name: "",
    data_type: "STRING",
    default_json_value: `""`,
  });
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const model_list = useSelector((s: StoreState) => s.model_list);

  const addDataField = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const add_res = await backend.addModelField(
        {
          token,
          model_name: props.model.model_name,
        },
        {
          field_name: state.field_name,
          data_type: state.data_type,
          default_json_value: state.default_json_value,
        }
      );
      if (add_res.err) throw new Error(add_res.err);
      if (!add_res.ok) throw new Error("Failed to get models.");
      SuccessToast("Field added.");
      const models_res = await backend.getModels({ token });
      if (models_res.err) throw new Error(models_res.err);
      if (!models_res.ok) throw new Error("Failed to get models.");
      dispatch(
        updateModelListState({
          ...model_list,
          models: models_res.ok,
        })
      );
      setLoading(false);
      setShowModal(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderForm = () => {
    return (
      <div>
        <label>Data field name</label>
        <input
          className="input-primary mb-4"
          value={state.field_name}
          onChange={({ target }) => {
            // create unique url identifier
            const noCrud = target.value.replace(/\W/g, "_");
            const noSpaces = noCrud.replace(/\s+/g, "_").toLowerCase();
            const identifier = noSpaces.toLowerCase();
            setState({
              ...state,
              field_name: identifier,
            });
          }}
          type="text"
          autoComplete="off"
          placeholder="A unique name for the data field"
        />
        <label>Data field type</label>
        <SelectDropDown
          options={dataFieldTypesList}
          value={state.data_type}
          onChange={(value) => {
            const item = dataFieldTypesList.find((v) => v.value === value);
            if (!item) return;
            setState({
              ...state,
              data_type: item.value,
              default_json_value: item.default_value,
            });
          }}
        />
        <label>Default value</label>
        <input
          className="input-primary"
          value={state.default_json_value}
          onChange={({ target }) => {
            setState({
              ...state,
              default_json_value: target.value,
            });
          }}
          type="text"
          autoComplete="off"
          placeholder={`""`}
        />
        <button className="btn-primary mt-4 w-full" onClick={addDataField}>
          Add data field
        </button>
      </div>
    );
  };

  return (
    <ModalWrapper
      headerLabel="Add data field to model"
      showModal={showModal}
      isLoading={loading}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button onClick={() => setShowModal(true)} className="btn-list w-80">
          <RiAddLine className="mr-4" />
          Add data field
        </button>
      }
    >
      {renderForm()}
    </ModalWrapper>
  );
};
