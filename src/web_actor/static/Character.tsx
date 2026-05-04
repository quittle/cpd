import type {
  Effect as BattleEffect,
  BattleState,
  CardInstance,
  CharacterId,
} from "./battle";
import { assetPath, countEntries, cssUrl } from "./utils";
import { bolt, footsteps } from "./images";
import { pass, takeAction } from "./state";
import { Character } from "./battle";
import Container from "./Container";
import Effect from "./Effect";
import HealthBar from "./HealthBar";
import React from "react";
import { isCardEligible } from "./Card";

export default function Character(props: {
  readonly isPlayer: boolean;
  readonly characterId: CharacterId;
  readonly draggedCard: CardInstance | undefined;
  readonly battleState: BattleState;
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
    (character.health === 0 || !isCardEligible(isPlayer, draggedCard, battle));
  return (
    <div
      className="character"
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
      style={{
        opacity: isIneligible ? 0.5 : 1,
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
        backgroundColor="var(--c-grey)"
        foregroundColor="var(--c-health)"
        max={character.max_health}
        maxTextColor="black"
        value={character.health}
        valueTextColor="white"
      />

      <div className="bottom">
        <div className="line-1">
          <h3>{character.name}</h3>

          <span
            className="movement"
            style={{
              backgroundImage: cssUrl(footsteps),
            }}
            title="Movement"
          >
            {character.movement}
          </span>

          <span
            className="actions"
            style={{
              backgroundImage: cssUrl(bolt),
            }}
            title="Actions"
          >
            {character.remaining_actions}
          </span>
        </div>

        <div className="effects">
          {effects.map(([effect, count]) => (
            <Effect count={count} effect={effect} key={effect.id} />
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
          battleState={battleState}
          characterId={characterId}
          contents={character.contains}
          onClose={() => {
            setContentsOpened(false);
          }}
        />
      ) : null}
    </div>
  );
}
