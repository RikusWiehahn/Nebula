import React from "react";
import { RiArrowLeftLine } from "react-icons/ri";

interface Props {
  children: JSX.Element | JSX.Element[];
  header?: string;
  onPressBack?: () => void;
}

export const SecondaryLayout = (props: Props) => {
  return (
    <div className="bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-white">
      <div className="flex flex-col" style={{ minHeight: "calc(100vh)" }}>
        <div className="border-b-4 border-gray-300 dark:border-gray-700 h-24 mx-4 flex items-center">
          <button className="btn-circle" onClick={props.onPressBack}>
            <RiArrowLeftLine size={24} className="" />
          </button>
          <h1 className="text-3xl font-semibold ml-4">{props.header}</h1>
        </div>
        <div className="container mx-auto px-4 flex-1 flex flex-col">
          {props.children}
        </div>
      </div>
    </div>
  );
};
