import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { backend } from "../../../declarations/backend";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast } from "../config/toast";
import { updateUserAccount } from "../config/_Actions";
import { EMPTY_USER_ACCOUNT } from "../config/_Interfaces";
import { SignInScreen } from "./SignInScreen";

export const AuthGate = ({ children }: { children: React.ReactNode }) => {
  const user = useSelector((s: StoreState) => s.user);
  const dispatch = useDispatch();

  useEffect(() => {
    if (user.token) {
      getUser(user.token);
    }
  }, []);

  const getUser = async (token: string) => {
    try {
      const user_res = await backend.getUserWithToken(token);
      if (user_res.err) throw new Error(user_res.err);
      const group_res = await backend.getUserGroups(token);
      if (group_res.err) throw new Error(group_res.err);
      dispatch(
        updateUserAccount({
          ...user,
          ...user_res.ok[0],
          groups_list: group_res.ok,
          token,
        })
      );
    } catch (e: any) {
      dispatch(updateUserAccount({ ...EMPTY_USER_ACCOUNT }));
      ErrorToast("Error fetching user");
    }
  };

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-white">
      {user.token ? children : <SignInScreen />}
    </div>
  );
};
