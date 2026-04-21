import { CardId, CharacterId, ObjectId, CardInstance } from "./battle";
import { Coordinate } from "./utils";

export async function takeAction(card: CardInstance, targetId: CharacterId) {
  await fetch("/act", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      card_id: card.card_id,
      card_instance_id: card.card_instance_id,
      target_id: targetId,
    }),
  });
}

export async function takeContent(
  targetId: CharacterId,
  from: Coordinate,
  item: { card: CardId } | { object: ObjectId },
) {
  await fetch("/take", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      target_id: targetId,
      from,
      item,
    }),
  });
}

export async function move(targetId: CharacterId, to: Coordinate) {
  await fetch("/move", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      target_id: targetId,
      to: to,
    }),
  });
}

export async function pass() {
  await fetch("/pass", {
    method: "POST",
  });
}
