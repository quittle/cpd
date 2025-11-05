import React, { useLayoutEffect, useId } from "react";

export interface TooltipProps {
  title: string;
  body: string;
  anchor: HTMLElement;
}

export default function Tooltip(props: TooltipProps) {
  const { title, body, anchor } = props;

  const anchorName = `--${useId()}`;

  useLayoutEffect(() => {
    anchor.style["anchorName"] = anchorName;
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
