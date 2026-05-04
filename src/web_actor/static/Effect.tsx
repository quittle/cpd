import React, { useState } from "react";

import Badge from "./Badge";
import type { Effect } from "./battle";
import HoverTooltip from "./HoverTooltip";
import { assetUrl } from "./utils";

export default function Effect(props: {
  readonly effect: Effect;
  readonly count: number;
}) {
  const { effect, count } = props;

  const [effectElement, setEffectElement] = useState<HTMLElement>();

  return (
    <Badge count={count} key={effect.id} showCountBelowTwo={false}>
      <span
        className="effect"
        ref={setEffectElement}
        style={{
          backgroundImage: assetUrl(effect.image),
        }}
        tabIndex={0}
      >
        {effectElement ? (
          <HoverTooltip
            anchor={effectElement}
            body={effect.description}
            title={effect.name}
          />
        ) : null}
      </span>
    </Badge>
  );
}
