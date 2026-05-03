import { ActionTarget, isBoardItemCharacter } from "./battle";
import type {
  Battle,
  BattleState,
  Card,
  CardAction,
  Character,
  CharacterId,
  Content,
} from "./battle";

import type React from "react";

export function getActionTarget(action: CardAction): ActionTarget {
  if ("Damage" in action) {
    return action.Damage.target;
  }
  if ("Heal" in action) {
    return action.Heal.target;
  }
  if ("Effect" in action) {
    return action.Effect.target;
  }
  if ("RemoveEffect" in action) {
    return action.RemoveEffect.target;
  }
  if ("GainAction" in action) {
    return action.GainAction.target;
  }
  if ("Move" in action) {
    return action.Move.target;
  }

  throw new Error(`Unrecognized CardAction. Keys: ${Object.keys(action)}`);
}

export function getCardTarget(card: Card): ActionTarget {
  let defaultTarget: ActionTarget = ActionTarget.Me;
  for (const action of card.actions) {
    const target = getActionTarget(action);
    switch (target) {
      case ActionTarget.Any:
        // Any is any
        return ActionTarget.Any;
      case ActionTarget.Others:
        // If others found, then it should target others
        defaultTarget = ActionTarget.Others;
        break;
      case ActionTarget.Me:
        break;
    }
  }

  return defaultTarget;
}

export function getLivingCharacters(battle: Battle): Character[] {
  return Object.values(battle.characters).filter(
    (character) => character.health > 0,
  );
}

export function getLivingEnemies(
  battle: Battle,
  player: CharacterId,
): Character[] {
  return getLivingCharacters(battle).filter(
    (character) => character.id !== player,
  );
}

export function assetPath(rawAssetPath: string): string {
  return `ref/${rawAssetPath}`;
}

export function cssUrl(path: string): string {
  return `url(${path})`;
}

// Like `assetPath()` but returns a CSS url() string.
export function assetUrl(rawAssetPath: string): string {
  return cssUrl(assetPath(rawAssetPath));
}

export interface Coordinate {
  x: number;
  y: number;
}

export function isAdjacent(a?: Coordinate, b?: Coordinate): boolean {
  if (a === undefined || b === undefined) {
    return false;
  }
  return (
    (a.x === b.x && (a.y === b.y - 1 || a.y === b.y + 1)) ||
    (a.y === b.y && (a.x === b.x - 1 || a.x === b.x + 1))
  );
}

export function countEntries<T>(entries: readonly T[]): Map<T, number> {
  const ret = new Map<T, number>();
  for (const entry of entries) {
    ret.set(entry, (ret.get(entry) ?? 0) + 1);
  }
  return ret;
}

export function getCharacterCoordinate(
  battle: Battle,
  characterId: CharacterId,
): Coordinate {
  const cells = battle.board.grid.members;
  for (let y = 0; y < cells.length; y++) {
    for (let x = 0; x < cells[y].length; x++) {
      const cell = cells[y][x];
      if (isBoardItemCharacter(cell) && cell.id === characterId) {
        return { x, y };
      }
    }
  }

  return null;
}

export function getPlayerCoordinate(
  battleState: BattleState,
): Coordinate | null {
  return getCharacterCoordinate(battleState.battle, battleState.character_id);
}

export function requireCharacterCoordinate(
  battle: Battle,
  characterId: CharacterId,
): Coordinate {
  const coordinate = getCharacterCoordinate(battle, characterId);
  if (coordinate === null) {
    throw new Error(
      `Character "${battle.characters[characterId].name}" (${characterId}) not found on board`,
    );
  }
  return coordinate;
}

export function describeContent(
  content: Content,
  battle: Battle,
): { key: React.Key; assetUrl: string } {
  if ("Card" in content) {
    return {
      key: `C${content.Card}`,
      assetUrl: assetUrl("card.png"),
    };
  }

  if ("Object" in content) {
    return {
      key: `O${content.Object}`,
      assetUrl: assetUrl(battle.objects[content.Object.object_id].image),
    };
  }

  throw new Error(`Unrecognized content. Keys: ${Object.keys(content)}`);
}
