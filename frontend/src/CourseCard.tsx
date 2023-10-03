import { Suspense, useMemo } from "react";
import { CheckIcon, Cross1Icon } from "@radix-ui/react-icons";
import { Label } from "@radix-ui/react-label";
import { rspc } from "./rspc";
import * as Checkbox from "@radix-ui/react-checkbox";
import { Course } from "../../api/bindings";
import { css } from "./theme";

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

        <div className={Styles.StudentGroups.container()}>
          {Array.from(allStudentGroups.values()).map((studentGroup) => (
            <div key={studentGroup}>
              <Label className={Styles.StudentGroups.inputGroup()}>
                <Checkbox.Root
                  className={Styles.StudentGroups.checkbox()}
                  checked={enabledStudentGroups.includes(studentGroup)}
                  onClick={() => toggleStudentGroup(studentGroup)}
                >
                  <Checkbox.Indicator
                    className={Styles.StudentGroups.indicator()}
                  >
                    <CheckIcon />
                  </Checkbox.Indicator>
                </Checkbox.Root>

                {formatStudentGroup(studentGroup)}
              </Label>
            </div>
          ))}
        </div>
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
    <div key={courseCode} className={Styles.container()}>
      <div className={Styles.header()}>
        <div className={Styles.Names.container()}>
          <span className={Styles.Names.name()}>{course.name}</span>
          <span className={Styles.Names.courseCode()}>{courseCode}</span>
        </div>

        <button
          className={Styles.removeButton()}
          aria-label="Remove"
          onClick={props.onRemove}
        >
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

namespace Styles {
  export const container = css({
    padding: "$3 $4",

    backgroundColor: "color-mix(in lch, $background, white 5%)",
  });

  export const header = css({
    display: "flex",
    justifyContent: "space-between",
  });

  export namespace Names {
    export const container = css({
      display: "flex",
      flexDirection: "column",
    });

    export const name = css({
      fontSize: "$4",
      fontWeight: 400,
    });

    export const courseCode = css({
      fontSize: "$2",
      fontWeight: 500,
      opacity: 0.8,
    });
  }

  export const removeButton = css({
    padding: "$1",
    background: "none",
    border: "none",
    cursor: "pointer",

    "& > *": {
      width: 32,
      height: 32,
      color: "$secondary",
    },
  });

  export namespace StudentGroups {
    export const container = css({
      display: "flex",
      flexDirection: "column",
      gap: "$2",
    });

    export const inputGroup = css({
      display: "flex",
      gap: "$3",
      cursor: "pointer",
    });

    export const checkbox = css({
      display: "flex",
      alignItems: "center",
      justifyContent: "center",

      minWidth: 25,
      minHeight: 25,

      maxWidth: 25,
      maxHeight: 25,

      backgroundColor: "$foreground",
      border: "none",

      borderRadius: 4,

      "& svg": {
        width: 19,
        height: 19,
      },
    });

    export const indicator = css({
      color: "black",
    });
  }
}
