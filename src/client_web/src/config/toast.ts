import { toast } from "react-toastify";

export const ErrorToast = (message: string) => {
  toast.error(message, {
    closeOnClick: true,
    pauseOnHover: true,
    draggable: true,
    icon: "🚧",
  });
};

export const SuccessToast = (message: string) => {
  toast.success(message, {
    closeOnClick: true,
    pauseOnHover: true,
    draggable: true,
    icon: "🚀",
  });
};

export const NotificationToast = (message: string) => {
  toast.success(message, {
    closeOnClick: true,
    pauseOnHover: true,
    draggable: true,
    icon: "🔔",
  });
};
