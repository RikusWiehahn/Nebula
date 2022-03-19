import { createStore, combineReducers, applyMiddleware } from "redux";
import autoMergeLevel2 from "redux-persist/es/stateReconciler/autoMergeLevel2";
import { persistStore, persistReducer } from "redux-persist";
import storage from "redux-persist/lib/storage";
// default reducers
import { ConfigState, AuthSession, Telemetry } from "./_Interfaces";
import {
  ConfigReducer,
  AuthSessionReducer,
  TelemetryReducer,
} from "./_Reducers";

export interface StoreState {
  config: ConfigState;
  auth: AuthSession;
  telemetry: Telemetry;
}

const reducers: any = combineReducers<StoreState>({
  config: ConfigReducer,
  auth: AuthSessionReducer,
  telemetry: TelemetryReducer,
});

const persistConfig = {
  key: "root",
  whitelist: ["config", "auth"],
  storage,
  // There is an issue in the source code of redux-persist (default setTimeout does not cleaning)
  timeout: 0, // <-- code checks for falsey so this should disable it
  stateReconciler: autoMergeLevel2,
};

const middlewares: any[] = [];

if (process.env.NODE_ENV === "development") {
  const { createLogger } = require(`redux-logger`);
  const logger = createLogger({ collapsed: true });
  middlewares.push(logger);
}

const persistedReducer = persistReducer(persistConfig, reducers);

export let store = createStore(
  persistedReducer,
  applyMiddleware(...middlewares)
);
export let persistor = persistStore(store);
