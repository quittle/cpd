import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ErrorBoundary } from "react-error-boundary";
import ErrorBoundaryFallback from "./ErrorBoundaryFallback";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ErrorBoundary FallbackComponent={ErrorBoundaryFallback}>
      <App />
    </ErrorBoundary>
  </React.StrictMode>,
);
