import type { Principal } from '@dfinity/principal';
export interface BasicResponse { 'ok' : [] | [string], 'err' : string }
export interface ChangeFolderUsers {
  'owners' : Array<string>,
  'viewers' : Array<string>,
  'folder_id' : string,
  'successors' : Array<string>,
}
export interface CreateFolder {
  'owners' : Array<string>,
  'name' : string,
  'group_id' : string,
  'viewers' : Array<string>,
  'successors' : Array<string>,
}
export interface CreateGroup {
  'name' : string,
  'proposal_vote_threshold' : number,
  'succession_timer_duration' : number,
}
export interface CreatePage { 'name' : string, 'folder_id' : string }
export interface CreateProposal {
  'group_id' : string,
  'proposal_type' : string,
  'payload' : string,
}
export interface CreateUser {
  'password' : string,
  'family_name' : string,
  'email' : string,
  'given_name' : string,
  'password_check' : string,
  'invite_token' : string,
}
export interface Folder {
  'id' : string,
  'owners' : Array<string>,
  'name' : string,
  'group_id' : string,
  'viewers' : Array<string>,
  'pages' : Array<string>,
  'successors' : Array<string>,
  'succession_time_from' : number,
}
export interface FolderListResponse { 'ok' : Array<Folder>, 'err' : string }
export interface FolderResponse { 'ok' : [] | [Folder], 'err' : string }
export interface Group {
  'id' : string,
  'members' : Array<string>,
  'name' : string,
  'folders' : Array<string>,
  'proposal_vote_threshold' : number,
  'succession_timer_duration' : number,
  'admins' : Array<string>,
  'proposals' : Array<string>,
  'avatar_canister_id' : string,
  'avatar' : string,
}
export interface GroupListResponse { 'ok' : Array<Group>, 'err' : string }
export interface GroupResponse { 'ok' : [] | [Group], 'err' : string }
export interface Page {
  'id' : string,
  'updated_at' : number,
  'content' : Array<string>,
  'name' : string,
  'created_at' : number,
  'folder_id' : string,
}
export interface PageListResponse { 'ok' : Array<Page>, 'err' : string }
export interface PageResponse { 'ok' : [] | [Page], 'err' : string }
export interface Proposal {
  'id' : string,
  'pending' : boolean,
  'result_message' : string,
  'proposal_resolution_count' : number,
  'created_at' : number,
  'group_id' : string,
  'votes_for' : Array<string>,
  'proposal_type' : string,
  'proposed_by' : string,
  'payload' : string,
  'adopted' : boolean,
  'votes_against' : Array<string>,
}
export interface ProposalListResponse { 'ok' : Array<Proposal>, 'err' : string }
export interface User {
  'id' : string,
  'password_hash' : string,
  'groups' : Array<string>,
  'family_name' : string,
  'last_active' : number,
  'email' : string,
  'given_name' : string,
  'email_verified' : boolean,
  'avatar_canister_id' : string,
  'avatar' : string,
}
export interface UserListResponse { 'ok' : Array<User>, 'err' : string }
export interface UserResponse { 'ok' : [] | [User], 'err' : string }
export interface Vote { 'is_for' : boolean, 'proposal_id' : string }
export interface _SERVICE {
  'changeFolderUsers' : (arg_0: string, arg_1: ChangeFolderUsers) => Promise<
      FolderResponse
    >,
  'createFolder' : (arg_0: string, arg_1: CreateFolder) => Promise<
      GroupResponse
    >,
  'createGroup' : (arg_0: string, arg_1: CreateGroup) => Promise<BasicResponse>,
  'createPage' : (arg_0: string, arg_1: CreatePage) => Promise<BasicResponse>,
  'createProposal' : (arg_0: string, arg_1: CreateProposal) => Promise<
      GroupResponse
    >,
  'createUser' : (arg_0: CreateUser) => Promise<BasicResponse>,
  'deleteFolder' : (arg_0: string, arg_1: string) => Promise<GroupResponse>,
  'deletePage' : (arg_0: string, arg_1: string) => Promise<BasicResponse>,
  'editUser' : (arg_0: string, arg_1: User) => Promise<UserResponse>,
  'editUserEmail' : (arg_0: string, arg_1: string) => Promise<UserResponse>,
  'getFolder' : (arg_0: string, arg_1: string) => Promise<FolderResponse>,
  'getFolderPages' : (arg_0: string, arg_1: string) => Promise<
      PageListResponse
    >,
  'getGroup' : (arg_0: string, arg_1: string) => Promise<GroupResponse>,
  'getGroupFolders' : (arg_0: string, arg_1: string) => Promise<
      FolderListResponse
    >,
  'getGroupMembers' : (arg_0: string, arg_1: string) => Promise<
      UserListResponse
    >,
  'getGroupProposals' : (arg_0: string, arg_1: string) => Promise<
      ProposalListResponse
    >,
  'getPage' : (arg_0: string, arg_1: string) => Promise<PageResponse>,
  'getUserGroups' : (arg_0: string) => Promise<GroupListResponse>,
  'getUserWithToken' : (arg_0: string) => Promise<UserResponse>,
  'signInWithEmail' : (arg_0: string, arg_1: string) => Promise<BasicResponse>,
  'updatePage' : (arg_0: string, arg_1: Page) => Promise<PageResponse>,
  'verifyUserEmail' : (arg_0: string) => Promise<BasicResponse>,
  'voteOnProposal' : (arg_0: string, arg_1: Vote) => Promise<BasicResponse>,
}
