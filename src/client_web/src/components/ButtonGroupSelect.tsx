import React from "react";

export const ButtonGroupSelect = ({
  value,
  options,
  onChange,
  className,
}: {
  value: string;
  options: { label: string; value: string }[];
  onChange: (value: string) => void;
  className?: string;
}) => {
  return (
    <div
      className={`my-4 flex flex-wrap w-full gap-4 ${className || ""}`}
    >
      {options.map((option, i) => {
        return (
          <button
            key={`${value}-${i}`}
            onClick={() => onChange(option.value)}
            className={`flex-1 rounded-md font-bold p-4 ${
              value === option.value
                ? "bg-gray-800 text-gray-100 dark:bg-gray-200 dark:text-gray-800"
                : "border border-gray-300 text-gray-300 dark:text-gray-600 dark:border-gray-600"
            }`}
          >
            {option.label}
          </button>
        );
      })}
    </div>
  );
};
