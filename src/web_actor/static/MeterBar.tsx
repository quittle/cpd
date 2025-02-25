import React from "react";

interface MeterBarProps {
  value: number;
  max: number;
  foregroundColor: string;
  backgroundColor: string;
  textColor: string;
}

const MeterBar: React.FC<MeterBarProps> = ({
  value,
  max,
  foregroundColor,
  backgroundColor,
  textColor,
}) => {
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
};

export default MeterBar;
