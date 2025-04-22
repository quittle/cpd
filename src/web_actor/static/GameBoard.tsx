import React, { useState } from "react";
import {
  BattleState,
  CardId,
  Character,
  isBoardItemCard,
  isBoardItemCharacter,
  isBoardItemInert,
} from "./battle";
import { assetUrl, Coordinate, isAdjacent } from "./utils";
import { move, takeAction } from "./state";
import { isCardEligible } from "./Card";

export function GameBoard(props: {
  battleState: BattleState;
  draggedCard: CardId | undefined;
}) {
  const battle = props.battleState.battle;
  const [selectedSquare, setSelectedSquare] = useState<Coordinate>();

  const backgroundImage = battle.background_image
    ? assetUrl(battle.background_image)
    : undefined;
  return (
    <table className="game-board" style={{ backgroundImage }}>
      <tbody>
        {battle.board.grid.members.map((row, y) => (
          <tr key={y}>
            {row.map((cell, x) => {
              let image: string | undefined;
              let character: Character | undefined;
              let isPlayer;
              let isInert = false;
              if (isBoardItemCharacter(cell)) {
                character = battle.characters[cell.id];
                if (character.image !== null) {
                  image = assetUrl(character.image);
                }
                if (character.health == 0) {
                  image = assetUrl("skull.png");
                }
                isPlayer = props.battleState.character_id === cell.id;
              } else if (isBoardItemCard(cell)) {
                image = assetUrl("card.png");
                isPlayer = false;
              } else if (isBoardItemInert(cell)) {
                isPlayer = false;
                isInert = true;
              }
              const curLocation: Coordinate = { x, y };
              const isSelectedSquare =
                selectedSquare &&
                selectedSquare.x === x &&
                selectedSquare.y === y;

              // Only ineligible if there is actively a card being dragged and that card isn't eligible.
              const isIneligible =
                props.draggedCard !== undefined &&
                (character?.health == 0 ||
                  !isCardEligible(isPlayer, props.draggedCard, battle));

              return (
                <td
                  key={x}
                  style={{
                    border: isInert ? 0 : undefined,
                    borderColor: isSelectedSquare ? "red" : "black",
                    backgroundImage: image,
                    opacity: isIneligible ? 0.5 : 1,
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
                  onClick={async () => {
                    if (isPlayer) {
                      if (isSelectedSquare) {
                        setSelectedSquare(undefined);
                      } else {
                        setSelectedSquare(curLocation);
                      }
                    } else {
                      if (
                        selectedSquare !== undefined &&
                        isAdjacent(selectedSquare, curLocation)
                      ) {
                        const item =
                          battle.board.grid.members[selectedSquare.y][
                            selectedSquare.x
                          ];
                        if (isBoardItemCharacter(item)) {
                          console.log("Trying to move");
                          setSelectedSquare(undefined);
                          await move(item.id, curLocation);
                        }
                      }
                    }
                  }}
                  title={character?.name}
                ></td>
              );
            })}
          </tr>
        ))}
      </tbody>
    </table>
  );
}
