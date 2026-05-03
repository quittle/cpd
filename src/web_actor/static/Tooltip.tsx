import React, { useId, useLayoutEffect, useRef } from "react";

export interface TooltipProps {
  title: string;
  body: string;
  anchor: HTMLElement;
}

export default function Tooltip(props: TooltipProps) {
  const { title, body, anchor } = props;

  const anchorRef = useRef(anchor);

  const anchorName = `--${useId()}`;

  useLayoutEffect(() => {
    anchorRef.current.style["anchorName"] = anchorName; // eslint-disable-line dot-notation
  }, [anchor, anchorName]);

  return (
    <div
      className="tooltip"
      style={{
        positionAnchor: anchorName,
      }}
      role="tooltip"
    >
      <h3>{title}</h3>
      <p>{body}</p>
    </div>
  );
}
