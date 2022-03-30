import React, { useState } from "react";
import { RiAddLine, RiDeleteBin2Line, RiSubtractLine } from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { backend } from "../../../declarations/backend";
import { ModalWrapper } from "../components/ModalWrapper";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateModelListState } from "../config/_Actions";
import { Model } from "../config/_Interfaces";

export const DeleteModelUtility = (props: { model: Model }) => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const model_list = useSelector((s: StoreState) => s.model_list);

  const deleteModel = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const delete_res = await backend.delete_model({
        token,
        model_name: props.model.model_name,
      });
      if (delete_res.err) throw new Error(delete_res.err);
      if (!delete_res.ok) throw new Error("Failed to delete model.");
      SuccessToast(delete_res.ok?.[0] || "");
      const models_res = await backend.get_models({ token });
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
          Caution: Future calls to this model will fail. Any data contained in
          this model will be lost forever.
        </div>
        <button className="btn-primary w-full" onClick={deleteModel}>
          Delete this data model
        </button>
      </div>
    );
  };

  return (
    <ModalWrapper
      headerLabel="Delete this data model"
      showModal={showModal}
      isLoading={loading}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button onClick={() => setShowModal(true)} className="btn-list w-80">
          <RiDeleteBin2Line className="mr-4" />
          Delete this model
        </button>
      }
    >
      {renderForm()}
    </ModalWrapper>
  );
};
