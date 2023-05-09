import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { CssBaseline, ThemeProvider, createTheme } from "@mui/material";
import { BrowserRouter } from "react-router-dom";

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <ThemeProvider theme={darkTheme}>
        <CssBaseline enableColorScheme />

        <App />
      </ThemeProvider>
    </BrowserRouter>
  </React.StrictMode>
);
