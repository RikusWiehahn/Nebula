import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Layout } from "../components/Layout";
import { AuthGate } from "../_user/AuthGate";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import { StoreState } from "../config/ReduxStore";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { backend } from "../../../declarations/backend";
import { ErrorToast } from "../config/toast";
import { RiTableLine } from "react-icons/ri";
import {
  updateModelListState,
  updateRecordTableState,
} from "../config/_Actions";
import { CreateRecordUtility } from "./CreateRecordUtility";
import { RecordsTable } from "./RecordsTable";
import { EMPTY_RECORD } from "../config/_Interfaces";
dayjs.extend(relativeTime);

export const RecordTablesScreen = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const model_list = useSelector((s: StoreState) => s.model_list);
  const record_table = useSelector((s: StoreState) => s.record_table);

  useEffect(() => {
    if (auth.token) {
      getModels();
    }
  }, [auth.token]);

  const getModels = async () => {
    try {
      const { token } = auth;
      setLoading(true);
      const models_res = await backend.get_models({ token });
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

  const getRecords = async (model_name: string) => {
    try {
      const { token } = auth;
      setLoading(true);
      const records_res = await backend.get_record_list({
        token,
        page: 1,
        page_size: 100,
        model_name,
      });
      if (records_res.err) throw new Error(records_res.err);
      if (!records_res.ok) throw new Error("Failed to get models.");

      dispatch(
        updateRecordTableState({
          page: records_res.page,
          page_size: records_res.page_size,
          model_name,
          data_fields: record_table.data_fields,
          records: records_res.ok.map((r: any) => {
            return {
              ...JSON.parse(r),
            };
          }),
        })
      );

      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const selectModel = (model_name: string) => {
    const model = model_list.models.find((m) => m.model_name === model_name);
    if (!model) return;
    dispatch(
      updateRecordTableState({
        model_name: model.model_name,
        data_fields: model.data_fields,
        records: [],
        page: 1,
        page_size: 100,
      })
    );
    getRecords(model.model_name);
  };

  const renderModelTabs = () => {
    return (
      <div>
        <div className="flex mb-4">
          <div className="text-center">
            {model_list.models.length === 0 ? "No models created yet." : null}
          </div>
          {model_list.models.map((model) => (
            <div key={model.model_name}>
              <button
                className={`btn-list ${
                  model.model_name === record_table.model_name ? "border-b" : ""
                }`}
                onClick={() => {
                  selectModel(model.model_name);
                }}
              >
                <RiTableLine className="mr-4" />
                {model.model_name}
              </button>
            </div>
          ))}
        </div>
        <div>
          {record_table.model_name ? (
            <CreateRecordUtility
              key={record_table.model_name}
              model={{
                model_name: record_table.model_name,
                data_fields: record_table.data_fields,
              }}
            />
          ) : null}
          {record_table.model_name ? <RecordsTable /> : null}
        </div>
      </div>
    );
  };

  return (
    <div>
      <AuthGate>
        <Layout>
          <div className="mx-auto container px-4 py-4">
            <div className="relative">
              <div className="absolute w-full">
                {loading ? <LoadingIndicator /> : null}
              </div>
            </div>
            {renderModelTabs()}
          </div>
        </Layout>
      </AuthGate>
    </div>
  );
};
