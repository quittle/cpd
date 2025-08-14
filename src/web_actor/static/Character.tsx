import React from "react";
import { BattleState, CardId, Character, CharacterId, Effect } from "./battle";
import { assetUrl, assetPath, countEntries } from "./utils";
import { pass, takeAction } from "./state";
import { isCardEligible } from "./Card";
import MeterBar from "./MeterBar";
import Badge from "./Badge";
import Container from "./Container";

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

  const effects: [Effect, number][] = Array.from(
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
      <MeterBar
        value={character.health}
        max={character.max_health}
        foregroundColor="red"
        backgroundColor="black"
        textColor="white"
      />
      <div className="effects">
        {effects.map(([effect, count]) => (
          <Badge key={effect.id} count={count} showCountBelowTwo={false}>
            <span
              className="effect"
              title={effect.name}
              style={{
                backgroundImage: assetUrl(effect.image),
              }}
            />
          </Badge>
        ))}
      </div>
      {character.image ? (
        <img
          src={assetPath(character.image)}
          style={{ width: isPlayer ? "100px" : "100%" }}
        />
      ) : null}
      {isPlayer ? (
        <button
          className="character-end-turn"
          onClick={async () => {
            await pass();
          }}
        >
          End Turn
        </button>
      ) : null}
      <h3>{character.name}</h3>
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
      {isPlayer
        ? `Remaining actions: ${"🔵".repeat(character.remaining_actions)}`
        : null}
      <div>
        Movement: <b>{character.movement}</b>
      </div>
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
