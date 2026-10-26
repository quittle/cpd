import React, { useEffect, useState } from "react";

import Tooltip from "./Tooltip";
import type { TooltipProps } from "./Tooltip";

export default function HoverTooltip(props: TooltipProps) {
  const { anchor } = props;

  const [shown, setShown] = useState(false);

  useEffect(() => {
    const show = () => {
      setShown(true);
    };
    const hide = () => {
      setShown(false);
    };
    anchor.addEventListener("mouseover", show);
    anchor.addEventListener("focus", show);
    anchor.addEventListener("mouseout", hide);
    anchor.addEventListener("blur", hide);
    return () => {
      anchor.removeEventListener("mouseover", show);
      anchor.removeEventListener("focus", show);
      anchor.removeEventListener("mouseout", hide);
      anchor.removeEventListener("blur", hide);
    };
  }, [anchor]);

  return shown ? <Tooltip {...props} /> : null;
}
