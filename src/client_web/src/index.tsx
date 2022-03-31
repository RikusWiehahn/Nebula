import "./config/style.css";
import "react-toastify/dist/ReactToastify.css";
import * as React from "react";
import * as ReactDOM from "react-dom";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { Provider } from "react-redux";
import { persistor, store } from "./config/ReduxStore";
import { PersistGate } from "redux-persist/integration/react";
import { routes } from "./config/routes";
import { HomeScreen } from "./_home/HomeScreen";
import { UIModeProvider } from "./config/UIModeProvider";
import { ProfileScreen } from "./_user/ProfileScreen";
import { ModelListScreen } from "./_models/ModelListScreen";
import { RecordTablesScreen } from "./_tables/RecordTablesScreen";

const App = () => {
  const renderRoutes = () => {
    return (
      <Routes>
        <Route path={routes.MODEL_TYPES} element={<ModelListScreen />} />
        <Route path={routes.SETTINGS} element={<ProfileScreen />} />
        <Route path={routes.MODEL_TABLE_LIST} element={<RecordTablesScreen />} />
        <Route path={routes.HOME} element={<HomeScreen />} />
      </Routes>
    );
  };

  return (
    <div className="">
      <Provider store={store}>
        <PersistGate loading={<div>Loading...</div>} persistor={persistor}>
          <UIModeProvider>
            <BrowserRouter>{renderRoutes()}</BrowserRouter>
          </UIModeProvider>
        </PersistGate>
      </Provider>
    </div>
  );
};

ReactDOM.render(<App />, document.getElementById("root"));
