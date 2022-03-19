import {
  ConfigState,
  AuthSession,
  Telemetry,
} from "./_Interfaces";

export type ReducerType =
  | "CONFIG_REDUCER"
  | "AUTH_SESSION_REDUCER"
  | "TELEMETRY_REDUCER"

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