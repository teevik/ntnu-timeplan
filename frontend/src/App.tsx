import { Header } from "./Header";
import { globalCss, css } from "./theme";
import { useSearchParam } from "./hooks/useSearchParam";
import { Label } from "@radix-ui/react-label";
import { MagnifyingGlassIcon } from "@radix-ui/react-icons";
import { rspc } from "./rspc";
import { useCombobox } from "downshift";
import { useEffect, useMemo, useState } from "react";
import { CourseCard } from "./CourseCard";

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

interface SelectedCourseState {
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
  const courseEntries = useMemo(() => Object.entries(courses), [courses]);

  const [courseSearch, setCourseSearch] = useState("");

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

  const searchedCourseCodes = useMemo(
    () =>
      courseEntries
        .filter(
          ([courseCode, course]) =>
            courseCode.toLowerCase().includes(courseSearch.toLowerCase()) ||
            course.name.toLowerCase().includes(courseSearch.toLowerCase())
        )
        .slice(0, 50)
        .filter(
          ([courseCode, _]) =>
            selectedCourses.find(
              (selectedCourse) => selectedCourse.courseCode == courseCode
            ) == undefined
        )
        .map(([courseCode, course]) => courseCode),
    [courseEntries, courseSearch, selectedCourses]
  );
  const [selectedSemester, setSelectedSemester] = useSearchParam(
    "semester",
    semesters.currentSemester
  );

  const downshift = useCombobox({
    inputValue: courseSearch,
    onInputValueChange({ inputValue }) {
      setCourseSearch(inputValue ?? "");
    },

    onSelectedItemChange(changes) {
      console.log(changes);
      const courseCode = changes.selectedItem!;

      setSelectedCourses((oldSelectedCourses) => [
        ...oldSelectedCourses,
        { courseCode, enabledStudentGroups: [], term: 1 },
      ]);

      // Hack to actually set course search
      requestAnimationFrame(() => setCourseSearch(""));
    },

    items: searchedCourseCodes,
  });

  const showSearchResults = downshift.isOpen && searchedCourseCodes.length != 0;

  function renderSearchResult(courseCode: string, index: number) {
    const course = courses[courseCode];

    return (
      <li
        key={courseCode}
        className={Styles.Main.SearchResults.listItem()}
        {...downshift.getItemProps({ item: courseCode, index })}
      >
        <span className={Styles.Main.SearchResults.courseName()}>
          {course.name}
        </span>
        <span className={Styles.Main.SearchResults.courseCode()}>
          {courseCode}
        </span>
      </li>
    );
  }

  return (
    <div>
      <Header
        semesters={semesters}
        selectedSemester={selectedSemester}
        setSelectedSemester={setSelectedSemester}
      />

      <main className={Styles.Main.container()}>
        <Label className={Styles.Main.Search.label()}>
          <input
            className={Styles.Main.Search.input()}
            type="text"
            placeholder="Søk studier"
            {...downshift.getInputProps()}
          />

          <MagnifyingGlassIcon className={Styles.Main.Search.icon()} />
        </Label>

        <ul
          className={Styles.Main.SearchResults.list()}
          hidden={!showSearchResults}
          {...downshift.getMenuProps()}
        >
          {downshift.isOpen &&
            searchedCourseCodes.map((courseName, index) =>
              renderSearchResult(courseName, index)
            )}
        </ul>

        <div>
          {selectedCourses.map(({ courseCode, enabledStudentGroups, term }) => (
            <div key={courseCode}>
              <CourseCard
                courseCode={courseCode}
                term={term}
                setTerm={(term) =>
                  setSelectedCourses((selectedCourses) => {
                    const arrayIndex = selectedCourses.findIndex(
                      (course) => course.courseCode === courseCode
                    );

                    return selectedCourses.with(arrayIndex, {
                      courseCode,
                      enabledStudentGroups: [],
                      term,
                    });
                  })
                }
                semester={selectedSemester}
                course={courses[courseCode]}
                onRemove={() =>
                  setSelectedCourses(() =>
                    selectedCourses.filter(
                      (selectedCourse) =>
                        selectedCourse.courseCode !== courseCode
                    )
                  )
                }
                enabledStudentGroups={enabledStudentGroups}
                toggleStudentGroup={(studentGroup) =>
                  setSelectedCourses(() => {
                    const arrayIndex = selectedCourses.findIndex(
                      (course) => course.courseCode === courseCode
                    );

                    let newEnabledStudentGroups;

                    if (enabledStudentGroups.includes(studentGroup)) {
                      newEnabledStudentGroups = [
                        ...enabledStudentGroups,
                        studentGroup,
                      ];
                    } else {
                      newEnabledStudentGroups = enabledStudentGroups.filter(
                        (target) => target !== studentGroup
                      );
                    }

                    return selectedCourses.with(arrayIndex, {
                      courseCode,
                      enabledStudentGroups: enabledStudentGroups,
                      term,
                    });
                  })
                }
              />
            </div>
          ))}
        </div>
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

    export namespace Search {
      export const label = css({
        display: "flex",

        fontSize: "$4",
        fontWeight: 500,

        color: "color-mix(in lch, $foreground, black 5%)",
        backgroundColor: "color-mix(in lch, $background, white 10%)",
      });

      export const input = css({
        flexGrow: 1,
        padding: "$3 $4",

        fontSize: "$7",
        fontWeight: 500,

        color: "inherit",
        background: "none",
        border: "none",

        "&::placeholder": {
          opacity: 1,
        },
      });

      export const icon = css({
        alignSelf: "center",
        width: 32,
        height: 32,
        marginRight: "$3",
      });
    }

    export namespace SearchResults {
      export const list = css({
        display: "flex",
        flexDirection: "column",

        backgroundColor: "color-mix(in lch, $background, white 20%)",
      });

      export const listItem = css({
        display: "flex",
        flexDirection: "column",
        cursor: "pointer",
      });

      export const courseName = css({
        fontSize: "$4",
        fontWeight: 400,
      });

      export const courseCode = css({
        fontSize: "$2",
        fontWeight: 500,
        opacity: 0.8,
      });
    }
  }
}
