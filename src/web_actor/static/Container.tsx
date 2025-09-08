import React from "react";
import { BattleState, CharacterId, Content } from "./battle";
import { assetPath, describeContent, getCharacterCoordinate } from "./utils";
import { PopUp } from "./PopUp";
import { takeContent } from "./state";

export default function Container(props: {
  characterId: CharacterId;
  battleState: BattleState;
  contents: Content[];
  onClose: () => void;
}) {
  const character = props.battleState.battle.characters[props.characterId];
  return (
    <PopUp onClose={props.onClose} className="container">
      <div className="header">
        <img src={assetPath(character.image)} />
        <h3>{character.name}</h3>
      </div>
      <div className="contents">
        {props.contents.map((content) => {
          const { key, assetUrl } = describeContent(
            content,
            props.battleState.battle,
          );
          return (
            <button
              key={key}
              style={{ backgroundImage: assetUrl }}
              onClick={async () => {
                let item;
                if ("Card" in content) {
                  item = { card: content.Card };
                } else {
                  item = { object: content.Object };
                }

                await takeContent(
                  props.battleState.character_id,
                  getCharacterCoordinate(
                    props.battleState.battle,
                    props.characterId,
                  ),
                  item,
                );
              }}
            />
          );
        })}
      </div>
    </PopUp>
  );
}
