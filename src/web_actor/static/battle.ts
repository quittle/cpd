export enum CharacterRace {
  Human = "Human",
}

export type CardId = number;
export type CharacterId = number;

export interface Character {
  id: CharacterId;
  name: string;
  race: CharacterRace;
  hand: CardId[];
  deck: CardId[];
  health: number;
  max_health: number;
  hand_size: number;
  remaining_actions: number;
  image: string | null;
  movement: number;
}

export interface Team {
  id: number;
  name: string;
}

export type BattleType = "Id" | "Attack" | "Damage";

export type TypedText = { Text: string } | { Typed: [BattleType, string] };

export type BattleHistoryEntry = TypedText[];

export enum ActionTarget {
  Me = "Me",
  Others = "Others",
  Any = "Any",
}

export type CardAction =
  | { Damage: { target: ActionTarget; amount: number } }
  | { Heal: { target: ActionTarget; amount: number } }
  | { Effect: { target: ActionTarget; amount: number; change: number } }
  | { RemoveEffect: { target: ActionTarget; amount: number; change: number } };

export interface Card {
  id: CardId;
  name: string;
  description: string;
  flavor?: string;
  range: number;
  actions: CardAction[];
}

export type StoryCardEntry = { h1: string } | { p: string };

export type StoryCard = StoryCardEntry[];

export type BoardItemCharacter = {
  Character: CharacterId;
};

export type BoardItemCard = {
  Card: CardId;
};

export type BoardItemInert = "Inert";

export function isBoardItemCharacter(
  item: BoardItem | null | undefined,
): item is BoardItemCharacter {
  return item instanceof Object && "Character" in item;
}

export function isBoardItemCard(
  item: BoardItem | null | undefined,
): item is BoardItemCard {
  return item instanceof Object && "Card" in item;
}

export function isBoardItemInert(
  item: BoardItem | null | undefined,
): item is BoardItemInert {
  return item === "Inert";
}

export type BoardItem = BoardItemCharacter | BoardItemCard | BoardItemInert;

export interface Board {
  grid: {
    members: Array<Array<BoardItem | null>>;
    width: number;
    height: number;
  };
}

export interface Battle {
  characters: Record<string, Character>;
  teams: Team[];
  introduction?: StoryCard;
  history: BattleHistoryEntry[];
  round: number;
  cards: Record<string, Card>;
  board: Board;
  background_image: string | null;
}

export interface BattleState {
  character_id: number;
  battle: Battle;
}
