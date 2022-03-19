import React, { useLayoutEffect, useState } from "react";
import { useEffect } from "react";

interface ChecklistItem {
  label: string;
  value: string;
}

export const Checklist = (props: {
  options: ChecklistItem[];
  selected: string[];
  onChange: (selected: string[]) => void;
}) => {
  
  const [state, setState] = useState<{
    selected: string[];
  }>({
    selected: [],
  });

  useEffect(() => {
    setState({ selected: props.selected });
  }, []);

  useEffect(() => {
    const selectionPropsChanged = () => {
      if (props.selected.length !== state.selected.length) return false;
      const hasSameValues = props.selected.every((propItem) => {
        const isIncluded: boolean = !!state.selected
          .map((stateItem) => (stateItem === propItem ? 1 : 0))
          .reduce((p: number, c) => p + c, 0);
        return isIncluded;
      });
      return !hasSameValues;
    };

    if (selectionPropsChanged()) {
      setState({ selected: props.selected });
    }
  }, [props.selected]);

  useEffect(() => {
    props.onChange(state.selected);
  }, [state.selected]);

  const list = props.options.map(({ label, value }, i) => {
    const bgColor = i % 2 == 1 ? "bg-white dark:bg-gray-700" : "bg-gray-100 dark:bg-gray-800";
    return (
      <label
        key={value}
        className={`${bgColor} px-4 py-2 flex items-center cursor-pointer gap-8`}
      >
        <span className="flex-1">{label}</span>
        <div className="w-16 flex justify-center items-center">
          <input
            type="checkbox"
            className="form-checkbox border h-4 w-4"
            checked={state.selected.includes(value)}
            onChange={() => {
              let selected: string[] = [...state.selected];
              const i = selected.indexOf(value);
              if (i !== -1) selected.splice(i, 1);
              else selected.push(value);
              setState({
                ...state,
                selected,
              });
            }}
          />
        </div>
      </label>
    );
  });

  return <div className="border rounded-md overflow-hidden">{list}</div>;
};
