import { createStitches } from "@stitches/react";

export const { styled, css, globalCss, theme } = createStitches({
  theme: {
    colors: {
      background: "#0f1511",
      foreground: "#fcfdfc",
      primary: "#d463b0",
      secondary: "#e7f8ef",
      accent: "#53eabf",
    },

    fonts: {
      normal: "'Inter', sans-serif",
    },

    space: {
      1: "4px",
      2: "8px",
      3: "16px",
      4: "32px",
    },

    fontSizes: {
      1: "12px",
      2: "15px",
      3: "18px",
      4: "22px",
      5: "24px",
    },

    shadows: {
      1: "rgba(0, 0, 0, 0.2) 0px 2px 1px -1px, rgba(0, 0, 0, 0.14) 0px 1px 1px 0px, rgba(0, 0, 0, 0.12) 0px 1px 3px 0px",
      2: "rgba(0, 0, 0, 0.2) 0px 3px 1px -2px, rgba(0, 0, 0, 0.14) 0px 2px 2px 0px, rgba(0, 0, 0, 0.12) 0px 1px 5px 0px",
      3: "rgba(0, 0, 0, 0.2) 0px 3px 3px -2px, rgba(0, 0, 0, 0.14) 0px 3px 4px 0px, rgba(0, 0, 0, 0.12) 0px 1px 8px 0px",
    },

    sizes: {
      maxWidth: "1200px",
    },
  },
});
