import { Header } from "./Header";
import { globalCss, css } from "./theme";
import { useSearchParam } from "./hooks/useSearchParam";
import { rspc } from "./rspc";
import { useEffect, useMemo, useState } from "react";
import { SearchBar } from "./SearchBar";
import { SelectedCourses } from "./SelectedCourses";
import { CalendarQuery } from "../../api/bindings";
import { calendarEndpoint } from "./main";

function useEncodeCalendarQuery(queries: CalendarQuery[]) {
  const query = rspc.useQuery(["encode-calendar-query", queries], {
    suspense: true,
  }).data!;

  return query;
}

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

export interface SelectedCourseState {
  courseCode: string;
  term: number;
  enabledStudentGroups: string[];
}

export function App() {
  const semesters = rspc.useQuery(["semesters"], {
    suspense: true,
  }).data!;

  const courses = rspc.useQuery(["courses"], {
    suspense: true,
  }).data!;

  // TODO refactor out to a single hook
  let [searchParam, setSearchParam] = useSearchParam("courses", "");
  const [selectedCourses, setSelectedCourses] = useState(() => {
    if (searchParam.length != 0) {
      try {
        const jsonData = JSON.parse(atob(searchParam));

        return jsonData as SelectedCourseState[];
      } catch (e) {
        return [];
      }
    }

    return [];
  });

  useEffect(() => {
    setSearchParam(btoa(JSON.stringify(selectedCourses)));
  }, [selectedCourses]);

  const [selectedSemester, setSelectedSemester] = useSearchParam(
    "semester",
    semesters.currentSemester
  );

  const calendarQuery = useEncodeCalendarQuery(
    selectedCourses.map(({ courseCode, term, enabledStudentGroups }) => ({
      customName: courses[courseCode].name,
      identifier: { courseCode, courseTerm: term, semester: selectedSemester },
      studentGroups: enabledStudentGroups,
    }))
  );

  const webcalUrl = "webcal://ntnu-timeplan-api.fly.dev";

  const calendarQueryUrl = `${webcalUrl}/calendar.ics?query=${calendarQuery}`;

  return (
    <div>
      <Header
        semesters={semesters}
        selectedSemester={selectedSemester}
        setSelectedSemester={setSelectedSemester}
      />

      <main className={Styles.container()}>
        <SearchBar
          courses={courses}
          selectedCourses={selectedCourses}
          setSelectedCourses={setSelectedCourses}
        />

        <SelectedCourses
          courses={courses}
          selectedSemester={selectedSemester}
          selectedCourses={selectedCourses}
          setSelectedCourses={setSelectedCourses}
        />

        <a
          style={{ marginTop: "20px", wordBreak: "break-all" }}
          href={calendarQueryUrl}
        >
          {calendarQueryUrl}
        </a>
      </main>
    </div>
  );
}
namespace Styles {
  export const container = css({
    display: "flex",
    flexDirection: "column",
    width: "min($maxWidth, 100vw)",
    margin: "0 auto",
    padding: "$4 $4",
  });
}
