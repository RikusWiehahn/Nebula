import React, { useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { ModalWrapper } from "../components/ModalWrapper";
import { routes } from "../config/routes";
import { SuccessToast } from "../config/toast";
import {
  updateConfigState,
  updateGroupState,
  updateUserAccount,
} from "../config/_Actions";
import {
  EMPTY_CONFIG_STATE,
  EMPTY_GROUP,
  EMPTY_USER_ACCOUNT,
} from "../config/_Interfaces";

export const SignOutUtility = () => {
  const [showModal, setShowModal] = useState<boolean>(false);
  const dispatch = useDispatch();
  const navigate = useNavigate();

  const logout = async () => {
    dispatch(updateUserAccount({ ...EMPTY_USER_ACCOUNT }));
    dispatch(updateConfigState({ ...EMPTY_CONFIG_STATE }));
    dispatch(updateGroupState({ ...EMPTY_GROUP }));
    setShowModal(false);
    SuccessToast("Logged out successfully");
    navigate(routes.HOME, { replace: true });
  };

  return (
    <ModalWrapper
      headerLabel="Sign out"
      showModal={showModal}
      onClose={() => setShowModal(false)}
      buttonComponent={
        <button onClick={() => setShowModal(true)} className="btn-list w-full">
          Sign out
        </button>
      }
    >
      <div className="mb-4">We hope to see you again!</div>
      <button className="btn-primary w-full" onClick={logout}>
        Sign out
      </button>
    </ModalWrapper>
  );
};
