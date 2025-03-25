import React from "react";
import { ActionTarget, Battle, Card, CardId } from "./battle";
import { getCardTarget } from "./utils";

export function isCardEligible(
  isPlayer: boolean,
  cardId: CardId,
  battle: Battle,
): boolean {
  const card = battle.cards[cardId];
  const target = getCardTarget(card);
  switch (target) {
    case ActionTarget.Me:
      return isPlayer;
    case ActionTarget.Others:
      return !isPlayer;
    case ActionTarget.Any:
      return true;
  }
}

export default function Card(props: {
  card: Card;
  enabled: boolean;
  onDragStart: () => void;
  onDragEnd: () => void;
  onClick: () => void;
  hasDefaultAction: boolean;
}) {
  return (
    <button
      className="card"
      draggable={props.enabled}
      onDragStart={(e) => {
        e.dataTransfer.setData("text/plain", String(props.card.id));
        props.onDragStart();
      }}
      onClick={props.onClick}
      onDragEnd={props.onDragEnd}
      disabled={!props.enabled}
      style={{
        cursor: props.enabled
          ? props.hasDefaultAction
            ? "pointer"
            : "grab"
          : "not-allowed",
      }}
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
