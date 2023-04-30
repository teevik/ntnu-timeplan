import { SelectedCourseState } from "./App";
import {
  Container,
  Autocomplete,
  TextField,
  MenuItem,
  Paper,
  Typography,
} from "@mui/material";
import Immutable, { OrderedMap } from "immutable";
import { useMemo, useState, Suspense } from "react";
import { Course } from "../../api/bindings/Course";
import { SemestersWithCurrent } from "../../api/bindings/SemestersWithCurrent";
import { CalendarUrl } from "./CalendarUrl";
import { CourseCard } from "./CourseCard";
import { useFetch } from "./useFetch";
import Grid from "@mui/material/Unstable_Grid2";
import { Timetable } from "./Timetable";

export function Body() {
  const semesters = useFetch<SemestersWithCurrent>("/semesters");
  const courses = useFetch<Record<string, Course>>("/courses");

  const semesterCodes = useMemo(
    () => Object.keys(semesters.semesters).sort(),
    [semesters]
  );

  const [selectedCourses, setSelectedCourses] = useState(() =>
    OrderedMap<string, SelectedCourseState>()
  );

  const [courseSearch, setCourseSearch] = useState("");

  const searchedCourses = useMemo(() => {
    return Object.entries(courses)
      .filter(
        ([courseCode, course]) =>
          courseCode.toLowerCase().includes(courseSearch.toLowerCase()) ||
          course.name.toLowerCase().includes(courseSearch.toLowerCase())
      )
      .slice(0, 50)
      .map(([courseCode, course]) => courseCode);
  }, [courseSearch, courses]);

  const [selectedSemester, setSelectedSemester] = useState(
    semesters.currentSemester
  );

  const [selectedCourse, setSelectedCourse] = useState<string | null>(null);

  return (
    <>
      <Container>
        <Grid container spacing={2} mt={4}>
          <Grid xs={7} md={8} lg={10}>
            <Autocomplete
              fullWidth
              options={searchedCourses}
              filterOptions={(options, state) => options}
              renderInput={(params) => (
                <TextField {...params} label="Add course" />
              )}
              getOptionLabel={(option) => `${option} - ${courses[option].name}`}
              inputValue={courseSearch}
              onInputChange={(_, newCourseSearch) =>
                setCourseSearch(newCourseSearch)
              }
              value={selectedCourse}
              onChange={(_, courseCode) => {
                if (courseCode !== null) {
                  setSelectedCourses((selectedCourses) =>
                    selectedCourses.set(courseCode, {
                      term: 1,
                      enabledStudentGroups: Immutable.Set(),
                    })
                  );

                  setSelectedCourse(null);
                  setCourseSearch("");
                }
              }}
            />
          </Grid>

          <Grid xs={5} md={4} lg={2}>
            <TextField
              fullWidth
              select
              label="Semester"
              value={selectedSemester}
              onChange={(e) => setSelectedSemester(e.target.value)}
            >
              {semesterCodes.map((semesterCode) => (
                <MenuItem key={semesterCode} value={semesterCode}>
                  {semesters.semesters[semesterCode].name}
                </MenuItem>
              ))}
            </TextField>
          </Grid>
        </Grid>

        <Grid container spacing={2} mt={2}>
          {Array.from(selectedCourses.entries()).map(
            ([courseCode, selectedCourse]) => (
              <Grid key={courseCode} sm={12} md={6} lg={4}>
                <CourseCard
                  courseCode={courseCode}
                  term={selectedCourse.term}
                  setTerm={(term) =>
                    setSelectedCourses((selectedCourses) =>
                      selectedCourses.set(courseCode, {
                        enabledStudentGroups: Immutable.Set<string>(),
                        term,
                      })
                    )
                  }
                  semester={selectedSemester}
                  course={courses[courseCode]}
                  onRemove={() =>
                    setSelectedCourses((selectedCourses) =>
                      selectedCourses.remove(courseCode)
                    )
                  }
                  enabledStudentGroups={selectedCourse.enabledStudentGroups}
                  toggleStudentGroup={(studentGroup) =>
                    setSelectedCourses((selectedCourses) =>
                      selectedCourses.update(courseCode, (selectedCourse) => {
                        const { enabledStudentGroups } = selectedCourse!;

                        const newEnabledStudentGroups =
                          enabledStudentGroups.includes(studentGroup)
                            ? enabledStudentGroups.remove(studentGroup)
                            : enabledStudentGroups.add(studentGroup);

                        return {
                          ...selectedCourse!,
                          enabledStudentGroups: newEnabledStudentGroups,
                        };
                      })
                    )
                  }
                />
              </Grid>
            )
          )}
        </Grid>

        <Suspense fallback={<Typography>Loading...</Typography>}>
          <CalendarUrl
            semester={selectedSemester}
            selectedCourses={selectedCourses}
          />
        </Suspense>

        <Timetable
          semester={selectedSemester}
          selectedCourses={selectedCourses}
          courses={courses}
        />
      </Container>
    </>
  );
}
