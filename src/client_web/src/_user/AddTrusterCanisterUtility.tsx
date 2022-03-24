import React, { useState } from "react";
import { RiAddLine } from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { backend } from "../../../declarations/backend";
import { ModalWrapper } from "../components/ModalWrapper";
import { StoreState } from "../config/ReduxStore";
import { routes } from "../config/routes";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateAuthSession, updateConfigState } from "../config/_Actions";
import { EMPTY_AUTH_SESSION, EMPTY_CONFIG_STATE } from "../config/_Interfaces";

export const AddTrustedCanisterUtility = () => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const [state, setState] = useState<{
    name: string;
    canister_id: string;
  }>({
    name: "",
    canister_id: "",
  });

  const addTrustedCanister = async () => {
    try {
      setLoading(true);
      const add_res = await backend.addTrustedCanister({
        token: auth.token,
        name: state.name,
        canister_id: state.canister_id,
      })
      if (add_res.err) throw new Error(add_res.err);
      if (!add_res.ok) throw new Error("Failed to add trusted canister.");
      SuccessToast("Trusted canister added.");
      dispatch(updateAuthSession({ ...auth, trusted_canisters: add_res.ok }));
      setLoading(false);
      setShowModal(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  return (
    <ModalWrapper
      headerLabel="Add trusted canister"
      showModal={showModal}
      isLoading={loading}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button onClick={() => setShowModal(true)} className="btn-list w-full">
          <RiAddLine className="mr-4" />
          Add trusted canister
        </button>
      }
    >
      <div className="mb-4">
        Authorize an external canister to make calls that can CREATE, READ, UPDATE,
        and DELETE instances of data models stored.
      </div>
      <label className="input-label">Canister name</label>
      <input
        className="input-primary mb-4"
        value={state.name}
        onChange={({ target }) => {
          setState({
            ...state,
            name: target.value,
          });
        }}
        type="text"
        autoComplete="off"
        placeholder={"Canister name"}
      />
      <label className="input-label">Canister ID</label>
      <input
        className="input-primary mb-4"
        value={state.canister_id}
        onChange={({ target }) => {
          setState({
            ...state,
            canister_id: target.value,
          });
        }}
        type="text"
        autoComplete="off"
        placeholder={"rrkah-fqaaa-aaaaa-aaaaq-cai"}
      />
      <button className="btn-primary w-full" onClick={addTrustedCanister}>
        Add trusted canister
      </button>
    </ModalWrapper>
  );
};
