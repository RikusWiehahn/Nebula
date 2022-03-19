import { InfoBox } from "./../components/InfoBox";
import React, { useEffect, useState } from "react";
import { ErrorToast, SuccessToast } from "../config/toast";
import { ModalWrapper } from "../components/ModalWrapper";
import { backend } from "../../../declarations/backend";
import { updateGroupState, updateUserAccount } from "../config/_Actions";
import { useDispatch, useSelector } from "react-redux";
import { StoreState } from "../config/ReduxStore";
import { SelectDropDown } from "../components/SelectDropDown";
import {
  proposalVoteThresholds,
  successionTimerDurations,
} from "../config/_Interfaces";
import { RiInformationLine } from "react-icons/ri";
import { getgroups } from "process";

interface Props {
  largeButton?: boolean;
}

export const CreateGroupUtility = (props: Props) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [showModal, setShowModal] = useState<boolean>(false);
  const [state, setState] = useState<{
    name: string;
    proposal_vote_threshold: string | null;
    succession_timer_duration: string | null;
  }>({
    name: "",
    proposal_vote_threshold: null,
    succession_timer_duration: null,
  });
  const user = useSelector((s: StoreState) => s.user);
  const group = useSelector((s: StoreState) => s.group);
  const dispatch = useDispatch();

  const createGroup = async (): Promise<void> => {
    try {
      setLoading(true);
      if (state.proposal_vote_threshold === null) {
        throw new Error("Please select a proposal vote threshold");
      }
      if (state.succession_timer_duration === null) {
        throw new Error("Please select a succession timer duration");
      }
      const create_res = await backend.createGroup(user.token, {
        name: state.name,
        proposal_vote_threshold: parseFloat(state.proposal_vote_threshold),
        succession_timer_duration: parseFloat(state.succession_timer_duration),
      });
      if (create_res.err) throw new Error(create_res.err);
      if (!create_res.ok[0]) throw new Error("Failed to create group");
      const user_res = await backend.getUserWithToken(user.token);
      if (user_res.err) throw new Error(user_res.err);
      const group_res = await backend.getGroup(user.token, create_res.ok[0]);
      if (group_res.err) throw new Error(group_res.err);
      const members_res = await backend.getGroupMembers(user.token, create_res.ok[0]);
      if (members_res.err) throw new Error(members_res.err);
      dispatch(
        updateGroupState({
          ...group,
          ...group_res.ok[0],
          members_list: members_res.ok,
        })
      );
      dispatch(
        updateUserAccount({
          ...user,
          ...user_res.ok[0],
        })
      );
      SuccessToast("Group created successfully");
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderForm = () => {
    return (
      <div>
        <label className="input-label">Group name</label>
        <input
          className="input-primary mb-4"
          value={state.name}
          onChange={(e) => setState({ ...state, name: e.target.value })}
          type="text"
          name="name"
          placeholder="Family Vault"
          onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
            if (ev.key === "Enter") {
            }
          }}
        />
        <label className="input-label">Voting threshold</label>
        <SelectDropDown
          placeholder="Select a voting threshold"
          options={proposalVoteThresholds.map((t) => ({
            label: t.label,
            value: t.value,
          }))}
          value={state.proposal_vote_threshold || ""}
          onChange={(value) => {
            setState({ ...state, proposal_vote_threshold: value });
          }}
        />
        <div className="">
          {state.proposal_vote_threshold !== null ? (
            <InfoBox
              label={proposalVoteThresholds
                .map((t) =>
                  t.value === state.proposal_vote_threshold ? t.description : ""
                )
                .reduce((a, b) => a + b, "")}
            />
          ) : null}
        </div>
        <label className="input-label">Succession timer duration</label>
        <SelectDropDown
          placeholder="Select a succession timer duration"
          value={state.succession_timer_duration || ""}
          options={successionTimerDurations.map((t) => ({
            label: t.label,
            value: t.value,
          }))}
          onChange={(value) => {
            setState({
              ...state,
              succession_timer_duration: value,
            });
          }}
        />
        <div className="">
          {state.succession_timer_duration !== null ? (
            <InfoBox
              label={successionTimerDurations
                .map((t) =>
                  t.value === state.succession_timer_duration
                    ? t.description
                    : ""
                )
                .reduce((a, b) => a + b, "")}
            />
          ) : null}
        </div>
        <button
          className="btn-primary w-full"
          tabIndex={0}
          color="primary"
          onClick={createGroup}
        >
          Create group
        </button>
      </div>
    );
  };

  return (
    <div>
      {props.largeButton ? (
        <button
          className="w-full py-20 font-bold text-xl p-4 my-8 bg-gray-900 dark:bg-gray-100 dark:bg-opacity-10 bg-opacity-10 dark:hover:bg-opacity-20 hover:bg-opacity-20 duration-200"
          onClick={() => setShowModal(true)}
        >
          Step 1: Click here to create a group!
        </button>
      ) : (
        <button
          className="btn-primary w-full"
          onClick={() => setShowModal(true)}
        >
          Create a group!
        </button>
      )}
      <ModalWrapper
        headerLabel="Create a new group"
        showModal={showModal}
        isLoading={loading}
        onClose={() => setShowModal(false)}
      >
        {renderForm()}
      </ModalWrapper>
    </div>
  );
};
