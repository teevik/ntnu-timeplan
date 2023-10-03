import { Label } from "@radix-ui/react-label";
import { MagnifyingGlassIcon } from "@radix-ui/react-icons";
import { css } from "./theme";
import { useCombobox } from "downshift";
import { Dispatch, useMemo, useState } from "react";
import { Course } from "../../api/bindings";
import { SelectedCourseState } from "./App";

interface SearchBarProps {
  courses: Record<string, Course>;
  selectedCourses: SelectedCourseState[];
  setSelectedCourses: Dispatch<
    (prev: SelectedCourseState[]) => SelectedCourseState[]
  >;
}

export function SearchBar(props: SearchBarProps) {
  const { courses, selectedCourses, setSelectedCourses } = props;

  const [courseSearch, setCourseSearch] = useState("");
  const courseEntries = useMemo(() => Object.entries(courses), [courses]);

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

  const downshift = useCombobox({
    inputValue: courseSearch,
    onInputValueChange({ inputValue }) {
      setCourseSearch(inputValue ?? "");
    },

    onSelectedItemChange(changes) {
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
        className={Styles.SearchResults.listItem()}
        {...downshift.getItemProps({ item: courseCode, index })}
      >
        <span className={Styles.SearchResults.courseName()}>{course.name}</span>
        <span className={Styles.SearchResults.courseCode()}>{courseCode}</span>
      </li>
    );
  }

  return (
    <div className={Styles.container()}>
      <Label className={Styles.label()}>
        <input
          className={Styles.input()}
          type="text"
          placeholder="Søk studier"
          {...downshift.getInputProps()}
        />

        <MagnifyingGlassIcon className={Styles.icon()} />
      </Label>

      <ul
        className={Styles.SearchResults.list()}
        hidden={!showSearchResults}
        {...downshift.getMenuProps()}
      >
        {downshift.isOpen &&
          searchedCourseCodes.map((courseName, index) =>
            renderSearchResult(courseName, index)
          )}
      </ul>
    </div>
  );
}

namespace Styles {
  export const container = css({
    position: "relative",
    marginBottom: "$4",
  });

  export const label = css({
    display: "flex",

    fontSize: "$4",
    fontWeight: 500,
    borderRadius: 3,

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

  export namespace SearchResults {
    export const list = css({
      zIndex: 1,
      position: "absolute",
      top: "100%",
      left: 0,
      right: 0,

      display: "flex",
      flexDirection: "column",

      padding: 0,
    });

    export const listItem = css({
      display: "flex",
      flexDirection: "column",
      cursor: "pointer",

      padding: "$2 $4",

      backgroundColor: "color-mix(in lch, $background, white 20%)",
      transition: "background-color .1s ease-in-out",

      "&:hover": {
        backgroundColor: "color-mix(in lch, $background, white 30%)",
      },
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
