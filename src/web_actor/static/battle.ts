export * from "./battle-schema.generated";
import {
  BoardItem,
  CardId,
  CharacterId,
  StoryCardEntry,
  TemplateEntry,
  Target,
} from "./battle-schema.generated";

export { Target as ActionTarget };

export type TypedText = TemplateEntry;
export type BattleHistoryEntry = TypedText[];
export type StoryCard = StoryCardEntry[];

export type BoardItemCharacter = BoardItem & {
  id: CharacterId;
  type: "Character";
};

export type BoardItemCard = BoardItem & {
  id: CardId;
  type: "Card";
};

export type BoardItemInert = BoardItem & {
  type: "Inert";
};

export function isBoardItemCharacter(
  item: BoardItem | null | undefined,
): item is BoardItemCharacter {
  return item instanceof Object && item.type === "Character";
}

export function isBoardItemCard(
  item: BoardItem | null | undefined,
): item is BoardItemCard {
  return item instanceof Object && item.type === "Card";
}

export function isBoardItemInert(
  item: BoardItem | null | undefined,
): item is BoardItemInert {
  return item instanceof Object && item.type === "Inert";
}
