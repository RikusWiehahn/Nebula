import {
  ConfigState,
  EMPTY_CONFIG_STATE,
  EMPTY_FOLDER,
  EMPTY_GROUP,
  EMPTY_PAGE,
  EMPTY_USER_ACCOUNT,
  Folder,
  Group,
  Page,
  UserAccount,
} from "./_Interfaces";

export const ConfigReducer = (
  state: ConfigState = { ...EMPTY_CONFIG_STATE },
  action: {
    type: "CONFIG_REDUCER";
    payload: ConfigState;
  }
) => {
  switch (action.type) {
    case "CONFIG_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};

export const UserAccountReducer = (
  state: UserAccount = { ...EMPTY_USER_ACCOUNT },
  action: {
    type: "USER_ACCOUNT_REDUCER";
    payload: UserAccount;
  }
) => {
  switch (action.type) {
    case "USER_ACCOUNT_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};

export const GroupReducer = (
  state: Group = { ...EMPTY_GROUP },
  action: {
    type: "GROUP_REDUCER";
    payload: Group;
  }
) => {
  switch (action.type) {
    case "GROUP_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};

export const FolderReducer = (
  state: Folder = { ...EMPTY_FOLDER },
  action: {
    type: "FOLDER_REDUCER";
    payload: Folder;
  }
) => {
  switch (action.type) {
    case "FOLDER_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};

export const PageReducer = (
  state: Page = { ...EMPTY_PAGE },
  action: {
    type: "PAGE_REDUCER";
    payload: Page;
  }
) => {
  switch (action.type) {
    case "PAGE_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};