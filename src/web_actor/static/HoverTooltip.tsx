import React, { useEffect, useState } from "react";
import Tooltip, { TooltipProps } from "./Tooltip";

export default function HoverTooltip(props: TooltipProps) {
  const { anchor } = props;

  const [shown, setShown] = useState(false);

  useEffect(() => {
    anchor.addEventListener("mouseover", () => setShown(true));
    anchor.addEventListener("focus", () => setShown(true));
    anchor.addEventListener("mouseout", () => setShown(false));
    anchor.addEventListener("blur", () => setShown(false));
  }, [anchor]);

  return shown ? <Tooltip {...props}></Tooltip> : null;
}
