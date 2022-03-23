import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router-dom";
import { Layout } from "../components/Layout";
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
      <Layout>
        <div className="container px-4 mx-auto">
        <div className="max-w-md pt-8">
          <h1 className="text-xl font-bold mb-4">Settings</h1>
          <SignOutUtility />
          <UIModeSwitch />
        </div>
        </div>
      </Layout>
    </AuthGate>
  );
};
