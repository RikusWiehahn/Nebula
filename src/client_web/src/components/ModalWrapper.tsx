import React from "react";
import { CustomModalHeader } from "../components/CustomModalHeader";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { StoreState } from "../config/ReduxStore";
import { useSelector } from "react-redux";
import Dialog from "@mui/material/Dialog";

export const ModalWrapper = (props: {
  children?: JSX.Element | JSX.Element[];
  buttonComponent?: JSX.Element;
  headerLabel?: string;
  isLoading?: boolean;
  showModal: boolean;
  onClose?: () => void;
}) => {
  const config = useSelector((state: StoreState) => state.config);

  const renderForm = (): JSX.Element | null => {
    return (
      <div>
        <CustomModalHeader
          label={props.headerLabel}
          {...(props.onClose ? { onClose: props.onClose } : {})}
        />
        <div>{props.children}</div>
      </div>
    );
  };

  const renderModal = () => {
    return (
      <Dialog
        open={props.showModal}
        onClose={(e, r) => {
          r !== "backdropClick" && props.onClose?.();
        }}
        PaperProps={{
          elevation: 0,
          style: {
            borderRadius: "0rem",
          },
        }}
        fullWidth
        maxWidth="xs"
      >
        <div className={`${config.uiMode === "light" ? "light" : "dark"}`}>
          <div className="w-full p-4 bg-white dark:bg-gray-800 text-gray-700 dark:text-white">
            {props.isLoading ? (
              <div>
                <LoadingIndicator />
              </div>
            ) : (
              renderForm()
            )}
          </div>
        </div>
      </Dialog>
    );
  };

  return (
    <div>
      {props.buttonComponent}
      {renderModal()}
    </div>
  );
};
