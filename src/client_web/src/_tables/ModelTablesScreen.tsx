import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Layout } from "../components/Layout";
import { AuthGate } from "../_user/AuthGate";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import { StoreState } from "../config/ReduxStore";
import { LoadingIndicator } from "../components/LoadingIndicator";
dayjs.extend(relativeTime);

export const ModelTablesScreen = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const telemetry = useSelector((s: StoreState) => s.telemetry);
  const model_list = useSelector((s: StoreState) => s.model_list);

  const renderModels = () => {
    return (
      <div className="">
        
      </div>
    );
  };

  return (
    <div>
      <AuthGate>
        <Layout>
          <div className="mx-auto container px-4">
            <h1 className="text-xl mb-4 mt-8 font-bold">Model tables</h1>
            {loading ? <LoadingIndicator /> : renderModels()}
          </div>
        </Layout>
      </AuthGate>
    </div>
  );
};
