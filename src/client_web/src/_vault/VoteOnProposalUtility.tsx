import { ExplainBox, InfoBox } from "./../components/InfoBox";
import React, { useEffect, useState } from "react";
import { ErrorToast, SuccessToast } from "../config/toast";
import { ModalWrapper } from "../components/ModalWrapper";
import { backend } from "../../../declarations/backend";
import { updateGroupState, updateUserAccount } from "../config/_Actions";
import { useDispatch, useSelector } from "react-redux";
import { StoreState } from "../config/ReduxStore";
import {
  Proposal,
  ProposalType,
  proposalTypesList,
  proposalVoteThresholds,
} from "../config/_Interfaces";
import { RiCheckLine, RiCloseLine, RiSettings3Line } from "react-icons/ri";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
import { ButtonGroupSelect } from "../components/ButtonGroupSelect";
dayjs.extend(relativeTime);

const voteOptions = [
  {
    value: "true",
    label: "Yes",
  },
  {
    value: "false",
    label: "No",
  },
];

interface Props {
  proposal: Proposal;
}

export const VoteOnProposalUtility = (props: Props) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [showModal, setShowModal] = useState<boolean>(false);
  const [state, setState] = useState<{
    is_for: boolean;
  }>({
    is_for: false,
  });
  const user = useSelector((s: StoreState) => s.user);
  const group = useSelector((s: StoreState) => s.group);
  const dispatch = useDispatch();

  const isResolved = props.proposal.pending === false;
  const adopted = props.proposal.adopted === true;
  const rejected = props.proposal.adopted === false;

  const voteOnProposal = async (): Promise<void> => {
    try {
      setLoading(true);
      const vote_res = await backend.voteOnProposal(user.token, {
        proposal_id: props.proposal.id,
        is_for: state.is_for,
      });
      if (vote_res.err) throw new Error(vote_res.err);
      const group_res = await backend.getGroup(user.token, group.id);
      if (group_res.err) throw new Error(group_res.err);
      const member_res = await backend.getGroupMembers(user.token, group.id);
      if (member_res.err) throw new Error(member_res.err);
      const proposals_res = await backend.getGroupProposals(
        user.token,
        group.id
      );
      if (proposals_res.err) throw new Error(proposals_res.err);
      SuccessToast(vote_res.ok[0] || "");
      dispatch(
        updateGroupState({
          ...group,
          ...group_res.ok[0],
          members_list: member_res.ok,
          proposals_list: proposals_res.ok,
        })
      );
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderVoteCount = () => {
    return (
      <>
        {isResolved ? (
          <div className="mb-4 mt-4 flex border">
            <div className="flex-1">
              <div className="border-b p-2">Votes for</div>
              {group.members_list.map((m) =>
                props.proposal.votes_for.includes(m.id) ? (
                  <div className="p-2">
                    {`${m.given_name} ${m.family_name}`}
                  </div>
                ) : null
              )}
            </div>
            <div className="flex-1">
              <div className="border-b p-2">Votes against</div>
              {group.members_list.map((m) =>
                props.proposal.votes_against.includes(m.id) ? (
                  <div className="p-2">
                    {`${m.given_name} ${m.family_name}`}
                  </div>
                ) : null
              )}
            </div>
          </div>
        ) : null}
        {isResolved ? <div className="font-bold">Result</div> : null}
        {adopted ? (
          <div>Proposal was adopted.</div>
        ) : rejected ? (
          <div>Proposal was rejected.</div>
        ) : null}
        {isResolved && props.proposal.result_message[0] ? (
          <div className="font-bold">Result message</div>
        ) : null}
        {isResolved ? (
          <div className="">{props.proposal.result_message[0] || ""}</div>
        ) : null}
      </>
    );
  };

  const renderProposalType = () => {
    if (
      props.proposal.proposal_type === ("CHANGE_GROUP_NAME" as ProposalType)
    ) {
      return (
        <div>
          <label className="input-label">Change group name to</label>
          <div className="mb-4">{props.proposal.payload}</div>
          {!isResolved ? (
            <ButtonGroupSelect
              value={state.is_for ? "true" : "false"}
              options={voteOptions}
              onChange={(v) => {
                setState({
                  is_for: v === "true",
                });
              }}
            />
          ) : null}
          {renderVoteCount()}
          {!isResolved ? (
            <button
              className="btn-primary w-full"
              tabIndex={0}
              color="primary"
              onClick={voteOnProposal}
            >
              Submit vote
            </button>
          ) : null}
        </div>
      );
    } else if (
      props.proposal.proposal_type === ("INVITE_USER" as ProposalType)
    ) {
      return (
        <div>
          <label className="input-label">Invite new user</label>
          <div className="mb-4">{props.proposal.payload}</div>
          {!isResolved ? (
            <ButtonGroupSelect
              value={state.is_for ? "true" : "false"}
              options={voteOptions}
              onChange={(v) => {
                setState({
                  is_for: v === "true",
                });
              }}
            />
          ) : null}
          {renderVoteCount()}
          {!isResolved ? (
            <button
              className="btn-primary w-full"
              tabIndex={0}
              color="primary"
              onClick={voteOnProposal}
            >
              Submit vote
            </button>
          ) : null}
        </div>
      );
    } else if (
      props.proposal.proposal_type === ("REMOVE_USER" as ProposalType)
    ) {
      return (
        <div>
          <label className="input-label">Member to remove</label>
          <div className="mb-4">
            {group.members_list
              .map((m) =>
                m.id === props.proposal.payload
                  ? `${m.given_name} ${m.family_name}`
                  : ""
              )
              .reduce((a, b) => a + b, "")}
          </div>
          {!isResolved ? (
            <ButtonGroupSelect
              value={state.is_for ? "true" : "false"}
              options={voteOptions}
              onChange={(v) => {
                setState({
                  is_for: v === "true",
                });
              }}
            />
          ) : null}
          {renderVoteCount()}
          {!isResolved ? (
            <button
              className="btn-primary w-full"
              tabIndex={0}
              color="primary"
              onClick={voteOnProposal}
            >
              Submit vote
            </button>
          ) : null}
        </div>
      );
    } else if (
      props.proposal.proposal_type ===
      ("CHANGE_SUCCESSION_TIMER_DURATION" as ProposalType)
    ) {
      const successionDuration = dayjs()
        .add(parseFloat(props.proposal.payload), "seconds")
        .fromNow(true);

      return (
        <div>
          <label className="input-label">
            Change succession time period to
          </label>
          <div className="mb-4">
            <ExplainBox
              label={`Succession triggered after ${successionDuration} of inactivity`}
            />
          </div>
          {!isResolved ? (
            <ButtonGroupSelect
              value={state.is_for ? "true" : "false"}
              options={voteOptions}
              onChange={(v) => {
                setState({
                  is_for: v === "true",
                });
              }}
            />
          ) : null}
          {renderVoteCount()}
          {!isResolved ? (
            <button
              className="btn-primary w-full"
              tabIndex={0}
              color="primary"
              onClick={voteOnProposal}
            >
              Submit vote
            </button>
          ) : null}
        </div>
      );
    } else if (
      props.proposal.proposal_type === ("CHANGE_VOTE_THRESHOLD" as ProposalType)
    ) {
      return (
        <div>
          <label className="input-label">Change vote threshold</label>
          <div className="mb-4">
            {proposalVoteThresholds
              .map((t) => (t.value === props.proposal.payload ? t.label : ""))
              .reduce((a, b) => a + b, "")}
          </div>
          {!isResolved ? (
            <ButtonGroupSelect
              value={state.is_for ? "true" : "false"}
              options={voteOptions}
              onChange={(v) => {
                setState({
                  is_for: v === "true",
                });
              }}
            />
          ) : null}
          {renderVoteCount()}
          {!isResolved ? (
            <button
              className="btn-primary w-full"
              tabIndex={0}
              color="primary"
              onClick={voteOnProposal}
            >
              Submit vote
            </button>
          ) : null}
        </div>
      );
    } else if (props.proposal.proposal_type === ("ADD_ADMIN" as ProposalType)) {
      return (
        <div>
          <label className="input-label">Member to promote to admin</label>
          <div className="mb-4">
            {group.members_list
              .map((m) =>
                m.id === props.proposal.payload
                  ? `${m.given_name} ${m.family_name}`
                  : ""
              )
              .reduce((a, b) => a + b, "")}
          </div>
          {!isResolved ? (
            <ButtonGroupSelect
              value={state.is_for ? "true" : "false"}
              options={voteOptions}
              onChange={(v) => {
                setState({
                  is_for: v === "true",
                });
              }}
            />
          ) : null}
          {renderVoteCount()}
          {!isResolved ? (
            <button
              className="btn-primary w-full"
              tabIndex={0}
              color="primary"
              onClick={voteOnProposal}
            >
              Submit vote
            </button>
          ) : null}
        </div>
      );
    } else if (
      props.proposal.proposal_type === ("REMOVE_ADMIN" as ProposalType)
    ) {
      return (
        <div>
          <label className="input-label">Member to remove as admin</label>
          <div className="mb-4">
            {group.members_list
              .map((m) =>
                m.id === props.proposal.payload
                  ? `${m.given_name} ${m.family_name}`
                  : ""
              )
              .reduce((a, b) => a + b, "")}
          </div>
          {!isResolved ? (
            <ButtonGroupSelect
              value={state.is_for ? "true" : "false"}
              options={voteOptions}
              onChange={(v) => {
                setState({
                  is_for: v === "true",
                });
              }}
            />
          ) : null}
          {renderVoteCount()}
          {!isResolved ? (
            <button
              className="btn-primary w-full"
              tabIndex={0}
              color="primary"
              onClick={voteOnProposal}
            >
              Submit vote
            </button>
          ) : null}
        </div>
      );
    }
  };

  return (
    <div>
      <button
        className="bg-white hover:bg-gray-100 dark:bg-gray-700 dark:hover:bg-gray-600 duration-200 p-4 items-center flex w-full"
        onClick={() => setShowModal(true)}
      >
        {!isResolved ? (
          <RiSettings3Line className="mr-2" />
        ) : adopted ? (
          <RiCheckLine className="mr-2 text-green-500" />
        ) : (
          <RiCloseLine className="mr-2 text-red-500" />
        )}

        {proposalTypesList.find((t) => t.value === props.proposal.proposal_type)
          ?.label || ""}
      </button>

      <ModalWrapper
        headerLabel={
          proposalTypesList.find(
            (t) => t.value === props.proposal.proposal_type
          )?.label || ""
        }
        showModal={showModal}
        isLoading={loading}
        onClose={() => setShowModal(false)}
      >
        <div>{renderProposalType()}</div>
      </ModalWrapper>
    </div>
  );
};
