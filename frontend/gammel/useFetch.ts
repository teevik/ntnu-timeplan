import useSWR from "swr";
import { swrOptions } from "./App";

// export const baseUrl = "http://localhost:3000";
export const baseUrl = "https://ntnu-timeplan-api.fly.dev";

export const fetcher = <T>(url: string) =>
  fetch(baseUrl + url).then((response) => response.json() as T);

export const useFetch = <T>(url: string) =>
  useSWR(url, fetcher<T>, swrOptions).data;
