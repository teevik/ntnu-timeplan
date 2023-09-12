import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { BrowserRouter } from "react-router-dom";
import { FetchTransport, createClient } from "@rspc/client";
import { Procedures } from "../../api/bindings";
import { QueryClient } from "@tanstack/react-query";
import { rspc } from "./rspc";

const client = createClient<Procedures>({
  transport: new FetchTransport("http://0.0.0.0:8080/rspc"),
});

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <rspc.Provider client={client} queryClient={queryClient}>
        <App />
      </rspc.Provider>
    </BrowserRouter>
  </React.StrictMode>
);
