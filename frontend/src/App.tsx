import { useState } from "react";
import { Header } from "./Header";
import {
  useGetActivities,
  useGetCourses,
  useGetSemesters,
} from "./api/apiComponents";
import { globalCss, css } from "./theme";
import { useSearchParam } from "./hooks/useSearchParam";
import { Label } from "@radix-ui/react-label";
import { MagnifyingGlassIcon } from "@radix-ui/react-icons";

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

      <main className={Styles.Main.container()}>
        <Label>
          <input
            className={Styles.Main.courseSearchInput()}
            type="text"
            placeholder="Search courses"
          />

          <MagnifyingGlassIcon />
        </Label>
      </main>
    </div>
  );
}
namespace Styles {
  export namespace Main {
    export const container = css({
      display: "flex",
      flexDirection: "column",
      width: "min($maxWidth, 100vw)",
      margin: "0 auto",
      padding: "$4 $4",
    });

    export const courseSearchInput = css({
      justifySelf: "stretch",
      // width: "140px",
      padding: "$3 $4",

      fontSize: "$4",
      fontWeight: 500,
    });
  }
}
