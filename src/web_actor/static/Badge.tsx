import React from "react";
import type { ReactNode } from "react";

export default function Badge(props: {
  readonly count: number;
  readonly showCountBelowTwo: boolean;
  readonly children: ReactNode;
}) {
  if (!props.showCountBelowTwo && props.count < 2) {
    return props.children;
  }

  return (
    <span className="badge" data-count={props.count}>
      {props.children}
    </span>
  );
}
