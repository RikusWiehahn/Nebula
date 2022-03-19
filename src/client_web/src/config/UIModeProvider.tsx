import React, { useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { ToastContainer } from "react-toastify";
import { StoreState } from "./ReduxStore";
import { updateConfigState } from "../config/_Actions";

export const UIModeProvider = ({
  children,
}: {
  children: JSX.Element | JSX.Element[];
}) => {
  const config = useSelector((state: StoreState) => state.config);
  const dispatch = useDispatch();

  useEffect(() => {
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      if (!config.uiModeWasManuallySet) {
        dispatch(
          updateConfigState({
            ...config,
            uiMode: "dark",
            uiModeWasManuallySet: true,
          })
        );
      }
    }
  }, []);

  return (
    <div className={`${config.uiMode === "light" ? "light" : "dark"}`}>
      <ToastContainer
        position="bottom-right"
        closeButton={false}
        autoClose={4500}
        hideProgressBar={true}
        newestOnTop={false}
        closeOnClick
        rtl={false}
        draggable={false}
        pauseOnHover
        theme={config.uiMode === "light" ? "light" : "dark"}
      />
      {children}
    </div>
  );
};
