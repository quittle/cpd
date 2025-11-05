import React, { useState } from "react";
import Badge from "./Badge";
import { Effect } from "./battle";
import { assetUrl } from "./utils";
import HoverTooltip from "./HoverTooltip";

export default function Effect(props: { effect: Effect; count: number }) {
  const { effect, count } = props;

  const [effectElement, setEffectElement] = useState<HTMLElement>();

  return (
    <Badge key={effect.id} count={count} showCountBelowTwo={false}>
      <span
        ref={setEffectElement}
        className="effect"
        tabIndex={0}
        style={{
          backgroundImage: assetUrl(effect.image),
        }}
      >
        {effectElement ? (
          <HoverTooltip
            title={effect.name}
            body={effect.description}
            anchor={effectElement}
          />
        ) : null}
      </span>
    </Badge>
  );
}
