import React, { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { backend } from "../../../declarations/backend";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast, SuccessToast } from "../config/toast";
import { updateAuthSession } from "../config/_Actions";

export const SignInScreen = () => {
  const [loading, setLoading] = React.useState(false);
  const auth = useSelector((s: StoreState) => s.auth);
  const config = useSelector((s: StoreState) => s.config);
  const [isActivated, setIsActivated] = React.useState(false);
  const [state, setState] = React.useState({
    password: "",
    passwordCheck: "",
  });
  const dispatch = useDispatch();

  useEffect(() => {
    isAuthSet();
  }, []);

  const isAuthSet = async () => {
    try {
      setLoading(true);
      const is_auth_set = await backend.is_activated();
      if (is_auth_set) {
        setIsActivated(true);
      }
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const signIn = async () => {
    try {
      setLoading(true);
      const sign_in_res = await backend.sign_in({ password: state.password });
      if (sign_in_res.err) throw new Error(sign_in_res.err);
      if (!sign_in_res.ok[0])
        throw new Error("Failed to sign in, no token returned.");
      dispatch(
        updateAuthSession({
          ...auth,
          token: sign_in_res.ok[0],
        })
      );
      SuccessToast(`Welcome back!`);
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const setAuth = async () => {

    try {
      setLoading(true);
      const path = "bucket-opt.wasm"
      const wasm_file = await fetch(path);
      const bytes_array = new Uint8Array(await wasm_file.arrayBuffer());
      const number_array: number[] = [];
      for (let i = 0; i < bytes_array.length; i++) {
        number_array.push(bytes_array[i]);
      }
      console.log(number_array);
      
      const activate_res = await backend.activate({
        password: state.password,
        password_check: state.passwordCheck,
        bucket_wasm: number_array,
      });
      console.log(activate_res);
      if (activate_res.err) throw new Error(activate_res.err);
      if (!activate_res.ok[0])
        throw new Error("Failed to sign in, no token returned.");
      dispatch(
        updateAuthSession({
          ...auth,
          token: activate_res.ok[0],
        })
      );
      SuccessToast(`Welcome!`);
      setLoading(false);
    } catch (e: any) {
      console.log({e});
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderSignInForm = () => {
    return (
      <div
        className="p-6 bg-white dark:bg-gray-800"
        style={{ maxWidth: "24rem" }}
      >
        <img
          src={config.uiMode === "dark" ? "logo-white.png" : "logo-black.png"}
        />
        <h2 className="text-2xl mb-2 mt-2 font-bold text-center">
          Welcome to Nebula
        </h2>
        <label className="input-label">Password</label>
        <input
          id="password-input"
          type="password"
          placeholder="************************************************************"
          className="input-primary mb-4 mt-1"
          name="password"
          value={state.password}
          onChange={({ target }) =>
            setState({ ...state, password: target.value })
          }
          onKeyPress={(ev) => {
            if (ev.key === "Enter") {
              signIn();
            }
          }}
        />
        <button onClick={signIn} className="btn-primary w-full">
          Sign in
        </button>
      </div>
    );
  };

  const renderSetAuthForm = () => {
    return (
      <div
        className="p-6 bg-white dark:bg-gray-800"
        style={{ maxWidth: "24rem" }}
      >
        <img
          src={config.uiMode === "dark" ? "logo-white.png" : "logo-black.png"}
        />
        <h2 className="text-2xl mb-2 mt-2 font-bold text-center">
          Welcome to Nebula
        </h2>
        <label className="input-label">Password</label>
        <input
          id="password-input"
          type="password"
          placeholder="60 characters minimum"
          className="input-primary mb-4 mt-1"
          name="password"
          value={state.password}
          onChange={({ target }) =>
            setState({ ...state, password: target.value })
          }
        />
        <label className="input-label">Password Check</label>
        <input
          id="password-check-input"
          type="password"
          placeholder="60 characters minimum"
          className="input-primary mb-4 mt-1"
          name="password"
          value={state.passwordCheck}
          onChange={({ target }) =>
            setState({ ...state, passwordCheck: target.value })
          }
          onKeyPress={(ev) => {
            if (ev.key === "Enter") {
              setAuth();
            }
          }}
        />
        <button onClick={setAuth} className="btn-primary w-full">
          Activate
        </button>
      </div>
    );
  };

  return (
    <div className="min-h-screen w-screen flex justify-center items-center p-4 bg-gray-300 dark:bg-gray-900 text-gray-700 dark:text-white">
      {loading ? (
        <LoadingIndicator />
      ) : isActivated ? (
        renderSignInForm()
      ) : (
        renderSetAuthForm()
      )}
    </div>
  );
};
