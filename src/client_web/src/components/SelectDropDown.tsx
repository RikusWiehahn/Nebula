import React from "react";

export const SelectDropDown = (props: {
  value?: string;
  placeholder?: string;
  options: {
    value: string;
    label: string;
  }[];
  onChange: (value: string) => void;
}) => {
  return (
    <select
      className="input-primary mb-4 mt-2"
      value={props.value || ""}
      onChange={({ target }) => {
        if (!target.value) return;
        props.onChange(target.value);
      }}
    >
      <option key="placeholder" value="" disabled>
        {props.placeholder || "Select an option"}
      </option>
      {props.options.map(({ value, label }) => {
        return (
          <option key={value} value={value}>
            {label}
          </option>
        );
      })}
    </select>
  );
};
