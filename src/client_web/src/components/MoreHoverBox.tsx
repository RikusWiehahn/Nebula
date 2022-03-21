import Popover from "@mui/material/Popover";
import React, { useEffect } from "react";
import { RiMoreFill } from "react-icons/ri";
import { useSelector } from "react-redux";
import { StoreState } from "../config/ReduxStore";

interface Props {
  children?: JSX.Element | JSX.Element[] | null;
  buttonClasses?: string;
  boxClasses?: string;
  customIcon?: JSX.Element;
}
export const MoreHoverBox = (props: Props) => {
  let generatedId = "";
  const uiMode = useSelector((state: StoreState) => state.config.uiMode);
  const [anchorEl, setAnchorEl] = React.useState<HTMLButtonElement | null>(
    null
  );

  const handleClick = (event: React.MouseEvent<HTMLButtonElement>) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  useEffect(() => {
    generatedId = `popover${Math.random}`;
  }, []);

  const open = Boolean(anchorEl);
  const id = open ? generatedId : undefined;

  return (
    <span>
      <button
        aria-describedby={id}
        onClick={handleClick}
        aria-label="More button"
        className={`h-12 w-12 rounded-full flex items-center justify-center overflow-hidden whitespace-nowrap font-semibold focus:outline-none duration-200 bg-gray-100 bg-opacity-80 hover:bg-opacity-90 text-black dark:text-white dark:bg-gray-700 dark:bg-opacity-80 dark:hover:bg-opacity-90 ${
          props.buttonClasses || ""
        }`}
      >
        {props.customIcon ? props.customIcon : <RiMoreFill />}
      </button>
      <Popover
        id={id}
        open={open}
        anchorEl={anchorEl}
        onClose={handleClose}
        PaperProps={{
          className: `w-64 overflow-visible bg-transparent ${
            uiMode === "light" ? "light" : "dark"
          }`,
        }}
        elevation={2}
        anchorOrigin={{
          vertical: "center",
          horizontal: "center",
        }}
        transformOrigin={{
          vertical: "top",
          horizontal: "left",
        }}
      >
        <div
          className={`bg-white dark:bg-gray-700 h-full text-gray-700 dark:text-white ${
            props.boxClasses || ""
          }`}
        >
          {props.children}
        </div>
      </Popover>
    </span>
  );
};
