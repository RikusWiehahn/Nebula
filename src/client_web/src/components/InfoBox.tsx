import React from "react";
import { RiInformationLine } from "react-icons/ri";

export function InfoBox({ label }: { label: string }) {
  return (
    <div className=" text-sm mb-4 dark:bg-blue-900 bg-blue-200 text-blue-900 dark:text-blue-200 p-2 rounded-lg flex items-center">
      <div>
        <RiInformationLine className="mr-2" size={16} />
      </div>
      <div>{label}</div>
    </div>
  );
}

export function ExplainBox({ label }: { label: string }) {
  return (
    <div className="text-sm mb-2 p-2 rounded-lg flex items-center">
      <div>
        <RiInformationLine className="mr-2" size={16} />
      </div>
      <div>{label}</div>
    </div>
  );
}
