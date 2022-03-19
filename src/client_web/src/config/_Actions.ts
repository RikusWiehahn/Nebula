import {
  ConfigState,
  Folder,
  Group,
  Page,
  UserAccount,
} from "./_Interfaces";

export type ReducerType =
  | "CONFIG_REDUCER"
  | "USER_ACCOUNT_REDUCER"
  | "GROUP_REDUCER"
  | "FOLDER_REDUCER"
  | "PAGE_REDUCER";

export const updateConfigState = (
  payload: ConfigState
): { type: ReducerType; payload: ConfigState } => ({
  type: "CONFIG_REDUCER",
  payload,
});

export const updateUserAccount = (
  payload: UserAccount
): { type: ReducerType; payload: UserAccount } => ({
  type: "USER_ACCOUNT_REDUCER",
  payload,
});

export const updateGroupState = (
  payload: Group
): { type: ReducerType; payload: Group } => ({
  type: "GROUP_REDUCER",
  payload,
});

export const updateFolderState = (
  payload: Folder
): { type: ReducerType; payload: Folder } => ({
  type: "FOLDER_REDUCER",
  payload,
});

export const updatePageState = (
  payload: Page
): { type: ReducerType; payload: Page } => ({
  type: "PAGE_REDUCER",
  payload,
});