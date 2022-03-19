import React, { useEffect, useState } from "react";
import { ErrorToast, SuccessToast } from "../config/toast";
import { ModalWrapper } from "../components/ModalWrapper";
import { backend } from "../../../declarations/backend";
import { updateGroupState, updateUserAccount } from "../config/_Actions";
import { useDispatch, useSelector } from "react-redux";
import { StoreState } from "../config/ReduxStore";
import { SelectDropDown } from "../components/SelectDropDown";
import {
  ProposalType,
  proposalTypesList,
  proposalVoteThresholds,
  successionTimerDurations,
} from "../config/_Interfaces";
import { RiAddLine, RiInformationLine } from "react-icons/ri";

interface Props {}

export const CreateProposalUtility = (props: Props) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [showModal, setShowModal] = useState<boolean>(false);
  const [state, setState] = useState<{
    proposal_type: string;
    proposal_payload: string;
  }>({
    proposal_type: "",
    proposal_payload: "",
  });
  const user = useSelector((s: StoreState) => s.user);
  const group = useSelector((s: StoreState) => s.group);
  const dispatch = useDispatch();

  const createProposal = async () => {
    try {
      setLoading(true);
      const create_res = await backend.createProposal(user.token, {
        group_id: group.id,
        proposal_type: state.proposal_type,
        payload: state.proposal_payload,
      });
      if (create_res.err) throw new Error(create_res.err);
      const proposals_res = await backend.getGroupProposals(
        user.token,
        group.id
      );
      if (proposals_res.err) throw new Error(proposals_res.err);
      dispatch(
        updateGroupState({
          ...group,
          ...create_res.ok[0],
          proposals_list: proposals_res.ok,
        })
      );
      SuccessToast("Proposal created successfully");
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
    }
  };

  const renderProposalType = () => {
    if (state.proposal_type === ("CHANGE_GROUP_NAME" as ProposalType)) {
      return (
        <div>
          <label className="input-label">New group name</label>
          <input
            className="input-primary mb-4"
            value={state.proposal_payload}
            onChange={(e) =>
              setState({ ...state, proposal_payload: e.target.value })
            }
            type="text"
            name="name"
            placeholder="Vault"
            onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
              if (ev.key === "Enter") {
              }
            }}
          />
          <button
            className="btn-primary w-full"
            tabIndex={0}
            color="primary"
            onClick={createProposal}
          >
            Submit proposal
          </button>
        </div>
      );
    } else if (state.proposal_type === ("INVITE_USER" as ProposalType)) {
      return (
        <div>
          <label className="input-label">Email for invite</label>
          <input
            className="input-primary mb-4"
            value={state.proposal_payload}
            onChange={(e) =>
              setState({ ...state, proposal_payload: e.target.value })
            }
            type="email"
            name="email"
            id="emailInput"
            placeholder="turgon@email.com"
            onKeyPress={(ev: React.KeyboardEvent<HTMLInputElement>) => {
              if (ev.key === "Enter") {
              }
            }}
          />
          <button
            className="btn-primary w-full"
            tabIndex={0}
            color="primary"
            onClick={createProposal}
          >
            Submit proposal
          </button>
        </div>
      );
    } else if (state.proposal_type === ("REMOVE_USER" as ProposalType)) {
      return (
        <div>
          <label className="input-label">Select member to remove</label>
          <SelectDropDown
            placeholder="Select member"
            options={group.members_list.map((m) => ({
              label: `${m.given_name} ${m.family_name}`,
              value: m.id,
            }))}
            value={state.proposal_payload}
            onChange={(value) => {
              setState({ ...state, proposal_payload: value });
            }}
          />
          <button
            className="btn-primary w-full"
            tabIndex={0}
            color="primary"
            onClick={createProposal}
          >
            Submit proposal
          </button>
        </div>
      );
    } else if (
      state.proposal_type ===
      ("CHANGE_SUCCESSION_TIMER_DURATION" as ProposalType)
    ) {
      return (
        <div>
          <label className="input-label">Change succession time period</label>
          <SelectDropDown
            placeholder="Select a time period"
            options={successionTimerDurations.map((m) => ({
              label: m.label,
              value: m.value,
            }))}
            value={state.proposal_payload}
            onChange={(value) => {
              setState({ ...state, proposal_payload: value });
            }}
          />
          <button
            className="btn-primary w-full"
            tabIndex={0}
            color="primary"
            onClick={createProposal}
          >
            Submit proposal
          </button>
        </div>
      );
    } else if (
      state.proposal_type === ("CHANGE_VOTE_THRESHOLD" as ProposalType)
    ) {
      return (
        <div>
          <label className="input-label">Change vote threshold</label>
          <SelectDropDown
            key={Math.random()}
            placeholder="Select a vote percentage"
            options={proposalVoteThresholds.map((m) => ({
              label: m.label,
              value: m.value,
            }))}
            value={state.proposal_payload}
            onChange={(value) => {
              setState({ ...state, proposal_payload: value });
            }}
          />
          <button
            className="btn-primary w-full"
            tabIndex={0}
            color="primary"
            onClick={createProposal}
          >
            Submit proposal
          </button>
        </div>
      );
    } else if (state.proposal_type === ("ADD_ADMIN" as ProposalType)) {
      return (
        <div>
          <label className="input-label">Select member to add as admin</label>
          <SelectDropDown
            placeholder="Select member"
            options={group.members_list
              .filter((m) => !group.admins.includes(m.id))
              .map((m) => ({
                label: `${m.given_name} ${m.family_name}`,
                value: m.id,
              }))}
            value={state.proposal_payload}
            onChange={(value) => {
              setState({ ...state, proposal_payload: value });
            }}
          />
          <button
            className="btn-primary w-full"
            tabIndex={0}
            color="primary"
            onClick={createProposal}
          >
            Submit proposal
          </button>
        </div>
      );
    } else if (state.proposal_type === ("REMOVE_ADMIN" as ProposalType)) {
      return (
        <div>
          <label className="input-label">Select member to remove as admin</label>
          <SelectDropDown
            placeholder="Select member"
            options={group.members_list
              .filter((m) => group.admins.includes(m.id))
              .map((m) => ({
                label: `${m.given_name} ${m.family_name}`,
                value: m.id,
              }))}
            value={state.proposal_payload}
            onChange={(value) => {
              setState({ ...state, proposal_payload: value });
            }}
          />
          <button
            className="btn-primary w-full"
            tabIndex={0}
            color="primary"
            onClick={createProposal}
          >
            Submit proposal
          </button>
        </div>
      );
    }
  };

  const renderForm = () => {
    return (
      <div>
        <label className="input-label">Proposal type</label>
        <SelectDropDown
          key={Math.random()}
          placeholder="Select a proposal type"
          options={proposalTypesList.map((t) => ({
            label: t.label,
            value: t.value,
          }))}
          value={state.proposal_type}
          onChange={(value) => {
            setState({ ...state, proposal_type: value, proposal_payload: "" });
          }}
        />
        {renderProposalType()}
      </div>
    );
  };

  return (
    <div>
      <button className="btn-list w-full" onClick={() => setShowModal(true)}>
        <RiAddLine className="mr-2" />
        Create a new proposal
      </button>

      <ModalWrapper
        headerLabel="Create a new proposal"
        showModal={showModal}
        isLoading={loading}
        onClose={() => setShowModal(false)}
      >
        {renderForm()}
      </ModalWrapper>
    </div>
  );
};
