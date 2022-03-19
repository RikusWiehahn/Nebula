import React from "react";
import { RiArrowLeftLine, RiCloseLine } from "react-icons/ri";

export const CustomModalHeader = ({
  label,
  labelComponent,
  onClose,
  onBack,
}: {
  label?: string;
  labelComponent?: JSX.Element;
  onClose?: () => void;
  onBack?: () => void;
}) => {
  return (
    <div className="w-full flex justify-between items-center mb-4">
      {onBack ? (
        <div className="w-12">
          <button className="btn-circle" onClick={onBack}>
            <RiArrowLeftLine className="" size={24} />
          </button>
        </div>
      ) : null}
      <div className="">
        {label ? <div className="text-xl font-bold">{label}</div> : null}
        {labelComponent}
      </div>
      <div className="w-12">
        {onClose ? (
          <button className="btn-circle" onClick={onClose}>
            <RiCloseLine className="" size={24} />
          </button>
        ) : null}
      </div>
    </div>
  );
};
