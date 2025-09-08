import React from "react";
import { BattleHistoryEntry, TypedText } from "./battle";

function convert(typedText: TypedText): React.ReactNode {
  if ("Text" in typedText) {
    return typedText.Text;
  } else if ("Typed" in typedText) {
    const [battleType, text] = typedText.Typed;
    switch (battleType) {
      case "Id":
        return <b>{text}</b>;
      case "Attack":
        return <b>{text}</b>;
      case "Damage":
        return <b>{text}</b>;
    }
  } else {
    throw new Error(
      `Invalid TypedText encountered: ${JSON.stringify(typedText)}`,
    );
  }
}

export default function BattleHistory(props: {
  history: BattleHistoryEntry[];
}) {
  // The <li> is relatively safe because the entries are append only
  // The spread entry text is to suppress missing keys. No good way to treat
  // this as a list because content could contain multiple entries with the same
  // values so no reasonable key exists.
  return (
    <ol className="battle-history" reversed={true}>
      {props.history
        .map((entry, index) => <li key={index}>{...entry.map(convert)}</li>)
        .toReversed()}
    </ol>
  );
}
