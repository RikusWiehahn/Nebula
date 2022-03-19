import React, { useEffect, useState } from "react";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { ErrorToast, SuccessToast } from "../config/toast";
import { CustomModalHeader } from "../components/CustomModalHeader";
import { RiCheckFill, RiCheckLine, RiMailFill } from "react-icons/ri";
import { ModalWrapper } from "../components/ModalWrapper";
import { backend } from "../../../declarations/backend";
import { updateUserAccount } from "../config/_Actions";
import { useDispatch, useSelector } from "react-redux";
import { StoreState } from "../config/ReduxStore";

interface Props {}

export const CreateAccountUtility = (props: Props) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [showModal, setShowModal] = useState<boolean>(false);
  const [state, setState] = useState<{
    email: string;
    password: string;
    password_check: string;
    given_name: string;
    family_name: string;
  }>({
    email: "",
    password: "",
    password_check: "",
    given_name: "",
    family_name: "",
  });
  const user = useSelector((s: StoreState) => s.user);
  const dispatch = useDispatch();

  const familyNameInput: any = React.useRef(null);
  const emailInput: any = React.useRef(null);
  const signUpPasswordInput: any = React.useRef(null);
  const passwordCheckInput: any = React.useRef(null);

  const createUser = async (): Promise<void> => {
    try {
      setLoading(true);
      const create_res = await backend.createUser({
        email: state.email,
        password: state.password,
        password_check: state.password_check,
        given_name: state.given_name,
        family_name: state.family_name,
        invite_token: "",
      });
      if (create_res.err) throw new Error(create_res.err);
      if (!create_res.ok[0]) throw new Error("Failed to create user, no token returned.");
      const user_res = await backend.getUserWithToken(create_res.ok[0]);
      if (user_res.err) throw new Error(user_res.err);
      dispatch(
        updateUserAccount({
          ...user,
          ...user_res.ok[0],
          token: create_res.ok[0],
        })
      );
      SuccessToast("Account created successfully!");
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderForm = () => {
    return (
      <div>
        <label className="input-label">First name</label>
        <input
          className="input-primary mb-4"
          value={state.given_name}
          onChange={(e) => setState({ ...state, given_name: e.target.value })}
          type="text"
          name="name"
          placeholder="Turgon"
          onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key === "Enter") {
              familyNameInput.current?.focus();
            }
          }}
        />
        <label className="input-label">Last name</label>
        <input
          className="input-primary mb-4"
          value={state.family_name}
          ref={familyNameInput}
          onChange={(e) => setState({ ...state, family_name: e.target.value })}
          type="text"
          name="name"
          placeholder="Fingolfin"
          onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key === "Enter") {
              emailInput.current?.focus();
            }
          }}
        />
        <label className="input-label">Email</label>
        <input
          className="input-primary mb-4"
          value={state.email}
          ref={emailInput}
          onChange={(e) => setState({ ...state, email: e.target.value })}
          type="email"
          name="email"
          id="emailInput"
          placeholder="turgon@email.com"
          onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key === "Enter") {
              signUpPasswordInput.current?.focus();
            }
          }}
        />

        <label className="input-label">Password</label>
        <input
          className="input-primary mb-4"
          value={state.password}
          ref={signUpPasswordInput}
          onChange={(e) => setState({ ...state, password: e.target.value })}
          type="password"
          name="password"
          id="passwordInput"
          placeholder="Enter your password"
          onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key === "Enter") {
              passwordCheckInput.current?.focus();
            }
          }}
        />
        <label className="input-label">Password check</label>
        <input
          className="input-primary mb-4"
          value={state.password_check}
          ref={passwordCheckInput}
          onChange={(e) =>
            setState({ ...state, password_check: e.target.value })
          }
          type="password"
          name="passwordCheck"
          id="passwordCheckInput"
          placeholder="Enter your password again"
          onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key === "Enter") {
              createUser();
            }
          }}
        />
        <button
          className="btn-primary w-full"
          tabIndex={0}
          color="primary"
          onClick={createUser}
        >
          Create account
        </button>
      </div>
    );
  };

  return (
    <div>
      <button className="btn-primary w-full" onClick={() => setShowModal(true)}>
        Create account
      </button>
      <ModalWrapper
        headerLabel="Create account"
        showModal={showModal}
        isLoading={loading}
        onClose={() => setShowModal(false)}
      >
        {renderForm()}
      </ModalWrapper>
    </div>
  );
};
