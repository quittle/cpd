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
      style={{
        textAlign: "center",
        position: "relative",
        fontFamily: "var(--font-family-pixelated)",
        border: `2px solid ${backgroundColor}`,
        background: `linear-gradient(to right, ${foregroundColor}, ${foregroundColor}, ${percentage}%, ${backgroundColor}, ${percentage}%, ${backgroundColor})`,
        // Source: https://pixelcorners.lukeb.co.uk
        clipPath: `polygon(
            3px 6px,
            6px 6px,
            6px 3px,
            calc(100% - 6px) 3px,
            calc(100% - 6px) 6px,
            calc(100% - 3px) 6px,
            calc(100% - 3px) calc(100% - 6px),
            calc(100% - 6px) calc(100% - 6px),
            calc(100% - 6px) calc(100% - 3px),
            6px calc(100% - 3px),
            6px calc(100% - 6px),
            3px calc(100% - 6px)
        )`,
        padding: "0.25em",
        margin: "0.25em 0",
      }}
    >
      <span style={{ color: textColor }}>
        {value} / {max}
      </span>
    </div>
  );
};

export default MeterBar;
