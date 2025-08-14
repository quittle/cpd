import React, { useEffect, useRef } from "react";

export function PopUp(
  props: React.PropsWithChildren<{
    onClose: () => void;
    className?: string;
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
    <dialog ref={buttonRef} className={props.className}>
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
