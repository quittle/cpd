import React, { StrictMode } from "react";

import App from "./App";
import { ErrorBoundary } from "react-error-boundary";
import ErrorBoundaryFallback from "./ErrorBoundaryFallback";
import ReactDOM from "react-dom/client";

const root = document.getElementById("root");
if (root === null) {
  throw new Error("Root element not found");
}

ReactDOM.createRoot(root).render(
  <StrictMode>
    <ErrorBoundary FallbackComponent={ErrorBoundaryFallback}>
      <App />
    </ErrorBoundary>
  </StrictMode>,
);
