import React from "react";
import {
  BattleState,
  CardId,
  Character,
  CharacterId,
  Effect as BattleEffect,
} from "./battle";
import { assetPath, countEntries, cssUrl } from "./utils";
import { pass, takeAction } from "./state";
import { isCardEligible } from "./Card";
import { footsteps, bolt } from "./images";
import Container from "./Container";
import HealthBar from "./HealthBar";
import Effect from "./Effect";

export default function Character(props: {
  isPlayer: boolean;
  characterId: CharacterId;
  draggedCard: CardId | undefined;
  battleState: BattleState;
}) {
  const { isPlayer, characterId, draggedCard, battleState } = props;
  const { battle } = battleState;

  const character = battle.characters[characterId];

  const [contentsOpened, setContentsOpened] = React.useState(false);

  const effects: [BattleEffect, number][] = Array.from(
    countEntries(character.effects),
  ).map(([id, count]) => [battle.effects[id], count]);

  // Only ineligible if there is actively a card being dragged and that card isn't eligible.
  const isIneligible =
    draggedCard !== undefined &&
    (character.health == 0 || !isCardEligible(isPlayer, draggedCard, battle));
  return (
    <div
      className="character"
      style={{
        opacity: isIneligible ? 0.5 : 1,
      }}
      onDragOver={(e) => {
        if (draggedCard === undefined) {
          return;
        }

        e.preventDefault();
        e.dataTransfer.dropEffect = isIneligible ? "none" : "move";
      }}
      onDrop={async (_e) => {
        if (draggedCard === undefined) {
          return;
        }

        await takeAction(draggedCard, characterId);
      }}
    >
      {character.image ? (
        <img src={assetPath(character.image)} style={{ width: "100%" }} />
      ) : null}

      {character.contains.length > 0 ? (
        <button
          className="open"
          onClick={() => {
            setContentsOpened(true);
          }}
        >
          Open
        </button>
      ) : null}

      <HealthBar
        value={character.health}
        max={character.max_health}
        foregroundColor="var(--c-health)"
        backgroundColor="var(--c-grey)"
        valueTextColor="white"
        maxTextColor="black"
      />
      <div className="bottom">
        <div className="line-1">
          <h3>{character.name}</h3>
          <span
            title="Movement"
            className="movement"
            style={{
              backgroundImage: cssUrl(footsteps),
            }}
          >
            {character.movement}
          </span>
          <span
            title="Actions"
            className="actions"
            style={{
              backgroundImage: cssUrl(bolt),
            }}
          >
            {character.remaining_actions}
          </span>
        </div>

        <div className="effects">
          {effects.map(([effect, count]) => (
            <Effect key={effect.id} effect={effect} count={count} />
          ))}
        </div>
      </div>
      {isPlayer ? (
        <button
          className="character-end-turn"
          onClick={async () => {
            await pass();
          }}
        >
          End Turn <span className="character-end-turn-icon">👍</span>
        </button>
      ) : null}
      {contentsOpened ? (
        <Container
          characterId={characterId}
          battleState={battleState}
          contents={character.contains}
          onClose={() => setContentsOpened(false)}
        />
      ) : null}
    </div>
  );
}
