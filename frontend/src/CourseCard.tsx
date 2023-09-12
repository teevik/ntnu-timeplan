import { Suspense, useMemo } from "react";
import { CheckIcon, Cross1Icon } from "@radix-ui/react-icons";
import { Label } from "@radix-ui/react-label";
import { rspc } from "./rspc";
import * as Checkbox from "@radix-ui/react-checkbox";
import { Course } from "../../api/bindings";

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
      <hr />

      <div>
        <Label>
          Term
          <select
            value={term}
            onChange={(e) => setTerm(parseInt(e.target.value))}
          >
            {Array.from({ length: amountOfTerms }, (_, index) => index + 1).map(
              (term) => (
                <option key={term} value={term}>
                  Term {term}
                </option>
              )
            )}
          </select>
        </Label>
      </div>
    </>
  );
}

interface SelectStudentGroupsProps {
  courseCode: string;
  courseTerm: number;
  semester: string;
  enabledStudentGroups: string[];
  toggleStudentGroup: (studentGroup: string) => void;
}

function SelectStudentGroups(props: SelectStudentGroupsProps) {
  const {
    courseCode,
    courseTerm,
    semester,
    enabledStudentGroups,
    toggleStudentGroup,
  } = props;

  let activities = rspc.useQuery(
    ["activities", { courseCode, courseTerm, semester }],
    { suspense: true }
  ).data!;

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
      <hr />

      <div>
        <p>Student groups</p>
        {Array.from(allStudentGroups.values()).map((studentGroup) => (
          <div key={studentGroup}>
            <Label>
              <Checkbox.Root
                checked={enabledStudentGroups.includes(studentGroup)}
                onClick={() => toggleStudentGroup(studentGroup)}
              >
                <Checkbox.Indicator>
                  <CheckIcon />
                </Checkbox.Indicator>
              </Checkbox.Root>

              {formatStudentGroup(studentGroup)}
            </Label>
          </div>
        ))}
      </div>
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
  enabledStudentGroups: string[];
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
    <div key={courseCode}>
      <div>
        <div>
          <span>{course.name}</span>
          <span>{courseCode}</span>
        </div>

        <button aria-label="Remove" onClick={props.onRemove}>
          <Cross1Icon />
        </button>
      </div>

      <TermSelector
        term={term}
        setTerm={setTerm}
        amountOfTerms={course.amountOfTerms}
      />

      <Suspense>
        <SelectStudentGroups
          courseCode={courseCode}
          courseTerm={term}
          semester={semester}
          enabledStudentGroups={enabledStudentGroups}
          toggleStudentGroup={toggleStudentGroup}
        />
      </Suspense>
    </div>
  );
}
