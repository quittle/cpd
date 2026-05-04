import React from "react";

interface MeterBarProps {
  readonly value: number;
  readonly max: number;
  readonly foregroundColor: string;
  readonly backgroundColor: string;
  readonly textColor: string;
}

export function MeterBar({
  value,
  max,
  foregroundColor,
  backgroundColor,
  textColor,
}: MeterBarProps) {
  const percentage = (value / max) * 100;

  return (
    <div
      className="meter-bar"
      style={{
        borderColor: backgroundColor,
        background: `linear-gradient(to right, ${foregroundColor}, ${foregroundColor}, ${percentage}%, ${backgroundColor}, ${percentage}%, ${backgroundColor})`,
      }}
    >
      <span style={{ color: textColor }}>
        {value} / {max}
      </span>
    </div>
  );
}
