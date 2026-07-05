import type { BattleState, CardInstance, Character } from "./battle";
import React, { useEffect, useState } from "react";
import { assetUrl, getPlayerCoordinate, isAdjacent } from "./utils";
import {
  isBoardItemCard,
  isBoardItemCharacter,
  isBoardItemInert,
} from "./battle";
import { move, takeAction } from "./state";

import type { Coordinate } from "./utils";
import { isCardEligible } from "./Card";

export function GameBoard(props: {
  readonly battleState: BattleState;
  readonly draggedCard: CardInstance | undefined;
}) {
  const { battle } = props.battleState;
  const playerCoordinate = getPlayerCoordinate(props.battleState);
  const [selectedSquare, setSelectedSquare] = useState<Coordinate | null>(playerCoordinate);
  const [isPendingAction, setIsPendingAction] = useState<boolean>(false);

  useEffect(() => {
    if (isPendingAction) {
      // eslint-disable-next-line react-hooks/set-state-in-effect: This shouldn't trigger recursive changes
      setSelectedSquare(playerCoordinate);
      setIsPendingAction(false);
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps: Intentionally non-exhaustive to only trigger when battle changes
  }, [battle])

  const backgroundImage = battle.background_image
    ? assetUrl(battle.background_image)
    : undefined;
  return (
    <table className="game-board" style={{ backgroundImage }}>
      <tbody>
        {battle.board.grid.members.map((row, y) => (
          <tr
            key={y} // eslint-disable-line react/no-array-index-key
          >
            {row.map((cell, x) => {
              let image: string | undefined;
              let character: Character | undefined;
              let isPlayer = false;
              let isInert = false;
              let isClickable: boolean;
              if (isBoardItemCharacter(cell)) {
                character = battle.characters[cell.id];
                if (character.image) {
                  image = assetUrl(character.image);
                }
                if (character.health === 0) {
                  image = assetUrl("skull.png");
                }
                isPlayer = props.battleState.character_id === cell.id;
                isClickable = !isPlayer;
              } else if (isBoardItemCard(cell)) {
                image = assetUrl("card.png");
                isClickable = true;
              } else if (isBoardItemInert(cell)) {
                isInert = true;
                isClickable = false;
              }
              const curLocation: Coordinate = { x, y };
              const isSelectedSquare =
                selectedSquare?.x === x && selectedSquare.y === y;

              // Only ineligible if there is actively a card being dragged and that card isn't eligible.
              const isIneligible =
                props.draggedCard !== undefined &&
                ((character?.health ?? 0) === 0 ||
                  !isCardEligible(isPlayer, props.draggedCard, battle));

              return (
                <td
                  key={x} // eslint-disable-line react/no-array-index-key
                  onClick={async () => {
                    if (isSelectedSquare) {
                      console.log("Clearning");
                      setSelectedSquare(undefined);
                      setIsPendingAction(false);
                    } else if (isPlayer) {
                      setSelectedSquare(curLocation);
                    } else if (
                      isAdjacent(selectedSquare, curLocation)
                    ) {
                      const item =
                        battle.board.grid.members[selectedSquare.y][
                        selectedSquare.x
                        ];
                      if (isBoardItemCharacter(item)) {
                        setIsPendingAction(true);
                        await move(item.id, curLocation);
                      }
                    }
                  }}
                  onDragOver={(e) => {
                    if (props.draggedCard === undefined) {
                      return;
                    }

                    e.preventDefault();
                    e.dataTransfer.dropEffect = isIneligible ? "none" : "move";
                  }}
                  onDrop={async (_e) => {
                    if (
                      props.draggedCard === undefined ||
                      character === undefined
                    ) {
                      return;
                    }

                    await takeAction(props.draggedCard, character.id);
                  }}
                  style={{
                    border: isInert ? 0 : undefined,
                    borderColor: isSelectedSquare ? "red" : "black",
                    borderStyle: isSelectedSquare && isPendingAction ? "dashed" : "solid",
                    backgroundImage: image,
                    opacity: isIneligible ? 0.5 : 1,
                    cursor: isClickable ? "pointer" : "default",
                  }}
                  title={character?.name}
                />
              );
            })}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
