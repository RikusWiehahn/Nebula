import React, { useEffect, useState } from "react";
import { ErrorToast, SuccessToast } from "../config/toast";
import { backend } from "../../../declarations/backend";
import { updateGroupState } from "../config/_Actions";
import { useDispatch, useSelector } from "react-redux";
import { StoreState } from "../config/ReduxStore";
import { RiAddLine, RiArrowRightLine } from "react-icons/ri";
import { Checklist } from "../components/Checklist";
import { ExplainBox } from "../components/InfoBox";
import {
  ModalWrapperSlideProps,
  MultipageModalWrapper,
} from "../components/MultipageModalWrapper";
import { CustomModalHeader } from "../components/CustomModalHeader";

interface Props {}

export const CreateFolderUtility = (props: Props) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [showModal, setShowModal] = useState<boolean>(false);
  const [state, setState] = useState<{
    name: string;
    viewers: string[];
    owners: string[];
    successors: string[];
  }>({
    name: "",
    viewers: [],
    owners: [],
    successors: [],
  });
  const user = useSelector((s: StoreState) => s.user);
  const group = useSelector((s: StoreState) => s.group);
  const dispatch = useDispatch();

  const createFolder = async () => {
    try {
      setLoading(true);
      const create_res = await backend.createFolder(user.token, {
        name: state.name,
        group_id: group.id,
        viewers: state.viewers,
        owners: state.owners,
        successors: state.successors,
      });
      if (create_res.err) throw new Error(create_res.err);
      const folders_res = await backend.getGroupFolders(user.token, group.id);
      if (folders_res.err) throw new Error(folders_res.err);
      dispatch(
        updateGroupState({
          ...group,
          ...create_res.ok[0],
          folders_list: folders_res.ok,
        })
      );
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderFormOne = (props: ModalWrapperSlideProps) => {
    return (
      <div>
        <CustomModalHeader
          label="Create a new folder"
          onClose={() => setShowModal(false)}
        />
        <label className="input-label">Folder name</label>
        <input
          className="input-primary mb-4"
          value={state.name}
          onChange={(e) => setState({ ...state, name: e.target.value })}
          type="text"
          name="name"
          placeholder="Backup codes"
          onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key === "Enter") {
            }
          }}
        />
        <button
          className="btn-primary w-full"
          tabIndex={0}
          color="primary"
          onClick={() => (props.next ? props.next() : null)}
        >
          Next
          <RiArrowRightLine className="ml-2" />
        </button>
      </div>
    );
  };

  const renderFormTwo = (props: ModalWrapperSlideProps) => {
    return (
      <div>
        <CustomModalHeader
          label="Create a new folder"
          onBack={() => (props && props.back ? props.back() : null)}
        />
        <label className="input-label">Select folder owners</label>
        <ExplainBox label="Owners have full editing access" />
        <Checklist
          options={group.members_list.map((m) => ({
            label: `${m.given_name} ${m.family_name}`,
            value: m.id,
          }))}
          selected={state.owners}
          onChange={(value) => {
            setState({ ...state, owners: value });
          }}
        />
        <button
          className="btn-primary w-full mt-4"
          tabIndex={0}
          color="primary"
          onClick={() => (props.next ? props.next() : null)}
        >
          Next
          <RiArrowRightLine className="ml-2" />
        </button>
      </div>
    );
  };

  const renderFormThree = (props: ModalWrapperSlideProps) => {
    return (
      <div>
        <CustomModalHeader
          label="Create a new folder"
          onBack={() => (props && props.back ? props.back() : null)}
        />
        <label className="input-label">Select folder viewers</label>
        <ExplainBox label="Viewers have read-only access" />
        <Checklist
          options={group.members_list
            .filter((m) => !state.owners.includes(m.id))
            .map((m) => ({
              label: `${m.given_name} ${m.family_name}`,
              value: m.id,
            }))}
          selected={state.viewers}
          onChange={(value) => {
            setState({ ...state, viewers: value });
          }}
        />
        <button
          className="btn-primary w-full mt-4"
          tabIndex={0}
          color="primary"
          onClick={() => (props.next ? props.next() : null)}
        >
          Next
          <RiArrowRightLine className="ml-2" />
        </button>
      </div>
    );
  };

  const renderFormFour = (props: ModalWrapperSlideProps) => {
    return (
      <div>
        <CustomModalHeader
          label="Create a new folder"
          onBack={() => (props && props.back ? props.back() : null)}
        />
        <label className="input-label">Select folder successors</label>
        <ExplainBox label="Successors have no access until succession is triggered, then they become owners." />
        <Checklist
          options={group.members_list
            .filter((m) => !state.owners.includes(m.id))
            .map((m) => ({
              label: `${m.given_name} ${m.family_name}`,
              value: m.id,
            }))}
          selected={state.successors}
          onChange={(value) => {
            setState({ ...state, successors: value });
          }}
        />
        <button
          className="btn-primary w-full mt-4"
          tabIndex={0}
          color="primary"
          onClick={createFolder}
        >
          Create new folder
        </button>
      </div>
    );
  };

  return (
    <div>
      <button className="btn-list w-full" onClick={() => setShowModal(true)}>
        <RiAddLine className="mr-2" />
        Create new folder
      </button>
      <MultipageModalWrapper
        showModal={showModal}
        isLoading={loading}
        onClose={() => setShowModal(false)}
        slides={[renderFormOne, renderFormTwo, renderFormThree, renderFormFour]}
      />
    </div>
  );
};
