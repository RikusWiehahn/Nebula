import React from "react";
import { StoreState } from "../config/ReduxStore";
import { useDispatch, useSelector } from "react-redux";
import { updateConfigState } from "../config/_Actions";
import { RiMoonLine, RiSunLine } from "react-icons/ri";

export const UIModeSwitch = () => {
  const config = useSelector((state: StoreState) => state.config);
  const dispatch = useDispatch();

  return (
    <div>
      <div className="p-2">
        <button
          className="btn-circle"
          onClick={() => {
            dispatch(
              updateConfigState({
                ...config,
                uiMode: config.uiMode === "light" ? "dark" : "light",
              })
            );
          }}
        >
          {config.uiMode === "dark" ? <RiSunLine /> : <RiMoonLine />}
        </button>
      </div>
    </div>
  );
};
