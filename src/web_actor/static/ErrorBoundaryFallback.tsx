import React from "react";
import { FallbackProps } from "react-error-boundary";

export default function ErrorBoundaryFallback({
  error,
  resetErrorBoundary,
}: FallbackProps) {
  const isError = error instanceof Error;

  return (
    <div role="alert">
      <h1>Something went wrong :(</h1>
      {isError ? (
        <pre style={{ color: "red" }}>
          {error.stack ?? error.name + " " + error.message}
        </pre>
      ) : (
        <pre style={{ color: "red" }}>
          Error: [{typeof error}] {String(error)}
        </pre>
      )}
      <button onClick={resetErrorBoundary}>Try again</button>
    </div>
  );
}
