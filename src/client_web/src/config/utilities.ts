export const generateImgUrl = (img: {
  id: string;
  canisterId: string;
}): string => {
  const isDevelopment = process.env.NODE_ENV !== "production";
  return isDevelopment
    ? `http://localhost:8000/images/${img.id}?canisterId=${img.canisterId}`
    : `https://${img.canisterId}.raw.ic0.app/images/${img.id}`;
};