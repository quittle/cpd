import React, { useEffect, useRef } from "react";
import type { StoryCard, StoryCardEntry } from "./battle";

function storyCardEntryToReactNode(entry: StoryCardEntry): React.ReactNode {
  if ("h1" in entry) {
    return <h1>{entry.h1}</h1>;
  } else if ("p" in entry) {
    return <p>{entry.p}</p>;
  }
  throw new Error(`Unknown StoryCardEntry: ${JSON.stringify(entry)}`);
}

export function StoryCard(props: {
  readonly storyCard: StoryCard;
  readonly show: boolean;
  readonly onClose: () => void;
}): React.ReactNode {
  const buttonRef = useRef<HTMLDialogElement>(null);
  useEffect(() => {
    if (buttonRef.current) {
      if (props.show) {
        buttonRef.current.showModal();
      } else {
        buttonRef.current.close();
      }
    }
  }, [props.show, buttonRef]);

  return (
    <dialog ref={buttonRef}>
      <button onClick={props.onClose} type="button">
        X
      </button>

      {props.storyCard.map(storyCardEntryToReactNode)}
    </dialog>
  );
}
