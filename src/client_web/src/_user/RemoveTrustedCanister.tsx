import React, { useState } from "react";
import { RiAddLine, RiDeleteBin2Fill } from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { backend } from "../../../declarations/backend";
import { ModalWrapper } from "../components/ModalWrapper";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateAuthSession, updateConfigState } from "../config/_Actions";

export const RemoveTrustedCanisterUtility = (props: {
  canister_id: string;
  name: string;
}) => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);

  const removeTrustedCanister = async () => {
    try {
      setLoading(true);
      const remove_res = await backend.removeTrustedCanister({
        token: auth.token,
        canister_id: props.canister_id,
      });
      if (remove_res.err) throw new Error(remove_res.err);
      if (!remove_res.ok) throw new Error("Failed to remove trusted canister.");
      SuccessToast("Trusted canister removed.");
      dispatch(
        updateAuthSession({ ...auth, trusted_canisters: remove_res.ok })
      );
      setLoading(false);
      setShowModal(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  return (
    <ModalWrapper
      headerLabel="Remove trusted canister"
      showModal={showModal}
      isLoading={loading}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button onClick={() => setShowModal(true)} className="btn-circle">
          <RiDeleteBin2Fill />
        </button>
      }
    >
      <div className="text-sm">Canister name</div>
      <div className="font-bold mb-2">{props.name}</div>
      <div className="text-sm">Canister ID</div>
      <div className="font-bold mb-4">{props.canister_id}</div>
      <div className="mb-4">
        Remove this canister's ability to make calls that can CREATE, READ,
        UPDATE, and DELETE instances of data models stored?
      </div>

      <button className="btn-primary w-full" onClick={removeTrustedCanister}>
        Remove trusted canister
      </button>
    </ModalWrapper>
  );
};
