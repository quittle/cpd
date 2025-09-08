export * from "./battle-schema.generated";
import {
  BoardItem,
  CardId,
  CharacterId,
  StoryCardEntry,
  TemplateEntryForBattleTextEntry,
  Target,
  BoardItemType,
} from "./battle-schema.generated";

export { Target as ActionTarget };

export type TypedText = TemplateEntryForBattleTextEntry;
export type BattleHistoryEntry = TypedText[];
export type StoryCard = StoryCardEntry[];

export type BoardItemCharacter = BoardItem & {
  id: CharacterId;
  type: BoardItemType.Character;
};

export type BoardItemCard = BoardItem & {
  id: CardId;
  type: BoardItemType.Card;
};

export type BoardItemInert = BoardItem & {
  type: BoardItemType.Inert;
};

export function isBoardItemCharacter(
  item: BoardItem | null | undefined,
): item is BoardItemCharacter {
  return item instanceof Object && item.type === BoardItemType.Character;
}

export function isBoardItemCard(
  item: BoardItem | null | undefined,
): item is BoardItemCard {
  return item instanceof Object && item.type === BoardItemType.Card;
}

export function isBoardItemInert(
  item: BoardItem | null | undefined,
): item is BoardItemInert {
  return item instanceof Object && item.type === BoardItemType.Inert;
}
