import React from "react";
import { RiCheckboxBlankCircleLine, RiLoader5Line } from "react-icons/ri";
interface Props {
  dark?: boolean;
  fullscreen?: boolean;
}

const _LoadingIndicator = ({ fullscreen }: Props) => (
  <div
    className={
      fullscreen
        ? "bg-black bg-opacity-25 text-black fixed top-0 left-0 bottom-0 right-0"
        : "transparent text-gray-800 dark:text-white"
    }
    style={{
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      flexDirection: "column",
    }}
  >
    <div>
      <div
        // className="animate-bounce"
        className="flex justify-center items-center"
        style={{
          position: "relative",
          padding: "2rem",
        }}
      >
        <RiCheckboxBlankCircleLine
          className="absolute text-gray-400 dark:text-gray-600"
          size={36}
        />
        <RiLoader5Line className="spin absolute" size={40} />
      </div>
    </div>
  </div>
);

export const LoadingIndicator = _LoadingIndicator;
