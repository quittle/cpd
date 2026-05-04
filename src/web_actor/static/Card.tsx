import type { Battle, Card, CardInstance } from "./battle";

import { ActionTarget } from "./battle";
import React from "react";
import { getCardTarget } from "./utils";

export function isCardEligible(
  isPlayer: boolean,
  cardInstance: CardInstance,
  battle: Battle,
): boolean {
  const card = battle.cards[cardInstance.card_id];
  const target = getCardTarget(card);
  switch (target) {
    case ActionTarget.Me:
      return isPlayer;
    case ActionTarget.Others:
      return !isPlayer;
    case ActionTarget.Any:
      return true;
  }
  throw new Error(`Unrecognized ActionTarget: ${target as ActionTarget}`);
}

export default function Card(props: {
  readonly card: Card;
  readonly cardInstance: CardInstance;
  readonly enabled: boolean;
  readonly onDragStart: () => void;
  readonly onDragEnd: () => void;
  readonly onClick: () => void;
  readonly hasDefaultAction: boolean;
}) {
  return (
    <button
      className="card"
      disabled={!props.enabled}
      draggable={props.enabled}
      onClick={props.onClick}
      onDragEnd={props.onDragEnd}
      onDragStart={(e) => {
        e.dataTransfer.setData(
          "text/plain",
          String(props.cardInstance.card_instance_id),
        );
        props.onDragStart();
      }}
      style={{
        cursor: props.enabled
          ? props.hasDefaultAction
            ? "pointer"
            : "grab"
          : "not-allowed",
      }}
      type="button"
    >
      <b className="card-header">{props.card.name}</b>

      <div className="card-body">
        <p>{props.card.description}</p>

        <p>
          <i>{props.card.flavor}</i>
        </p>

        {props.card.range > 0 ? (
          <p className="card-range">{props.card.range}</p>
        ) : null}
      </div>
    </button>
  );
}
