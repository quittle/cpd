import React from "react";

interface HealthBarProps {
  value: number;
  max: number;
  foregroundColor: string;
  backgroundColor: string;
  valueTextColor: string;
  maxTextColor: string;
}

const HealthBar: React.FC<HealthBarProps> = ({
  value,
  max,
  foregroundColor,
  backgroundColor,
  valueTextColor,
  maxTextColor,
}) => {
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
      <span style={{ color: maxTextColor }}>{value == max ? "" : max}</span>
    </div>
  );
};

export default HealthBar;
