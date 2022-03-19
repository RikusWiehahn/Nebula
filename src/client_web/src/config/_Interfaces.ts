//
//   ####   ####  #    # ###### #  ####
//  #    # #    # ##   # #      # #    #
//  #      #    # # #  # #####  # #
//  #      #    # #  # # #      # #  ###
//  #    # #    # #   ## #      # #    #
//   ####   ####  #    # #      #  ####

import * as Backend from "../../../declarations/backend/backend.did";

export interface ConfigState {
  uiMode: "light" | "dark";
  uiModeWasManuallySet: boolean;
}

export const EMPTY_CONFIG_STATE: ConfigState = {
  uiMode: "light",
  uiModeWasManuallySet: false,
};

//
//  #    #  ####  ###### #####       ##    ####   ####   ####  #    # #    # #####
//  #    # #      #      #    #     #  #  #    # #    # #    # #    # ##   #   #
//  #    #  ####  #####  #    #    #    # #      #      #    # #    # # #  #   #
//  #    #      # #      #####     ###### #      #      #    # #    # #  # #   #
//  #    # #    # #      #   #     #    # #    # #    # #    # #    # #   ##   #
//   ####   ####  ###### #    #    #    #  ####   ####   ####   ####  #    #   #

export interface UserAccount {
  id: string;
  token: string;
  given_name: string;
  family_name: string;
  avatar: string;
  avatar_canister_id: string;
  email: string;
  email_verified: boolean;
  groups: string[];
  groups_list: Backend.Group[];
  last_active: number;
}

export const EMPTY_USER_ACCOUNT: UserAccount = {
  id: "",
  token: "",
  given_name: "",
  family_name: "",
  avatar: "",
  avatar_canister_id: "",
  email: "",
  email_verified: false,
  groups: [],
  groups_list: [],
  last_active: 0,
};

//
//   ####  #####   ####  #    # #####
//  #    # #    # #    # #    # #    #
//  #      #    # #    # #    # #    #
//  #  ### #####  #    # #    # #####
//  #    # #   #  #    # #    # #
//   ####  #    #  ####   ####  #

export interface Group {
  id: string;
  name: string;
  avatar: string;
  avatar_canister_id: string;
  members: string[];
  members_list: Backend.User[];
  admins: string[];
  proposal_vote_threshold: number;
  succession_timer_duration: number;
  folders: string[];
  folders_list: Backend.Folder[];
  proposals: string[];
  proposals_list: Backend.Proposal[];
}

export const proposalVoteThresholds: {
  label: string;
  value: string;
  description: string;
}[] = [
  {
    label: "100%",
    value: "1.00",
    description:
      "Admins must vote unanimously to make any changes to group settings and membership.",
  },
  {
    label: "75%",
    value: "0.75",
    description:
      "Over 75% of admin votes are required to make any changes to group settings and membership.",
  },
  {
    label: "50%",
    value: "0.50",
    description:
      "Over 50% of admin votes are required to make any changes to group settings and membership.",
  },
  {
    label: "25%",
    value: "0.25",
    description:
      "Over 25% of admin votes are required to make any changes to group settings and membership.",
  },
  {
    label: "0%",
    value: "0.00",
    description:
      "Any admin can make any changes to group settings and membership",
  },
];

export const successionTimerDurations: {
  label: string;
  value: string;
  description: string;
}[] = [
  {
    label: "6 Months",
    value: ((60 * 60 * 24 * 365) / 2).toFixed(0),
    description:
      "If you fail to log in for 6 months, your folders will execute the succession plan you have set.",
  },
  {
    label: "1 Year",
    value: (60 * 60 * 24 * 365).toFixed(0),
    description:
      "If you fail to log in for 1 year, your folders will execute the succession plan you have set.",
  },
  {
    label: "2 Years",
    value: (60 * 60 * 24 * 365 * 2).toFixed(0),
    description:
      "If you fail to log in for 2 years, your folders will execute the succession plan you have set.",
  },
  {
    label: "5 Years",
    value: (60 * 60 * 24 * 365 * 5).toFixed(0),
    description:
      "If you fail to log in for 5 years, your folders will execute the succession plan you have set.",
  },
];

export const EMPTY_GROUP = {
  id: "",
  name: "",
  avatar: "",
  avatar_canister_id: "",
  members: [],
  members_list: [],
  admins: [],
  proposal_vote_threshold: 0,
  succession_timer_duration: 0,
  folders: [],
  folders_list: [],
  proposals: [],
  proposals_list: [],
};

//
//  #####  #####   ####  #####   ####   ####    ##   #
//  #    # #    # #    # #    # #    # #       #  #  #
//  #    # #    # #    # #    # #    #  ####  #    # #
//  #####  #####  #    # #####  #    #      # ###### #
//  #      #   #  #    # #      #    # #    # #    # #
//  #      #    #  ####  #       ####   ####  #    # ######

export interface Proposal {
  id: string;
  group_id: string;
  proposed_by: string;
  created_at: number;
  proposal_type: string;
  payload: string;
  votes_for: string[];
  votes_against: string[];
  proposal_resolution_count: number;
  adopted: boolean;
  pending: boolean;
  result_message: string;
}

export type ProposalType =
  | "INVITE_USER"
  | "REMOVE_USER"
  | "CHANGE_VOTE_THRESHOLD"
  | "CHANGE_SUCCESSION_TIMER_DURATION"
  | "CHANGE_GROUP_NAME"
  | "ADD_ADMIN"
  | "REMOVE_ADMIN";

export const proposalTypesList: { label: string; value: ProposalType }[] = [
  { label: "Change group name", value: "CHANGE_GROUP_NAME" },
  { label: "Invite member", value: "INVITE_USER" },
  { label: "Remove member", value: "REMOVE_USER" },
  { label: "Change vote threshold", value: "CHANGE_VOTE_THRESHOLD" },
  {
    label: "Change succession timer",
    value: "CHANGE_SUCCESSION_TIMER_DURATION",
  },
  {
    label: "Add administrator",
    value: "ADD_ADMIN",
  },
  {
    label: "Remove administrator",
    value: "REMOVE_ADMIN",
  },
];

export const EMPTY_PROPOSAL: Proposal = {
  id: "",
  proposed_by: "",
  created_at: 0,
  group_id: "",
  proposal_type: "",
  payload: "",
  votes_for: [],
  votes_against: [],
  proposal_resolution_count: 0,
  adopted: false,
  pending: false,
  result_message: "",
};

//
//  ######  ####  #      #####  ###### #####
//  #      #    # #      #    # #      #    #
//  #####  #    # #      #    # #####  #    #
//  #      #    # #      #    # #      #####
//  #      #    # #      #    # #      #   #
//  #       ####  ###### #####  ###### #    #

export interface Folder {
  id: string;
  group_id: string;
  name: string;
  owners: string[]; // view and edit
  viewers: string[]; // view only
  successors: string[]; // view and edit upon succession only.
  pages: string[];
}

export const EMPTY_FOLDER: Folder = {
  id: "",
  group_id: "",
  name: "",
  owners: [],
  viewers: [],
  successors: [],
  pages: [],
};

//
//  #####    ##    ####  ######
//  #    #  #  #  #    # #
//  #    # #    # #      #####
//  #####  ###### #  ### #
//  #      #    # #    # #
//  #      #    #  ####  ######

export interface Page {
  id: string;
  folder_id: string;
  name: string;
  created_at: number;
  updated_at: number;
  content: string[];
}

export const EMPTY_PAGE: Page = {
  id: "",
  folder_id: "",
  name: "",
  created_at: 0,
  updated_at: 0,
  content: [],
};
