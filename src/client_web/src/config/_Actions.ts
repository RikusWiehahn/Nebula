import {
  ConfigState,
  AuthSession,
  Telemetry,
  Model,
  ModelList,
} from "./_Interfaces";

export type ReducerType =
  | "CONFIG_REDUCER"
  | "AUTH_SESSION_REDUCER"
  | "TELEMETRY_REDUCER"
  | "MODEL_LIST_REDUCER"
  | "MODEL_REDUCER";

export const updateConfigState = (
  payload: ConfigState
): { type: ReducerType; payload: ConfigState } => ({
  type: "CONFIG_REDUCER",
  payload,
});

export const updateAuthSession = (
  payload: AuthSession
): { type: ReducerType; payload: AuthSession } => ({
  type: "AUTH_SESSION_REDUCER",
  payload,
});

export const updateTelemetryState = (
  payload: Telemetry
): { type: ReducerType; payload: Telemetry } => ({
  type: "TELEMETRY_REDUCER",
  payload,
});

export const updateModelListState = (
  payload: ModelList
): { type: ReducerType; payload: ModelList } => ({
  type: "MODEL_LIST_REDUCER",
  payload,
});

export const updateModelState = (
  payload: Model
): { type: ReducerType; payload: Model } => ({
  type: "MODEL_LIST_REDUCER",
  payload,
});
