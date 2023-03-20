import { FormEvent, useEffect, useMemo, useRef, useState } from "react";
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
} from "ariakit";
import { OrderedMap } from "immutable";

const baseUrl = "http://localhost:3000";

const fetcher = <T,>(url: string) =>
  fetch(baseUrl + url).then((res) => res.json() as T);

const useFetch = <T,>(url: string) =>
  useSWR(url, fetcher<T>, { suspense: true }).data;

preload("/semesters", fetcher);
preload("/courses", fetcher);

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
      setSelectedCourses((pog) => pog.set(valueToAdd, { term: 1 }));

      courseSearch.setValue("");
    }
  };

  // let a = useSelectState();

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

        {[...selectedCourses.entries()].map(([courseCode, course]) => (
          <div style={{ display: "flex" }}>
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

            {/* <Select state={{}}></Select> */}
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
