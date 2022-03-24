import React, { useEffect, useState } from "react";
import { RiDeleteBin2Fill } from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { backend } from "../../../declarations/backend";
import { Layout } from "../components/Layout";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { UIModeSwitch } from "../components/UIModeSwitch";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast } from "../config/toast";
import { updateAuthSession } from "../config/_Actions";
import { AddTrustedCanisterUtility } from "./AddTrusterCanisterUtility";
import { AuthGate } from "./AuthGate";
import { RemoveTrustedCanisterUtility } from "./RemoveTrustedCanister";
import { SignOutUtility } from "./SignOutUtility";

export const ProfileScreen = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const auth = useSelector((state: StoreState) => state.auth);
  let dispatch = useDispatch();

  useEffect(() => {
    if (auth.token) {
      getTrustedCanisters();
    }
  }, [auth.token]);

  const getTrustedCanisters = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const trusted_res = await backend.getTrustedCanisters({ token });
      if (trusted_res.err) throw new Error(trusted_res.err);
      dispatch(
        updateAuthSession({
          ...auth,
          trusted_canisters: trusted_res.ok,
        })
      );
      setLoading(false);
    } catch (e: any) {
      console.log({ e });
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderTrustedCanisters = () => {
    return (
      <div className="p-4 dark:bg-gray-800 bg-gray-100 mb-4">
        <h2 className="text-lg font-bold mb-4">Trusted external canisters</h2>
        {auth.trusted_canisters.map((canister) => (
          <div className="px-4 py-2 flex items-center bg-gray-900 bg-opacity-10 dark:bg-white dark:bg-opacity-10 mb-2">
            <div className="flex-1">
              <div className="text-xs">{canister.name}</div>
              <div className="font-bold">{canister.canister_id}</div>
            </div>
            <div>
              <RemoveTrustedCanisterUtility
                canister_id={canister.canister_id}
                name={canister.name}
              />
            </div>
          </div>
        ))}
        <AddTrustedCanisterUtility />
      </div>
    );
  };

  const renderScreen = () => {
    return (
      <div className="max-w-md">
        {renderTrustedCanisters()}
        <SignOutUtility />
        <UIModeSwitch />
      </div>
    );
  };

  return (
    <AuthGate>
      <Layout>
        <div className="container px-4 mx-auto">
          <h1 className="text-xl font-bold mb-4 mt-8">Settings</h1>
          {loading ? <LoadingIndicator /> : renderScreen()}
        </div>
      </Layout>
    </AuthGate>
  );
};
