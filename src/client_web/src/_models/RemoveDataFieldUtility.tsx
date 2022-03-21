import React, { useState } from "react";
import { RiAddLine, RiDeleteBin2Line, RiSubtractLine } from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { backend } from "../../../declarations/backend";
import { ModalWrapper } from "../components/ModalWrapper";
import { SelectDropDown } from "../components/SelectDropDown";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateModelListState } from "../config/_Actions";
import {
  dataFieldTypesList,
  Model,
  ModelDataFieldType,
} from "../config/_Interfaces";

export const RemoveDataFieldUtility = (props: { model: Model }) => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [fieldName, setFieldName] = useState<string>("");
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const model_list = useSelector((s: StoreState) => s.model_list);

  const removeDataField = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const remove_res = await backend.removeModelField({
        token,
        model_name: props.model.model_name,
        field_name: fieldName,
      });
      if (remove_res.err) throw new Error(remove_res.err);
      if (!remove_res.ok) throw new Error("Failed to get models.");
      SuccessToast("Field removed.");
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
        <div className="mb-4">
          Caution: Future calls that include this field will fail. Any data
          contained in this field will be lost forever.
        </div>
        <label>Field to remove</label>
        <SelectDropDown
          options={props.model.data_fields.map((f) => ({
            label: f.field_name,
            value: f.field_name,
          }))}
          value={fieldName}
          onChange={(value) => {
            setFieldName(value);
          }}
        />
        <button className="btn-primary w-full" onClick={removeDataField}>
          Remove data field
        </button>
      </div>
    );
  };

  return (
    <ModalWrapper
      headerLabel="Remove data field from model"
      showModal={showModal}
      isLoading={loading}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button onClick={() => setShowModal(true)} className="btn-list w-80">
          <RiSubtractLine className="mr-4" />
          Remove data field
        </button>
      }
    >
      {renderForm()}
    </ModalWrapper>
  );
};
