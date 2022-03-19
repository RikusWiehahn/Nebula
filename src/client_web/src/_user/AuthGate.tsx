import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { backend } from "../../../declarations/backend";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast } from "../config/toast";
import { updateAuthSession } from "../config/_Actions";
import { EMPTY_AUTH_SESSION } from "../config/_Interfaces";
import { SignInScreen } from "./SignInScreen";

export const AuthGate = ({ children }: { children: React.ReactNode }) => {
  const auth = useSelector((s: StoreState) => s.auth);
  const dispatch = useDispatch();

  useEffect(() => {
    if (auth.token) {
      checkSession();
    }
  }, []);

  const checkSession = async () => {
    try {
      const { token } = auth;
      const session_res = await backend.checkSession(token);
      if (session_res.err) throw new Error(session_res.err);
      dispatch(
        updateAuthSession({
          ...auth,
          token,
        })
      );
    } catch (e: any) {
      dispatch(updateAuthSession({ ...EMPTY_AUTH_SESSION }));
      ErrorToast("Session expired, please sign in again.");
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-white">
      {auth.token ? children : <SignInScreen />}
    </div>
  );
};
