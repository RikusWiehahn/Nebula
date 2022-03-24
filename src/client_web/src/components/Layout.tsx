import React, { useEffect, useState } from "react";
import { routes } from "../config/routes";
import { Link, NavLink } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import { StoreState } from "../config/ReduxStore";
import { RiBarChart2Fill, RiBarChart2Line, RiBarChartFill, RiBarChartLine, RiEditBoxFill, RiEditBoxLine, RiSettings3Line, RiTableLine, RiUser3Line } from "react-icons/ri";

interface Props {
  children: JSX.Element | null;
  title?: string;
  header?: string;
}

export const Layout = (props: Props) => {
  const config = useSelector((state: StoreState) => state.config);
  const dispatch = useDispatch();

  const renderNavButtons = () => {
    return (
      <div className="flex">
        <NavLink to={routes.HOME}>
          <div className="h-12 w-12 flex items-center justify-center rounded-full ml-2 bg-black bg-opacity-10 dark:bg-white dark:bg-opacity-10">
            <RiBarChartLine className="h-6 w-6" />
          </div>
        </NavLink>
        <NavLink to={routes.MODEL_TYPES}>
          <div className="h-12 w-12 flex items-center justify-center rounded-full ml-2 bg-black bg-opacity-10 dark:bg-white dark:bg-opacity-10">
            <RiEditBoxLine className="h-6 w-6" />
          </div>
        </NavLink>
        <NavLink to={routes.MODEL_TABLE_LIST}>
          <div className="h-12 w-12 flex items-center justify-center rounded-full ml-2 bg-black bg-opacity-10 dark:bg-white dark:bg-opacity-10">
            <RiTableLine className="h-6 w-6" />
          </div>
        </NavLink>
        <NavLink to={routes.SETTINGS}>
          <div className="h-12 w-12 flex items-center justify-center rounded-full ml-2 bg-black bg-opacity-10 dark:bg-white dark:bg-opacity-10">
            <RiSettings3Line className="h-6 w-6" />
          </div>
        </NavLink>
      </div>
    );
  };

  const renderMobileHeader = () => {
    return (
      <div
        className="flex md:hidden fixed top-0 left-0 right-0 z-50 h-16 bg-white dark:bg-gray-700 text-gray-700 dark:text-white duration-200"
        style={{ zIndex: 1000 }}
      >
        <div className="w-24 h-16 flex items-center justify-start pl-4"></div>
        <div className="flex-1"></div>
        <div className="w-32 h-16 flex items-center justify-end pr-4">
          {renderNavButtons()}
        </div>
      </div>
    );
  };

  const renderDesktopHeader = () => {
    return (
      <div
        className="w-full hidden md:flex fixed top-0 left-0 right-0 z-50 h-16 duration-200 bg-white dark:bg-gray-700 text-gray-700 dark:text-white"
        style={{ zIndex: 1000 }}
      >
        <div className="h-16 flex items-center ml-4 flex-1">
          <Link
            to={routes.HOME}
            className="text-2xl flex rounded-lg items-center"
          >
            <img
              src={
                config.uiMode === "dark" ? "logo-white.png" : "logo-black.png"
              }
              className="h-12"
            />
          </Link>
          <div className="ml-4 py-1 px-2 text-xs font-semibold rounded-md bg-green-300 text-green-800 dark:bg-green-800 dark:text-gray-300">
            Beta
          </div>
        </div>
        <div className="flex-1 flex justify-center items-center">
          <div className="font-bold text-2xl">✨ Nebula ✨</div>
        </div>
        <div className="h-16 flex items-center justify-end mr-2 flex-1">
          {renderNavButtons()}
        </div>
      </div>
    );
  };

  return (
    <div className="bg-gray-200 dark:bg-gray-900 text-gray-700 dark:text-white min-h-screen flex pt-16">
      {renderMobileHeader()}
      {renderDesktopHeader()}
      <div className="flex-1">{props.children}</div>
    </div>
  );
};
