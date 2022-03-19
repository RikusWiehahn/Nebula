import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { backend } from "../../../declarations/backend";
import { SecondaryLayout } from "../components/SecondaryLayout";
import { UIModeSwitch } from "../components/UIModeSwitch";
import { StoreState } from "../config/ReduxStore";
import { routes } from "../config/routes";
import { ErrorToast } from "../config/toast";
import { updateUserAccount } from "../config/_Actions";
import { AuthGate } from "./AuthGate";
// import { ProfilePicUtility } from "./ProfilePictureUtility";
import { SignOutUtility } from "./SignOutUtility";

export const ProfileScreen = () => {
  const user = useSelector((state: StoreState) => state.user);  
  let navigate = useNavigate();

  return (
    <SecondaryLayout
      onPressBack={() => navigate(routes.HOME, { replace: true })}
      header="My account"
    >
      <AuthGate>
        <div className="max-w-md pt-8">
          <h1 className="font-bold text-3xl mt-4 mb-4">
            {`${user.given_name} ${user.family_name}`}
          </h1>
          <SignOutUtility />
          <UIModeSwitch />
        </div>
      </AuthGate>
    </SecondaryLayout>
  );
};
