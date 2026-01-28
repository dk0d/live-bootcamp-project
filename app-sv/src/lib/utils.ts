
export function getAuthUrl() {
  return typeof process === "undefined"
    ? "http://localhost:5170"
    : (process.env.AUTH_URL ?? "http://localhost:5170");
}
