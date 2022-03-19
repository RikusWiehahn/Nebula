import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { SecondaryLayout } from "../components/SecondaryLayout";
import { UIModeSwitch } from "../components/UIModeSwitch";
import { StoreState } from "../config/ReduxStore";
import { routes } from "../config/routes";
import { AuthGate } from "./AuthGate";
import { SignOutUtility } from "./SignOutUtility";

export const ProfileScreen = () => {
  const auth = useSelector((state: StoreState) => state.auth);
  let navigate = useNavigate();

  return (
    <AuthGate>
      <SecondaryLayout
        onPressBack={() => navigate(routes.HOME, { replace: true })}
        header="Settings"
      >
        <div className="max-w-md pt-8">
          <SignOutUtility />
          <UIModeSwitch />
        </div>
      </SecondaryLayout>
    </AuthGate>
  );
};
