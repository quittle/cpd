import {
  type BattleHistoryEntry,
  BattleTextEntry,
  type TypedText,
} from "./battle";

import React from "react";

function convert(typedText: TypedText): React.ReactNode {
  if ("Text" in typedText) {
    return typedText.Text;
  }
  if ("Typed" in typedText) {
    const [battleType, text] = typedText.Typed;
    switch (battleType) {
      case BattleTextEntry.Id:
        return <b>{text}</b>;
      case BattleTextEntry.Attack:
        return <b>{text}</b>;
      case BattleTextEntry.Damage:
        return <b>{text}</b>;
    }
  }

  throw new Error(
    `Invalid TypedText encountered: ${JSON.stringify(typedText)}`,
  );
}

export default function BattleHistory(props: {
  readonly history: BattleHistoryEntry[];
}) {
  // The <li> is relatively safe because the entries are append only
  // the spread entry text is to suppress missing keys. No good way to treat
  // this as a list because content could contain multiple entries with the same
  // values so no reasonable key exists.
  return (
    <ol className="battle-history" reversed>
      {props.history
        .map((entry, index) => <li key={index}>{...entry.map(convert)}</li>)
        .toReversed()}
    </ol>
  );
}
