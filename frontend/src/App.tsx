import { useState } from "react";
import { Header } from "./Header";
import { useGetActivities, useGetSemesters } from "./api/apiComponents";
import { globalCss, css } from "./theme";
import { useSearchParam } from "./hooks/useSearchParam";

const globalStyles = globalCss({
  body: {
    fontFamily: "$normal",
    backgroundColor: "$background",
    color: "$foreground",

    margin: 0,
  },

  "*, ::before, ::after": {
    boxSizing: "border-box",
    outline: "none",
  },
});

globalStyles();

export function App() {
  const semesters = useGetSemesters({}, { suspense: true }).data!;

  const [selectedSemester, setSelectedSemester] = useSearchParam(
    "semester",
    semesters.currentSemester
  );

  return (
    <div>
      <Header
        semesters={semesters}
        selectedSemester={selectedSemester}
        setSelectedSemester={setSelectedSemester}
      />
    </div>
  );
}
