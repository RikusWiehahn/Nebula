import React, { useEffect, useState } from "react";
import {
  RiAddLine,
  RiArrowUpDownLine,
  RiDropboxLine,
  RiFolder3Fill,
  RiFolder3Line,
  RiUser3Line,
} from "react-icons/ri";
import { useDispatch, useSelector } from "react-redux";
import { backend } from "../../../declarations/backend";
import { Layout } from "../components/Layout";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { StoreState } from "../config/ReduxStore";
import { ErrorToast } from "../config/toast";
import { updateGroupState } from "../config/_Actions";
import { AuthGate } from "../_user/AuthGate";
import { CreateFolderUtility } from "./CreateFolderUtility";
import { CreateGroupUtility } from "./CreateGroupUtility";
import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";
dayjs.extend(relativeTime);
import { ExplainBox, InfoBox } from "../components/InfoBox";
import { CreateProposalUtility } from "./CreateProposalUtility";
import { Group, proposalTypesList } from "../config/_Interfaces";
import { VoteOnProposalUtility } from "./VoteOnProposalUtility";

export const HomeScreen = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const user = useSelector((s: StoreState) => s.user);
  const group = useSelector((s: StoreState) => s.group);
  const dispatch = useDispatch();
  const successionDuration = dayjs()
    .add(group.succession_timer_duration, "seconds")
    .fromNow(true);

  useEffect(() => {
    if (user.token && user.groups.length > 0) {
      getGroup(user.groups[0]);
    }
  }, []);

  const getGroup = async (groupId: string): Promise<void> => {
    try {
      setLoading(true);
      const group_res = await backend.getGroup(user.token, groupId);
      if (group_res.err) throw new Error(group_res.err);
      const member_res = await backend.getGroupMembers(user.token, groupId);
      if (member_res.err) throw new Error(member_res.err);
      const proposals_res = await backend.getGroupProposals(user.token, groupId);
      if (proposals_res.err) throw new Error(proposals_res.err);
      const folders_res = await backend.getGroupFolders(user.token, groupId);
      if (folders_res.err) throw new Error(folders_res.err);
      dispatch(
        updateGroupState({
          ...group,
          ...group_res.ok[0],
          members_list: member_res.ok,
          proposals_list: proposals_res.ok,
          folders_list: folders_res.ok,
        })
      );
      setLoading(false);
    } catch (e: any) {
      ErrorToast(e.message);
      setLoading(false);
    }
  };

  const renderCard = ({
    key,
    name,
    icon,
  }: {
    key: string;
    name: string;
    icon: JSX.Element;
  }) => {
    return (
      <div
        key={key}
        className="bg-gray-100 dark:bg-gray-600 p-4 flex items-center"
      >
        <div className="mr-2">{icon}</div>
        <div>{name}</div>
      </div>
    );
  };

  const renderGroup = () => {
    const folder_elems = group.folders_list.map((folder) => {
      return renderCard({
        key: folder.id,
        name: folder.name,
        icon: <RiFolder3Line />,
      });
    });
    folder_elems.unshift(<CreateFolderUtility key={Math.random()} />);
    const member_elems = group.members_list.map((member) => {
      return renderCard({
        key: member.id,
        name: `${member.given_name} ${member.family_name}`,
        icon: <RiUser3Line />,
      });
    });
    const proposal_elems = group.proposals_list.map((proposal) => {
      return <VoteOnProposalUtility key={Math.random()} proposal={proposal} />;
    });
    proposal_elems.unshift(<CreateProposalUtility key={Math.random()} />);

    return (
      <div>
        <div className="flex items-center mt-8 mb-4">
          <button className="btn-circle mr-4">
            <RiArrowUpDownLine />
          </button>
          <h1 className="text-3xl font-bold">{group.name}</h1>
        </div>
        <div className="flex flex-wrap border-b-2 border-gray-300 dark:border-gray-700 mb-4 w-full">
          <ExplainBox
            label={`Succession triggered after ${successionDuration} of inactivity`}
          />
          <ExplainBox
            label={`${
              group.proposal_vote_threshold * 100
            }% of admin votes required to pass a proposal`}
          />
        </div>
        <h2 className="mt-4 mb-4 text-xl font-bold">Folders</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {folder_elems}
        </div>
        <h2 className="mt-4 mb-4 text-xl font-bold">Members</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {member_elems}
        </div>
        <h2 className="mt-4 mb-4 text-xl font-bold">Proposals</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {proposal_elems}
        </div>
      </div>
    );
  };

  const renderScreen = () => {
    return (
      <div className="container mx-auto px-4">
        {user.groups.length === 0 ? (
          <CreateGroupUtility largeButton />
        ) : (
          renderGroup()
        )}
      </div>
    );
  };

  return (
    <div>
      <AuthGate>
        <Layout>
          <>{loading ? <LoadingIndicator /> : renderScreen()}</>
        </Layout>
      </AuthGate>
    </div>
  );
};
