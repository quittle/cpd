import React, { useEffect, useRef } from "react";

export function PopUp(
  props: React.PropsWithChildren<{
    readonly onClose: () => void;
    readonly className?: string;
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
    <dialog className={props.className} ref={buttonRef}>
      <button
        onClick={() => {
          setShow(false);
          props.onClose();
        }}
      >
        X
      </button>

      {props.children}
    </dialog>
  );
}
