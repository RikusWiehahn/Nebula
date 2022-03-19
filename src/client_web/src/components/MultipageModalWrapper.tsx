import React, { useState } from "react";
import { CustomModalHeader } from "../components/CustomModalHeader";
import { LoadingIndicator } from "../components/LoadingIndicator";
import { StoreState } from "../config/ReduxStore";
import { useSelector } from "react-redux";
import Dialog from "@mui/material/Dialog";

export interface ModalWrapperSlideProps {
  back?: () => void;
  next?: () => void;
}

export const MultipageModalWrapper = (props: {
  children?: JSX.Element | JSX.Element[];
  buttonComponent?: JSX.Element;
  isLoading?: boolean;
  showModal: boolean;
  slides: (({ back, next }: ModalWrapperSlideProps) => JSX.Element)[];
  onClose?: () => void;
}) => {
  const config = useSelector((state: StoreState) => state.config);
  const [stage, setStage] = useState<number>(0);

  const _next = () => {
    if (stage + 1 < props.slides.length) {
      setStage(stage + 1);
    }
  };
  const _back = () => {
    if (stage - 1 > -1) {
      setStage(stage - 1);
    }
  };

  const renderForm = (): JSX.Element | null => {
    const slide = props.slides[stage];
    if (!props.slides.length) {
      return <div>No slides</div>;
    }
    return <div key={stage}>{slide({ back: _back, next: _next })}</div>;
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
