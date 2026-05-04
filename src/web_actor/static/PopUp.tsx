import React, { useEffect, useRef } from "react";

export function PopUp(
  props: React.PropsWithChildren<{
    readonly onClose: () => void;
  }>,
): React.ReactNode {
  const buttonRef = useRef<HTMLDialogElement>(null);
  const [show, setShow] = React.useState(true);
  useEffect(() => {
    if (buttonRef.current) {
      if (show) {
        buttonRef.current.showModal();
      } else {
        buttonRef.current.close();
      }
    }
  }, [show, buttonRef]);

  return (
    <dialog className="pop-up" ref={buttonRef}>
      <button
        onClick={() => {
          setShow(false);
          props.onClose();
        }}
        type="button"
      >
        X
      </button>

      {props.children}
    </dialog>
  );
}
