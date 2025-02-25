import React, { useEffect, useState } from "react";
import { ActionTarget, BattleState, CardId } from "./battle";
import * as messages from "./messages.js";
import Card from "./Card.js";
import Character from "./Character.js";
import BattleHistory from "./BattleHistory.js";
import { getCardTarget, getLivingEnemies } from "./utils.js";
import { takeAction } from "./state.js";
import { StoryCard } from "./StoryCard.js";
import { GameBoard } from "./GameBoard.js";

messages.init();

export default function App() {
  const [battleState, setBattleState] = useState<BattleState>();
  const [dragState, setDragState] = useState<CardId>();
  const [showIntroState, setShowIntroState] = useState<boolean>(false);

  useEffect(() => {
    // Throwaway
    fetch("/info");

    const onBattleState = (e) => {
      setBattleState(JSON.parse(e.data));
    };

    messages.addEventListener("battle_state", onBattleState);
    return () => {
      messages.removeEventListener("battle_state", onBattleState);
    };
  }, [setBattleState]);

  useEffect(() => {
    const round = battleState?.battle.round;
    if (round === undefined) {
      setShowIntroState(false);
    } else {
      setShowIntroState(round <= 1);
    }
  }, [battleState?.battle.round]);

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
          onClose={() => setShowIntroState(false)}
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
            battle={battle}
          />
          {Object.values(battle.characters)
            .filter((character) => character.id !== characterId)
            .map((character) => (
              <Character
                isPlayer={false}
                key={character.id}
                characterId={character.id}
                draggedCard={dragState}
                battle={battle}
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
            {battle.characters[characterId].hand.map((cardId) => {
              const card = battle.cards[cardId];
              const target = getCardTarget(card);
              let defaultAction: undefined | (() => Promise<void>);
              if (target === ActionTarget.Me) {
                defaultAction = async () =>
                  await takeAction(card.id, characterId);
              } else if (target === ActionTarget.Others) {
                const enemies = getLivingEnemies(battle, characterId);
                if (enemies.length == 1) {
                  defaultAction = async () =>
                    await takeAction(card.id, enemies[0].id);
                }
              }
              return (
                <li key={cardId}>
                  <Card
                    card={card}
                    onDragStart={() => setDragState(cardId)}
                    onDragEnd={() => setDragState(undefined)}
                    onClick={async () => {
                      // Take default actions when clicking buttons
                      if (defaultAction) {
                        await defaultAction();
                      }
                    }}
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
