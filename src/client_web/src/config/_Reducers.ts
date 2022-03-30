import {
  ConfigState,
  EMPTY_CONFIG_STATE,
  AuthSession,
  EMPTY_AUTH_SESSION,
  EMPTY_TELEMETRY,
  Telemetry,
  EMPTY_MODEL_LIST,
  ModelList,
  Model,
  EMPTY_MODEL,
  EMPTY_MODEL_TABLE,
  ModelTable,
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

export const AuthSessionReducer = (
  state: AuthSession = { ...EMPTY_AUTH_SESSION },
  action: {
    type: "AUTH_SESSION_REDUCER";
    payload: AuthSession;
  }
) => {
  switch (action.type) {
    case "AUTH_SESSION_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};

export const TelemetryReducer = (
  state: Telemetry = { ...EMPTY_TELEMETRY },
  action: {
    type: "TELEMETRY_REDUCER";
    payload: Telemetry;
  }
) => {
  switch (action.type) {
    case "TELEMETRY_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};

export const ModelListReducer = (
  state: ModelList = { ...EMPTY_MODEL_LIST },
  action: {
    type: "MODEL_LIST_REDUCER";
    payload: ModelList;
  }
) => {
  switch (action.type) {
    case "MODEL_LIST_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};

export const ModelTableReducer = (
  state: ModelTable = { ...EMPTY_MODEL_TABLE },
  action: {
    type: "MODEL_TABLE_REDUCER";
    payload: ModelTable;
  }
) => {
  switch (action.type) {
    case "MODEL_TABLE_REDUCER": {
      return {
        ...state,
        ...action.payload,
      };
    }
    default:
      return state;
  }
};
