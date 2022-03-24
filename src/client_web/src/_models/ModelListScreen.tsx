import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { Layout } from "../components/Layout";
import { AuthGate } from "../_user/AuthGate";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import { backend } from "../../../declarations/backend";
import { StoreState } from "../config/ReduxStore";
import { updateModelListState, updateTelemetryState } from "../config/_Actions";
import { ErrorToast } from "../config/toast";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { CreateDatModelUtility } from "./CreateDataModelUtility";
import { MoreHoverBox } from "../components/MoreHoverBox";
import { AddDataFieldUtility } from "./AddDataFieldUtility";
import { RemoveDataFieldUtility } from "./RemoveDataFieldUtility";
import { DeleteModelUtility } from "./DeleteModelUtility";
import { dataFieldTypesList } from "../config/_Interfaces";
dayjs.extend(relativeTime);

export const ModelListScreen = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const auth = useSelector((s: StoreState) => s.auth);
  const telemetry = useSelector((s: StoreState) => s.telemetry);
  const model_list = useSelector((s: StoreState) => s.model_list);

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

  const renderModelList = () => {
    const { models } = model_list;
    if (models.length < 1)
      return <div className="text-center p-4">No data models found.</div>;
    const list = models.map((model) => {
      return (
        <div key={model.model_name} className="bg-white dark:bg-gray-800 p-4">
          <div className="flex justify-between items-center">
            <div className="font-bold text-xl">{model.model_name}</div>
            <MoreHoverBox>
              <AddDataFieldUtility model={model} />
              <RemoveDataFieldUtility model={model} />
              <DeleteModelUtility model={model} />
            </MoreHoverBox>
          </div>
          <div>
            <div className="flex mb-4  font-bold">
              <div className="flex-1">Data field</div>
              <div className="flex-1">Data type</div>
              <div className="flex-1">Default value</div>
            </div>
          </div>
          <div className="flex">
            <div className="flex-1">id</div>
            <div className="flex-1">String</div>
            <div className="flex-1">Auto-generated</div>
          </div>
          <div className="flex">
            <div className="flex-1">model_name</div>
            <div className="flex-1">String</div>
            <div className="flex-1">Auto-generated</div>
          </div>
          {model.data_fields.map((field) => (
            <div className="flex">
              <div className="flex-1">{field.field_name}</div>
              <div className="flex-1">
                {dataFieldTypesList
                  .map((t) => (t.value === field.data_type ? t.label : ""))
                  .reduce((a, b) => a + b)}
              </div>
              <div className="flex-1">{field.default_json_value}</div>
            </div>
          ))}
        </div>
      );
    });

    return (
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 py-4">{list}</div>
    );
  };

  const renderModels = () => {
    return (
      <div className="">
        <div className="">
          <CreateDatModelUtility />
        </div>
        {renderModelList()}
      </div>
    );
  };

  return (
    <div>
      <AuthGate>
        <Layout>
          <div className="mx-auto container px-4">
            <h1 className="text-xl mb-4 mt-8 font-bold">Data models</h1>
            {loading ? <LoadingIndicator /> : renderModels()}
          </div>
        </Layout>
      </AuthGate>
    </div>
  );
};
