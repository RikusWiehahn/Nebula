import React, { useState } from "react";
import { RiAddLine } from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { backend } from "../../../declarations/backend";
import { ModalWrapper } from "../components/ModalWrapper";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateModelListState } from "../config/_Actions";

export const CreateDataModelUtility = () => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [modelName, setModelName] = useState<string>("");
  const dispatch = useDispatch();
  const navigate = useNavigate();
  const auth = useSelector((s: StoreState) => s.auth);
  const model_list = useSelector((s: StoreState) => s.model_list);

  const createDataModel = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const create_res = await backend.create_model({
        token,
        model_name: modelName,
      });
      if (create_res.err) throw new Error(create_res.err);
      if (!create_res.ok) throw new Error("Failed to get models.");
      const models_res = await backend.get_models({ token });
      SuccessToast(create_res.ok?.[0] || "");
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
        <label>Model name</label>
        <input
          className="input-primary"
          value={modelName}
          onChange={({ target }) => {
            // create unique url identifier
            const noCrud = target.value.replace(/\W/g, "_");
            const noSpaces = noCrud.replace(/\s+/g, "_").toLowerCase();
            const identifier = noSpaces.toUpperCase();
            setModelName(identifier);
          }}
          type="text"
          autoComplete="off"
          placeholder="A unique name for the model"
        />
        <button className="btn-primary mt-4 w-full" onClick={createDataModel}>
          Create data model
        </button>
      </div>
    );
  };

  return (
    <ModalWrapper
      headerLabel="Create a new data model"
      showModal={showModal}
      isLoading={loading}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button
          onClick={() => setShowModal(true)}
          className="btn-list"
        >
          <RiAddLine className="mr-4" />
          Create new data model
        </button>
      }
    >
      {renderForm()}
    </ModalWrapper>
  );
};
