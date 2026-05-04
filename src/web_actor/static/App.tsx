import * as messages from "./messages.js";

import type { BattleState, CardInstance } from "./battle";
import React, { useEffect, useState } from "react";
import { getCardTarget, getLivingEnemies } from "./utils.js";

import { ActionTarget } from "./battle";
import BattleHistory from "./BattleHistory.js";
import Card from "./Card.js";
import Character from "./Character.js";
import { GameBoard } from "./GameBoard.js";
import { StoryCard } from "./StoryCard.js";
import { takeAction } from "./state.js";

messages.init();

export default function App() {
  const [battleState, setBattleState] = useState<BattleState>();
  const [dragState, setDragState] = useState<CardInstance>();
  const [showIntroState, setShowIntroState] = useState<boolean>(false);

  useEffect(() => {
    // Throwaway
    void fetch("/info");

    const onBattleState: (MessageEvent) => void = (e: MessageEvent<string>) => {
      const newBattleState = JSON.parse(e.data) as BattleState;
      setBattleState(newBattleState);

      const { round } = newBattleState.battle;
      if (round === undefined) {
        setShowIntroState(false);
      } else {
        setShowIntroState(round <= 1);
      }
    };

    messages.addEventListener("battle_state", onBattleState);
    return () => {
      messages.removeEventListener("battle_state", onBattleState);
    };
  }, [setBattleState]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== "Alt") {
        return;
      }
      console.log(
        JSON.stringify(
          battleState?.battle.characters[battleState?.character_id],
          undefined,
          2,
        ),
        battleState,
      );
    };

    window.addEventListener("keydown", handleKeyDown);

    // Cleanup function
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [battleState]);

  if (!battleState) {
    return <div>Loading...</div>;
  }

  const { character_id: characterId, battle } = battleState;

  return (
    <div id="app">
      {battleState.battle.introduction ? (
        <StoryCard
          storyCard={battleState.battle.introduction}
          show={showIntroState}
          onClose={() => {
            setShowIntroState(false);
          }}
        />
      ) : (
        <></>
      )}
      <div style={{ flexGrow: 5 }}>
        <div id="characters">
          <Character
            isPlayer={true}
            characterId={characterId}
            draggedCard={dragState}
            battleState={battleState}
          />
          {Object.values(battle.characters)
            .filter((character) => character.id !== characterId)
            .map((character) => (
              <Character
                isPlayer={false}
                key={character.id}
                characterId={character.id}
                draggedCard={dragState}
                battleState={battleState}
              />
            ))}
        </div>
        <div
          style={{
            display: "flex",
          }}
        >
          <GameBoard battleState={battleState} draggedCard={dragState} />
          <ul id="cards">
            {battle.characters[characterId].hand.map((cardInstance) => {
              const card = battle.cards[cardInstance.card_id];
              const target = getCardTarget(card);
              let defaultAction: undefined | (() => Promise<void>);
              if (target === ActionTarget.Me) {
                defaultAction = async () =>
                  await takeAction(cardInstance, characterId);
              } else if (target === ActionTarget.Others) {
                const enemies = getLivingEnemies(battle, characterId);
                if (enemies.length === 1) {
                  defaultAction = async () =>
                    await takeAction(cardInstance, enemies[0].id);
                }
              }
              return (
                <li key={cardInstance.card_instance_id}>
                  <Card
                    card={card}
                    cardInstance={cardInstance}
                    onDragStart={() => {
                      setDragState(cardInstance);
                    }}
                    onDragEnd={() => {
                      setDragState(undefined);
                    }}
                    onClick={async () => {
                      // Take default actions when clicking buttons
                      if (defaultAction) {
                        await defaultAction();
                      }
                    }}
                    enabled={
                      battle.characters[characterId].remaining_actions > 0
                    }
                    hasDefaultAction={defaultAction !== undefined}
                  />
                </li>
              );
            })}
          </ul>
        </div>
      </div>
      <div style={{ flexGrow: 2 }}>
        <BattleHistory history={battle.history} />
      </div>
    </div>
  );
}
