import React, { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
dayjs.extend(relativeTime);
import { StoreState } from "../config/ReduxStore";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { backend } from "../../../declarations/backend";
import { ErrorToast } from "../config/toast";
import { updateRecordTableState } from "../config/_Actions";
import {
  DataFieldType,
  dataFieldTypesList,
  EMPTY_RECORD,
} from "../config/_Interfaces";

interface Props {}

export const RecordsTable = (props: Props) => {
  const [loading, setLoading] = useState<boolean>(false);
  const dispatch = useDispatch();
  const record_table = useSelector((s: StoreState) => s.record_table);
  const auth = useSelector((s: StoreState) => s.auth);

  useEffect(() => {
    if (auth.token && record_table.model_name) {
      getRecords();
    }
  }, [record_table.model_name]);

  const getRecords = async () => {
    try {
      const { token } = auth;
      const records_res = await backend.get_record_list({
        token,
        page: 1,
        page_size: 10,
        model_name: record_table.model_name,
      });
      if (records_res.err) throw new Error(records_res.err);
      if (!records_res.ok) throw new Error("Failed to get records");
      dispatch(
        updateRecordTableState({
          page: records_res.page,
          page_size: records_res.page_size,
          model_name: record_table.model_name,
          data_fields: record_table.data_fields,
          records: records_res.ok.map((r: any) => {
            return {
              ...JSON.parse(r),
            };
          }),
        })
      );
    } catch (e: any) {
      console.log({ e });
      ErrorToast(e.message);
    }
  };

  const renderTable = () => {
    let field_headings = record_table.data_fields.map((f) => f);
    field_headings.push({
      field_name: "id",
      data_type: DataFieldType.STRING,
      default_json_value: "",
    });
    field_headings.push({
      field_name: "model_name",
      data_type: DataFieldType.STRING,
      default_json_value: "",
    });
    field_headings.sort((a, b) => a.field_name.localeCompare(b.field_name));

    return (
      <div>
        <table className="w-full mt-4">
          <thead>
            <tr>
              {field_headings.map((f) => {
                return (
                  <td className="font-bold border-b p-4" key={f.field_name}>
                    {f.field_name}
                  </td>
                );
              })}
            </tr>
          </thead>
          <tbody>
            {record_table.records.map((r) => {
              let fields = Object.entries(r);
              fields.sort((a, b) => a[0].localeCompare(b[0]));
              console.log(fields);
              return (
                <tr key={r.id}>
                  {fields.map((f) => {
                    return <td className="p-4 text-left">{`${f[1]}`}</td>;
                  })}
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    );
  };

  return <div>{loading ? <LoadingIndicator /> : renderTable()}</div>;
};
