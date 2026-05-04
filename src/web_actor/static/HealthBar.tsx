import React from "react";

interface HealthBarProps {
  readonly value: number;
  readonly max: number;
  readonly foregroundColor: string;
  readonly backgroundColor: string;
  readonly valueTextColor: string;
  readonly maxTextColor: string;
}

export default function HealthBar({
  value,
  max,
  foregroundColor,
  backgroundColor,
  valueTextColor,
  maxTextColor,
}: HealthBarProps) {
  const percentage = (value / max) * 100;

  return (
    <div
      className="health-bar"
      style={{
        borderColor: backgroundColor,
        background: `linear-gradient(to right, ${foregroundColor}, ${foregroundColor}, ${percentage}%, ${backgroundColor}, ${percentage}%, ${backgroundColor})`,
      }}
    >
      <span style={{ color: valueTextColor }}>{value}</span>

      <span style={{ color: maxTextColor }}>{value === max ? "" : max}</span>
    </div>
  );
}
