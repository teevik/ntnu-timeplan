import React, { Suspense } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { CssBaseline, ThemeProvider, createTheme } from "@mui/material";

import "./index.scss";

const darkTheme = createTheme({
  palette: {
    mode: "dark",
  },
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider theme={darkTheme}>
      <CssBaseline enableColorScheme />

      <Suspense fallback={<h1>Loading...</h1>}>
        <App />
      </Suspense>
    </ThemeProvider>
  </React.StrictMode>
);
