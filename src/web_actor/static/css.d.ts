import "react"; // eslint-disable-line react/no-typos

declare module "react" {
  interface CSSProperties {
    positionAnchor?: string;
  }
}
