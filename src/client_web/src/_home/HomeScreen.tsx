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
    getSystemStatus();
  }, []);

  const getSystemStatus = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const telemetry_res = await backend.getSystemTelemetry(token);
      if (telemetry_res.err) throw new Error(telemetry_res.err);
      if (!telemetry_res.ok?.[0]) throw new Error("Failed to get telemetry.");
      dispatch(
        updateTelemetryState({
          ...telemetry,
          ...telemetry_res.ok[0],
        })
      );
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderMainCanisterTelemetry = () => {
    const last_status = dayjs(telemetry.last_status_check / 1000000);
    const renderStat = (title: string, label: string) => (
      <div className="bg-white dark:bg-gray-800 p-4">
        <div className="text-sm">{title}</div>
        <div className="font-bold">{label}</div>
      </div>
    );

    return (
      <div className="py-4">
        <h1 className="text-xl font-bold mt-4 mb-4">{`Main Canister ID: ${telemetry.main_id}`}</h1>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          {renderStat(
            "Last Status Check",
            last_status.format("hh:mm A ddd DD MMM YYYY")
          )}
          {renderStat(
            "Main canister cycles balance",
            `${(telemetry.main_cycles / 1_000_000_000_000).toFixed(
              2
            )} trillion cycles`
          )}
          {renderStat(
            "Main canister memory available",
            `${(telemetry.main_memory_size / 1000).toFixed(2)} kB`
          )}
          {renderStat(
            "Main canister memory used",
            `${(telemetry.main_memory_used / 1000).toFixed(2)} kB`
          )}
        </div>
      </div>
    );
  };

  return (
    <div>
      <AuthGate>
        <Layout>
          <div className="mx-auto container px-4">
            {loading ? <LoadingIndicator /> : renderMainCanisterTelemetry()}
          </div>
        </Layout>
      </AuthGate>
    </div>
  );
};
