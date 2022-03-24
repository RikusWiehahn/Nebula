import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Layout } from "../components/Layout";
import { AuthGate } from "../_user/AuthGate";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import { backend } from "../../../declarations/backend";
import { StoreState } from "../config/ReduxStore";
import { updateTelemetryState } from "../config/_Actions";
import { ErrorToast } from "../config/toast";
import { LoadingIndicator } from "../components/LoadingIndicator";
dayjs.extend(relativeTime);

export const HomeScreen = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const telemetry = useSelector((s: StoreState) => s.telemetry);

  useEffect(() => {
    if (auth.token) {
      getSystemStatus();
    }
  }, [auth.token]);

  const getSystemStatus = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const telemetry_res = await backend.getSystemTelemetry({ token });

      if (telemetry_res.err) throw new Error(telemetry_res.err);
      if (!telemetry_res.ok?.[0]) throw new Error("Failed to get telemetry.");
      console.log(telemetry_res);
      dispatch(
        updateTelemetryState({
          ...telemetry,
          ...telemetry_res.ok[0],
        })
      );
      setLoading(false);
    } catch (e: any) {
      console.log({ e });
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderSubCanisterTelemetry = () => {
    const renderStat = (title: string, label: string) => (
      <div className="">
        <div className="text-sm">{title}</div>
        <div className="font-bold">{label}</div>
      </div>
    );

    return (
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {telemetry.sub_canisters.map((sub_canister) => {
          return (
            <div
              key={sub_canister.id}
              className="bg-white dark:bg-gray-800 p-4"
            >
              <div className="font-bold mb-4">{`Canister ID: ${sub_canister.id}`}</div>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {renderStat("Data model", sub_canister.model_name)}
                {renderStat(
                  "Cycles balance",
                  `${(sub_canister.cycles / 1_000_000_000_000).toFixed(
                    2
                  )} trillion cycles`
                )}
                {renderStat(
                  "Memory used",
                  `${(sub_canister.memory_used / 1000).toFixed(2)} kB`
                )}
              </div>
            </div>
          );
        })}
      </div>
    );
  };

  const renderMainCanisterTelemetry = () => {
    const renderStat = (title: string, label: string) => (
      <div className="">
        <div className="text-sm">{title}</div>
        <div className="font-bold">{label}</div>
      </div>
    );
    const last_status = dayjs(telemetry.last_status_check / 1000000);
    return (
      <div className="">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 p-4 bg-white dark:bg-gray-800">
          {renderStat(
            "Last Status Check",
            last_status.format("hh:mm:ss A ddd DD MMM YYYY")
          )}
          {renderStat(
            "Main canister cycles balance",
            `${(telemetry.main_cycles / 1_000_000_000_000).toFixed(
              2
            )} trillion cycles`
          )}
          {renderStat(
            "Main canister memory used",
            `${(telemetry.main_memory_used / 1000).toFixed(2)} kB`
          )}
          {renderStat(
            "Bucket canister installer size",
            `${(telemetry.bucket_wasm_size / 1000).toFixed(2)} kB`
          )}
        </div>
        <h1 className="text-xl font-bold mt-8 mb-4">Sub Canisters</h1>
        {renderSubCanisterTelemetry()}
      </div>
    );
  };

  return (
    <div>
      <AuthGate>
        <Layout>
          <div className="mx-auto container px-4">
            <h1 className="text-xl font-bold mt-8 mb-4">{`Main Canister ID: ${telemetry.main_id}`}</h1>
            {loading ? <LoadingIndicator /> : renderMainCanisterTelemetry()}
          </div>
        </Layout>
      </AuthGate>
    </div>
  );
};
