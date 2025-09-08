import React from "react";
import { ReactNode } from "react";
export default function Badge(props: {
  count: number;
  showCountBelowTwo: boolean;
  children: ReactNode;
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
