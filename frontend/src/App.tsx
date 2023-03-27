import Immutable from "immutable";
import { TopBar } from "./TopBar";
import { Body } from "./Body";
import { Suspense } from "react";

export const swrOptions = {
  suspense: true,
  revalidateIfStale: false,
  revalidateOnFocus: false,
  revalidateOnReconnect: false,
} as const;

export interface SelectedCourseState {
  term: number;
  enabledStudentGroups: Immutable.Set<string>;
}

export function App() {
  return (
    <>
      <TopBar />
      <Suspense>
        <Body />
      </Suspense>
    </>
  );
}
