import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Layout } from "../components/Layout";
import { AuthGate } from "../_user/AuthGate";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import { StoreState } from "../config/ReduxStore";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { backend } from "../../../declarations/backend";
import { updateModelListState } from "../config/_Actions";
import { ErrorToast } from "../config/toast";
import { RiTableLine } from "react-icons/ri";
dayjs.extend(relativeTime);

export const ModelTablesScreen = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const telemetry = useSelector((s: StoreState) => s.telemetry);
  const model_list = useSelector((s: StoreState) => s.model_list);
  const [model_name, setModelName] = useState<string>("");

  useEffect(() => {
    if (auth.token) {
      getModels();
    }
  }, [auth.token]);

  const getModels = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const models_res = await backend.getModels({ token });
      if (models_res.err) throw new Error(models_res.err);
      if (!models_res.ok) throw new Error("Failed to get models.");
      dispatch(
        updateModelListState({
          ...model_list,
          models: models_res.ok,
        })
      );
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderModelTabs = () => {
    return (
      <div className="flex">
        {model_list.models.map((model) => (
          <div key={model.model_name}>
            <button
              className="btn-list"
              onClick={() => setModelName(model.model_name)}
            >
              <RiTableLine className="mr-4" />
              {model.model_name}
            </button>
          </div>
        ))}
      </div>
    );
  };

  return (
    <div>
      <AuthGate>
        <Layout>
          <div className="mx-auto container px-4">
            <h1 className="text-xl mb-4 mt-8 font-bold">Model tables</h1>
            {loading ? <LoadingIndicator /> : renderModelTabs()}
          </div>
        </Layout>
      </AuthGate>
    </div>
  );
};
