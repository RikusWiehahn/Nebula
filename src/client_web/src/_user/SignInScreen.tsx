import React from "react";
import { useDispatch, useSelector } from "react-redux";
import { backend } from "../../../declarations/backend";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateUserAccount } from "../config/_Actions";
import { CreateAccountUtility } from "./CreateAccountUtility";

export const SignInScreen = () => {
  const [loading, setLoading] = React.useState(false);
  const user = useSelector((s: StoreState) => s.user);
  const config = useSelector((s: StoreState) => s.config);
  const [state, setState] = React.useState({
    email: "",
    password: "",
  });
  const dispatch = useDispatch();
  const passwordInput: any = React.useRef(null);

  const signInWithEmail = async () => {
    try {
      setLoading(true);
      const sign_in_res = await backend.signInWithEmail(
        state.email,
        state.password
      );
      if (sign_in_res.err) throw new Error(sign_in_res.err);
      if (!sign_in_res.ok[0]) throw new Error("Failed to sign in, no token returned.");
      const user_res = await backend.getUserWithToken(sign_in_res.ok[0]);
      if (user_res.err) throw new Error(user_res.err);
      dispatch(
        updateUserAccount({
          ...user,
          ...user_res.ok[0],
          token: sign_in_res.ok[0],
        })
      );
      SuccessToast(`Welcome back, ${user.given_name}!`);
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderForm = () => {
    return (
      <div
        className="p-6 rounded-md bg-white dark:bg-gray-800"
        style={{ maxWidth: "24rem" }}
      >
        <img
          src={config.uiMode === "dark" ? "logo-white.png" : "logo-black.png"}
        />
        <h2 className="text-2xl mb-2 mt-2 font-bold text-center">
          Welcome to Gondolin
        </h2>
        <label className="input-label">email</label>
        <input
          id="email-input"
          placeholder="turgon@email.com"
          className="input-primary mb-4 mt-1"
          name="email"
          value={state.email}
          onChange={({ target }) => setState({ ...state, email: target.value })}
          onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key === "Enter") {
              passwordInput.current?.focus();
            }
          }}
        />
        <label className="input-label">Password</label>
        <input
          id="password-input"
          type="password"
          placeholder="********"
          className="input-primary mb-4 mt-1"
          name="password"
          value={state.password}
          ref={passwordInput}
          onChange={({ target }) =>
            setState({ ...state, password: target.value })
          }
          onKeyPress={(ev) => {
            if (ev.key === "Enter") {
              signInWithEmail();
            }
          }}
        />
        <button onClick={signInWithEmail} className="btn-primary w-full">
          Sign in
        </button>
        <div className="flex items-center">
          <div className="flex-1 border-b-2" />
          <div className="p-2">or</div>
          <div className="flex-1 border-b-2" />
        </div>
        <CreateAccountUtility />
        <button
          // onClick={signInUser}
          className="btn-menu justify-center w-full"
        >
          Forgot password?
        </button>
      </div>
    );
  };

  return (
    <div className="min-h-screen w-screen flex justify-center items-center p-4 bg-gray-300 dark:bg-gray-900 text-gray-700 dark:text-white">
      {loading ? <LoadingIndicator /> : renderForm()}
    </div>
  );
};
