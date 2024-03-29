import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { BrowserRouter } from "react-router-dom";
import { FetchTransport, createClient } from "@rspc/client";
import { Procedures } from "../../api/bindings";
import { QueryClient } from "@tanstack/react-query";
import { rspc } from "./rspc";

const endpoint = import.meta.env.PROD
  ? "https://ntnu-timeplan-api.fly.dev"
  : "http://0.0.0.0:8080";

export const calendarEndpoint = (encodedQuery: string) =>
  `${endpoint}/calendar.ics?query=${encodedQuery}`;

const rspcEndpoint = `${endpoint}/rspc`;

const client = createClient<Procedures>({
  transport: new FetchTransport(rspcEndpoint),
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
