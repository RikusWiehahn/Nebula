export const idlFactory = ({ IDL }) => {
  const ChangeFolderUsers = IDL.Record({
    'owners' : IDL.Vec(IDL.Text),
    'viewers' : IDL.Vec(IDL.Text),
    'folder_id' : IDL.Text,
    'successors' : IDL.Vec(IDL.Text),
  });
  const Folder = IDL.Record({
    'id' : IDL.Text,
    'owners' : IDL.Vec(IDL.Text),
    'name' : IDL.Text,
    'group_id' : IDL.Text,
    'viewers' : IDL.Vec(IDL.Text),
    'pages' : IDL.Vec(IDL.Text),
    'successors' : IDL.Vec(IDL.Text),
    'succession_time_from' : IDL.Float64,
  });
  const FolderResponse = IDL.Record({
    'ok' : IDL.Opt(Folder),
    'err' : IDL.Text,
  });
  const CreateFolder = IDL.Record({
    'owners' : IDL.Vec(IDL.Text),
    'name' : IDL.Text,
    'group_id' : IDL.Text,
    'viewers' : IDL.Vec(IDL.Text),
    'successors' : IDL.Vec(IDL.Text),
  });
  const Group = IDL.Record({
    'id' : IDL.Text,
    'members' : IDL.Vec(IDL.Text),
    'name' : IDL.Text,
    'folders' : IDL.Vec(IDL.Text),
    'proposal_vote_threshold' : IDL.Float64,
    'succession_timer_duration' : IDL.Float64,
    'admins' : IDL.Vec(IDL.Text),
    'proposals' : IDL.Vec(IDL.Text),
    'avatar_canister_id' : IDL.Text,
    'avatar' : IDL.Text,
  });
  const GroupResponse = IDL.Record({ 'ok' : IDL.Opt(Group), 'err' : IDL.Text });
  const CreateGroup = IDL.Record({
    'name' : IDL.Text,
    'proposal_vote_threshold' : IDL.Float64,
    'succession_timer_duration' : IDL.Float64,
  });
  const BasicResponse = IDL.Record({
    'ok' : IDL.Opt(IDL.Text),
    'err' : IDL.Text,
  });
  const CreatePage = IDL.Record({ 'name' : IDL.Text, 'folder_id' : IDL.Text });
  const CreateProposal = IDL.Record({
    'group_id' : IDL.Text,
    'proposal_type' : IDL.Text,
    'payload' : IDL.Text,
  });
  const CreateUser = IDL.Record({
    'password' : IDL.Text,
    'family_name' : IDL.Text,
    'email' : IDL.Text,
    'given_name' : IDL.Text,
    'password_check' : IDL.Text,
    'invite_token' : IDL.Text,
  });
  const User = IDL.Record({
    'id' : IDL.Text,
    'password_hash' : IDL.Text,
    'groups' : IDL.Vec(IDL.Text),
    'family_name' : IDL.Text,
    'last_active' : IDL.Float64,
    'email' : IDL.Text,
    'given_name' : IDL.Text,
    'email_verified' : IDL.Bool,
    'avatar_canister_id' : IDL.Text,
    'avatar' : IDL.Text,
  });
  const UserResponse = IDL.Record({ 'ok' : IDL.Opt(User), 'err' : IDL.Text });
  const Page = IDL.Record({
    'id' : IDL.Text,
    'updated_at' : IDL.Float64,
    'content' : IDL.Vec(IDL.Text),
    'name' : IDL.Text,
    'created_at' : IDL.Float64,
    'folder_id' : IDL.Text,
  });
  const PageListResponse = IDL.Record({
    'ok' : IDL.Vec(Page),
    'err' : IDL.Text,
  });
  const FolderListResponse = IDL.Record({
    'ok' : IDL.Vec(Folder),
    'err' : IDL.Text,
  });
  const UserListResponse = IDL.Record({
    'ok' : IDL.Vec(User),
    'err' : IDL.Text,
  });
  const Proposal = IDL.Record({
    'id' : IDL.Text,
    'pending' : IDL.Bool,
    'result_message' : IDL.Text,
    'proposal_resolution_count' : IDL.Float64,
    'created_at' : IDL.Float64,
    'group_id' : IDL.Text,
    'votes_for' : IDL.Vec(IDL.Text),
    'proposal_type' : IDL.Text,
    'proposed_by' : IDL.Text,
    'payload' : IDL.Text,
    'adopted' : IDL.Bool,
    'votes_against' : IDL.Vec(IDL.Text),
  });
  const ProposalListResponse = IDL.Record({
    'ok' : IDL.Vec(Proposal),
    'err' : IDL.Text,
  });
  const PageResponse = IDL.Record({ 'ok' : IDL.Opt(Page), 'err' : IDL.Text });
  const GroupListResponse = IDL.Record({
    'ok' : IDL.Vec(Group),
    'err' : IDL.Text,
  });
  const Vote = IDL.Record({ 'is_for' : IDL.Bool, 'proposal_id' : IDL.Text });
  return IDL.Service({
    'changeFolderUsers' : IDL.Func(
        [IDL.Text, ChangeFolderUsers],
        [FolderResponse],
        [],
      ),
    'createFolder' : IDL.Func([IDL.Text, CreateFolder], [GroupResponse], []),
    'createGroup' : IDL.Func([IDL.Text, CreateGroup], [BasicResponse], []),
    'createPage' : IDL.Func([IDL.Text, CreatePage], [BasicResponse], []),
    'createProposal' : IDL.Func(
        [IDL.Text, CreateProposal],
        [GroupResponse],
        [],
      ),
    'createUser' : IDL.Func([CreateUser], [BasicResponse], []),
    'deleteFolder' : IDL.Func([IDL.Text, IDL.Text], [GroupResponse], []),
    'deletePage' : IDL.Func([IDL.Text, IDL.Text], [BasicResponse], []),
    'editUser' : IDL.Func([IDL.Text, User], [UserResponse], []),
    'editUserEmail' : IDL.Func([IDL.Text, IDL.Text], [UserResponse], []),
    'getFolder' : IDL.Func([IDL.Text, IDL.Text], [FolderResponse], ['query']),
    'getFolderPages' : IDL.Func(
        [IDL.Text, IDL.Text],
        [PageListResponse],
        ['query'],
      ),
    'getGroup' : IDL.Func([IDL.Text, IDL.Text], [GroupResponse], ['query']),
    'getGroupFolders' : IDL.Func(
        [IDL.Text, IDL.Text],
        [FolderListResponse],
        ['query'],
      ),
    'getGroupMembers' : IDL.Func(
        [IDL.Text, IDL.Text],
        [UserListResponse],
        ['query'],
      ),
    'getGroupProposals' : IDL.Func(
        [IDL.Text, IDL.Text],
        [ProposalListResponse],
        ['query'],
      ),
    'getPage' : IDL.Func([IDL.Text, IDL.Text], [PageResponse], ['query']),
    'getUserGroups' : IDL.Func([IDL.Text], [GroupListResponse], ['query']),
    'getUserWithToken' : IDL.Func([IDL.Text], [UserResponse], ['query']),
    'signInWithEmail' : IDL.Func([IDL.Text, IDL.Text], [BasicResponse], []),
    'updatePage' : IDL.Func([IDL.Text, Page], [PageResponse], []),
    'verifyUserEmail' : IDL.Func([IDL.Text], [BasicResponse], []),
    'voteOnProposal' : IDL.Func([IDL.Text, Vote], [BasicResponse], []),
  });
};
export const init = ({ IDL }) => { return []; };
