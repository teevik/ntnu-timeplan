import {
  FormEvent,
  Suspense,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.scss";
import useSWR, { preload } from "swr";
import { SemestersWithCurrent } from "../../shared/bindings/SemestersWithCurrent";
import { Course } from "../../shared/bindings/Course";
import {
  Button,
  Combobox,
  ComboboxItem,
  ComboboxPopover,
  useComboboxState,
  FormSubmit,
  FormInput,
  Select,
  useSelectState,
  SelectPopover,
  SelectItem,
} from "ariakit";
import { OrderedMap } from "immutable";
import { Activity } from "../../shared/bindings/Activity";

const baseUrl = "http://localhost:3000";

const fetcher = <T,>(url: string) =>
  fetch(baseUrl + url).then((res) => res.json() as T);

const useFetch = <T,>(url: string) =>
  useSWR(url, fetcher<T>, { suspense: true }).data;

preload("/semesters", fetcher);
preload("/courses", fetcher);

interface SelectedSemesterProps {
  courseCode: string;
  courseTerm: number;
  semester: string;
}

function SelectedSemester(props: SelectedSemesterProps) {
  const { courseCode, courseTerm, semester } = props;
  let activities = useFetch<Activity[]>(
    `/activities?courseCode=${courseCode}&courseTerm=${courseTerm}&semester=${semester}`
  );

  const pog = useMemo(() => {
    let allStudentGroups = new Set<string>();

    for (const activity of activities) {
      for (const studentGroup of activity.studentGroups) {
        allStudentGroups.add(studentGroup);
      }
    }

    return allStudentGroups;
  }, [activities]);

  return (
    <div>
      {Array.from(pog.values()).map((pog) => (
        <div key={pog}>
          <label>
            <input type="checkbox"></input>
            {pog}
          </label>
        </div>
      ))}
    </div>
  );
}

function App() {
  const semesters = useFetch<SemestersWithCurrent>("/semesters");
  const courses = useFetch<Record<string, Course>>("/courses");

  interface SelectedCourse {
    term: number;
  }

  const [selectedCourses, setSelectedCourses] = useState(() =>
    OrderedMap<string, SelectedCourse>()
  );

  const courseSearch = useComboboxState({
    gutter: 4,
    sameWidth: true,
    virtualFocus: true,
  });

  const searchQuery = courseSearch.value;

  const searchedCourses = useMemo(() => {
    return Object.entries(courses)
      .filter(
        ([courseCode, course]) =>
          courseCode.toLowerCase().includes(searchQuery.toLowerCase()) ||
          course.name.toLowerCase().includes(searchQuery.toLowerCase())
      )
      .slice(0, 10);
  }, [searchQuery, courses]);

  const onSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    // const valueToAdd = courseSearch.value;
    const valueToAdd = searchedCourses[0]?.[0];

    if (valueToAdd !== undefined) {
      setSelectedCourses((pog) =>
        pog.set(valueToAdd, {
          term: 1,
        })
      );

      courseSearch.setValue("");
    }
  };

  const semesterSelectState = useSelectState({
    defaultValue: semesters.currentSemester,
  });

  return (
    <div className="App">
      <div className="wrapper">
        <form onSubmit={onSubmit}>
          <label className="label">
            Your favorite fruit
            <Combobox
              style={{ width: 200 }}
              state={courseSearch}
              placeholder="e.g., Apple"
              className="combobox"
            />
          </label>
          <ComboboxPopover state={courseSearch} className="popover">
            {searchedCourses.map(([courseCode, course]) => (
              <ComboboxItem
                key={courseCode}
                className="combobox-item"
                value={courseCode}
                style={{ backgroundColor: "hotpink" }}
              >
                <p>{courseCode}</p>
                <p>{course.name}</p>
              </ComboboxItem>
            ))}
          </ComboboxPopover>
          <FormSubmit>Pog</FormSubmit>
        </form>

        <Select state={semesterSelectState}></Select>
        <SelectPopover state={semesterSelectState}>
          {Object.entries(semesters.semesters).map(([pog, semester]) => (
            <SelectItem
              key={pog}
              value={pog}
              style={{ backgroundColor: "#2f2f2f" }}
            >
              {semester.name}
            </SelectItem>
          ))}
        </SelectPopover>

        {[...selectedCourses.entries()].map(([courseCode, course]) => {
          const amountOfTerms = courses[courseCode].amountOfTerms;

          const pog =
            amountOfTerms > 1 ? (
              <>
                <p>Term:</p>
                <p>{course.term}</p>
                <Button
                  onClick={() => {
                    if (course.term >= amountOfTerms) return;

                    setSelectedCourses((pog) =>
                      pog.update(courseCode, (map) => ({
                        ...map!,
                        term: map!.term + 1,
                      }))
                    );
                  }}
                >
                  +
                </Button>
                <Button
                  onClick={() => {
                    if (course.term <= 1) return;
                    setSelectedCourses((pog) =>
                      pog.update(courseCode, (map) => ({
                        ...map!,
                        term: map!.term - 1,
                      }))
                    );
                  }}
                >
                  -
                </Button>
              </>
            ) : null;

          return (
            <div
              key={courseCode}
              style={{
                background: "#2f2f2f",
              }}
            >
              <p key={courseCode}>{courseCode}</p>
              <Button
                onClick={() => {
                  setSelectedCourses((selectedCourses) =>
                    selectedCourses.remove(courseCode)
                  );
                }}
              >
                x
              </Button>

              <Suspense fallback={<div>Loading...</div>}>
                <SelectedSemester
                  courseCode={courseCode}
                  courseTerm={course.term}
                  semester={semesterSelectState.value}
                ></SelectedSemester>
              </Suspense>

              {pog}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default App;
