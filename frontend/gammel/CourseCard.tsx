import {
  Card,
  CardContent,
  CardHeader,
  Checkbox,
  Divider,
  FormControlLabel,
  IconButton,
  MenuItem,
  Stack,
  TextField,
  Typography,
} from "@mui/material";
import { Suspense, useMemo } from "react";
import { Activity } from "../../api/bindings/Activity";
import { Course } from "../../api/bindings/Course";
import ClearIcon from "@mui/icons-material/Clear";
import { useFetch } from "./useFetch";

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

export function CourseCard(props: CourseCardProps) {
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

      <Suspense>
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
