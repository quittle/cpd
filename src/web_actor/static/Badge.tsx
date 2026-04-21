import React from "react";
import type { ReactNode } from "react";
export default async function Badge(props: {
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
