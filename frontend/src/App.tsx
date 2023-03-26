import { Suspense, useMemo, useState } from "react";
import "./App.scss";
import useSWR, { preload } from "swr";
import Immutable, { OrderedMap } from "immutable";
import { SemestersWithCurrent } from "../../api/bindings/SemestersWithCurrent";
import { Course } from "../../api/bindings/Course";
import { Activity } from "../../api/bindings/Activity";
import { CalendarQuery } from "../../api/bindings/CalendarQuery";
import Grid from "@mui/material/Unstable_Grid2";
import ClearIcon from "@mui/icons-material/Clear";
import {
  Divider,
  CardContent,
  TextField,
  MenuItem,
  Typography,
  FormControlLabel,
  Checkbox,
  Card,
  CardHeader,
  IconButton,
  Container,
  Autocomplete,
  Paper,
  Link,
  Stack,
} from "@mui/material";

const baseUrl = "http://localhost:3000";

const fetcher = <T,>(url: string) =>
  fetch(baseUrl + url).then((res) => res.json() as T);

const useFetch = <T,>(url: string) =>
  useSWR(url, fetcher<T>, {
    suspense: true,
    revalidateIfStale: false,
    revalidateOnFocus: false,
    revalidateOnReconnect: false,
  }).data;

preload("/semesters", fetcher);
preload("/courses", fetcher);

function formatStudentGroup(studentGroup: string) {
  studentGroup = studentGroup.replaceAll("_", " ");
  studentGroup =
    studentGroup[0].toUpperCase() + studentGroup.slice(1).toLowerCase();

  return studentGroup;
}

interface TermSelectorProps {
  term: number;
  amountOfTerms: number;
  setTerm: (newTerm: number) => void;
}

function TermSelector(props: TermSelectorProps) {
  const { term, amountOfTerms, setTerm } = props;

  if (amountOfTerms == 1) return null;

  return (
    <>
      <Divider />

      <CardContent>
        <TextField
          select
          label="Term"
          value={term}
          onChange={(e) => setTerm(parseInt(e.target.value))}
        >
          {Array.from({ length: amountOfTerms }, (_, index) => index + 1).map(
            (term) => (
              <MenuItem key={term} value={term}>
                Term {term}
              </MenuItem>
            )
          )}
        </TextField>
      </CardContent>
    </>
  );
}

interface SelectStudentGroupsProps {
  courseCode: string;
  term: number;
  semester: string;
  enabledStudentGroups: Immutable.Set<string>;
  toggleStudentGroup: (studentGroup: string) => void;
}

function SelectStudentGroups(props: SelectStudentGroupsProps) {
  const {
    courseCode,
    term,
    semester,
    enabledStudentGroups,
    toggleStudentGroup,
  } = props;

  let activities = useFetch<Activity[]>(
    `/activities?courseCode=${courseCode}&courseTerm=${term}&semester=${semester}`
  );

  const allStudentGroups = useMemo(() => {
    let allStudentGroups = new Set<string>();

    for (const activity of activities) {
      for (const studentGroup of activity.studentGroups) {
        allStudentGroups.add(studentGroup);
      }
    }

    return allStudentGroups;
  }, [activities]);

  if (allStudentGroups.size == 0) return null;

  return (
    <>
      <Divider />
      <CardContent>
        <Typography variant="h5">Student groups</Typography>
        {Array.from(allStudentGroups.values()).map((studentGroup) => (
          <Stack key={studentGroup}>
            <FormControlLabel
              control={
                <Checkbox
                  checked={enabledStudentGroups.contains(studentGroup)}
                  onClick={() => toggleStudentGroup(studentGroup)}
                />
              }
              label={formatStudentGroup(studentGroup)}
            />
          </Stack>
        ))}
      </CardContent>
    </>
  );
}

interface CourseCardProps {
  courseCode: string;
  term: number;
  setTerm: (newTerm: number) => void;
  semester: string;
  course: Course;
  onRemove: () => void;
  enabledStudentGroups: Immutable.Set<string>;
  toggleStudentGroup: (studentGroup: string) => void;
}

function CourseCard(props: CourseCardProps) {
  const {
    courseCode,
    term,
    setTerm,
    semester,
    course,
    enabledStudentGroups,
    toggleStudentGroup,
  } = props;

  return (
    <Card key={courseCode}>
      <CardHeader
        title={course.name}
        subheader={courseCode}
        action={
          <IconButton aria-label="Remove" onClick={props.onRemove}>
            <ClearIcon />
          </IconButton>
        }
      />

      <TermSelector
        term={term}
        setTerm={setTerm}
        amountOfTerms={course.amountOfTerms}
      />

      <Suspense fallback={null}>
        <SelectStudentGroups
          courseCode={courseCode}
          term={term}
          semester={semester}
          enabledStudentGroups={enabledStudentGroups}
          toggleStudentGroup={toggleStudentGroup}
        />
      </Suspense>
    </Card>
  );
}

function App() {
  const semesters = useFetch<SemestersWithCurrent>("/semesters");
  const courses = useFetch<Record<string, Course>>("/courses");

  const semesterCodes = useMemo(
    () => Object.keys(semesters.semesters).sort(),
    [semesters]
  );

  const courseCodes = useMemo(() => Object.keys(courses), [courses]);

  interface SelectedCourseState {
    term: number;
    enabledStudentGroups: Immutable.Set<string>;
  }

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

  const queries: CalendarQuery[] = [];
  for (const [courseCode, selectedCourse] of selectedCourses) {
    queries.push({
      identifier: {
        courseCode,
        semester: selectedSemester,
        courseTerm: selectedCourse.term,
      },
      studentGroups: Array.from(selectedCourse.enabledStudentGroups.values()),
    });
  }

  const url = `${baseUrl}/calendar.ics?queries=${JSON.stringify(queries)}`;

  return (
    <div className="App">
      <div className="wrapper">
        <Container>
          <Grid container spacing={2} mt={2}>
            <Grid xs={7} md={8} lg={10}>
              <Autocomplete
                fullWidth
                options={searchedCourses}
                renderInput={(params) => (
                  <TextField {...params} label="Add course" />
                )}
                inputValue={courseSearch}
                onInputChange={(_, newCourseSearch) =>
                  setCourseSearch(newCourseSearch)
                }
                value={selectedCourse}
                onChange={(_, courseCode) => {
                  if (courseCode !== null) {
                    setSelectedCourses((pog) =>
                      pog.set(courseCode, {
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
                        selectedCourses.update(
                          courseCode,
                          (selectedCourse) => ({
                            ...selectedCourse!,
                            enabledStudentGroups:
                              selectedCourse!.enabledStudentGroups.includes(
                                studentGroup
                              )
                                ? selectedCourse!.enabledStudentGroups.remove(
                                    studentGroup
                                  )
                                : selectedCourse!.enabledStudentGroups.add(
                                    studentGroup
                                  ),
                          })
                        )
                      )
                    }
                  />
                </Grid>
              )
            )}
          </Grid>

          <Paper sx={{ padding: 2, mt: 2 }}>
            <Link href={url} target="_blank">
              {url}
            </Link>
          </Paper>
        </Container>
      </div>
    </div>
  );
}

export default App;
