// @generated

/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */

export type paths = Record<string, never>;
export type webhooks = Record<string, never>;
export interface components {
    schemas: {
        Battle: {
            background_image?: string | null;
            board: components["schemas"]["Board"];
            cards: {
                [key: string]: components["schemas"]["Card"];
            };
            characters: {
                [key: string]: components["schemas"]["Character"];
            };
            /** Format: uint64 */
            default_turn_actions: number;
            effects: {
                [key: string]: components["schemas"]["Effect"];
            };
            history: components["schemas"]["TemplateEntry_for_BattleTextEntry"][][];
            introduction?: components["schemas"]["StoryCardEntry"][] | null;
            objects: {
                [key: string]: components["schemas"]["Object"];
            };
            /** Format: uint16 */
            round: number;
            teams: components["schemas"]["Team"][];
        };
        BattleState: {
            battle: components["schemas"]["Battle"];
            character_id: components["schemas"]["CharacterId"];
        };
        /** @enum {string} */
        BattleTextEntry: BattleTextEntry;
        Board: {
            grid: components["schemas"]["Grid_for_BoardItem"];
        };
        BoardItem: {
            id: components["schemas"]["CharacterId"];
            /** @enum {string} */
            type: BoardItemType;
        } | {
            id: components["schemas"]["CardId"];
            /** @enum {string} */
            type: BoardItemType;
        } | {
            /** @enum {string} */
            type: BoardItemType;
        };
        Card: {
            actions: components["schemas"]["CardAction"][];
            description: string;
            flavor?: string | null;
            id: components["schemas"]["CardId"];
            name: string;
            /** Format: uint64 */
            range: number;
        };
        CardAction: {
            Damage: {
                amount: components["schemas"]["U64Range"];
                area: components["schemas"]["U64Range"];
                target: components["schemas"]["Target"];
            };
        } | {
            Heal: {
                amount: components["schemas"]["U64Range"];
                area: components["schemas"]["U64Range"];
                target: components["schemas"]["Target"];
            };
        } | {
            GainAction: {
                amount: components["schemas"]["U64Range"];
                target: components["schemas"]["Target"];
            };
        } | {
            Move: {
                amount: components["schemas"]["U64Range"];
                target: components["schemas"]["Target"];
            };
        } | {
            Effect: {
                chance: components["schemas"]["Chance"];
                effect: components["schemas"]["EffectId"];
                target: components["schemas"]["Target"];
            };
        } | {
            RemoveEffect: {
                chance: components["schemas"]["Chance"];
                effect: components["schemas"]["EffectId"];
                target: components["schemas"]["Target"];
            };
        } | {
            ReduceEffect: {
                /** Format: uint64 */
                amount: number;
                chance: components["schemas"]["Chance"];
                effect: components["schemas"]["EffectId"];
                target: components["schemas"]["Target"];
            };
        };
        /** Format: uint */
        CardId: number;
        /** Format: uint32 */
        Chance: number;
        Character: {
            contains: components["schemas"]["Content"][];
            deck: components["schemas"]["CardId"][];
            /** Format: uint64 */
            default_movement: number;
            discard: components["schemas"]["CardId"][];
            effects: components["schemas"]["EffectId"][];
            hand: components["schemas"]["CardId"][];
            /** Format: uint */
            hand_size: number;
            health: components["schemas"]["Health"];
            id: components["schemas"]["CharacterId"];
            image?: string | null;
            max_health: components["schemas"]["Health"];
            /** Format: uint64 */
            movement: number;
            name: string;
            race: components["schemas"]["CharacterRace"];
            /** Format: uint64 */
            remaining_actions: number;
        };
        /** Format: uint */
        CharacterId: number;
        /** @enum {string} */
        CharacterRace: CharacterRace;
        Content: {
            Card: components["schemas"]["CardId"];
        } | {
            Object: components["schemas"]["ObjectId"];
        };
        Effect: {
            actions: components["schemas"]["CardAction"][];
            description: string;
            id: components["schemas"]["EffectId"];
            image?: string | null;
            name: string;
            triggers: components["schemas"]["Trigger"][];
        };
        /** Format: uint */
        EffectId: number;
        Grid_for_BoardItem: {
            /** Format: uint */
            height: number;
            members: (components["schemas"]["BoardItem"] | null)[][];
            /** Format: uint */
            width: number;
        };
        /** Format: uint64 */
        Health: number;
        Object: {
            description: string;
            id: components["schemas"]["ObjectId"];
            image?: string | null;
            name: string;
        };
        /** Format: uint */
        ObjectId: number;
        StoryCardEntry: {
            h1: string;
        } | {
            p: string;
        };
        /** @enum {string} */
        Target: Target;
        Team: {
            id: components["schemas"]["TeamId"];
            name: string;
        };
        /** Format: uint64 */
        TeamId: number;
        TemplateEntry_for_BattleTextEntry: {
            Text: string;
        } | {
            Typed: [
                components["schemas"]["BattleTextEntry"],
                string
            ];
        };
        /** @enum {string} */
        Trigger: Trigger;
        U64Range: [
            number,
            number
        ];
    };
    responses: never;
    parameters: never;
    requestBodies: never;
    headers: never;
    pathItems: never;
}
export type Battle = components['schemas']['Battle'];
export type BattleState = components['schemas']['BattleState'];
export type Board = components['schemas']['Board'];
export type BoardItem = components['schemas']['BoardItem'];
export type Card = components['schemas']['Card'];
export type CardAction = components['schemas']['CardAction'];
export type CardId = components['schemas']['CardId'];
export type Chance = components['schemas']['Chance'];
export type Character = components['schemas']['Character'];
export type CharacterId = components['schemas']['CharacterId'];
export type Content = components['schemas']['Content'];
export type Effect = components['schemas']['Effect'];
export type EffectId = components['schemas']['EffectId'];
export type GridForBoardItem = components['schemas']['Grid_for_BoardItem'];
export type Health = components['schemas']['Health'];
export type Object = components['schemas']['Object'];
export type ObjectId = components['schemas']['ObjectId'];
export type StoryCardEntry = components['schemas']['StoryCardEntry'];
export type Team = components['schemas']['Team'];
export type TeamId = components['schemas']['TeamId'];
export type TemplateEntryForBattleTextEntry = components['schemas']['TemplateEntry_for_BattleTextEntry'];
export type U64Range = components['schemas']['U64Range'];
export type $defs = Record<string, never>;
export enum BattleTextEntry {
    Id = "Id",
    Attack = "Attack",
    Damage = "Damage"
}
export enum BoardItemType {
    Character = "Character"
}
export enum BoardItemType {
    Card = "Card"
}
export enum BoardItemType {
    Inert = "Inert"
}
export enum CharacterRace {
    Human = "Human",
    Machine = "Machine"
}
export enum Target {
    Me = "Me",
    Others = "Others",
    Any = "Any"
}
export enum Trigger {
    Death = "Death",
    TurnStart = "TurnStart"
}
export type operations = Record<string, never>;
