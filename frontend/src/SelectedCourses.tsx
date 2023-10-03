import { Dispatch } from "react";
import { SelectedCourseState } from "./App";
import { CourseCard } from "./CourseCard";
import { Course } from "../../api/bindings";
import { css } from "./theme";

interface SelectedCoursesProps {
  courses: Record<string, Course>;
  selectedSemester: string;
  selectedCourses: SelectedCourseState[];
  setSelectedCourses: Dispatch<
    (prev: SelectedCourseState[]) => SelectedCourseState[]
  >;
}

export function SelectedCourses(props: SelectedCoursesProps) {
  const { courses, selectedSemester, selectedCourses, setSelectedCourses } =
    props;

  return (
    <div className={Styles.container()}>
      {selectedCourses.map(({ courseCode, enabledStudentGroups, term }) => (
        <CourseCard
          key={courseCode}
          courseCode={courseCode}
          term={term}
          setTerm={(term) =>
            setSelectedCourses((selectedCourses) => {
              const arrayIndex = selectedCourses.findIndex(
                (course) => course.courseCode === courseCode
              );

              const newSelectedCourses = [...selectedCourses];

              newSelectedCourses[arrayIndex] = {
                courseCode,
                enabledStudentGroups: [],
                term,
              };

              return newSelectedCourses;
            })
          }
          semester={selectedSemester}
          course={courses[courseCode]}
          onRemove={() =>
            setSelectedCourses(() =>
              selectedCourses.filter(
                (selectedCourse) => selectedCourse.courseCode !== courseCode
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

              if (!enabledStudentGroups.includes(studentGroup)) {
                newEnabledStudentGroups = [
                  ...enabledStudentGroups,
                  studentGroup,
                ];
              } else {
                newEnabledStudentGroups = enabledStudentGroups.filter(
                  (target) => target !== studentGroup
                );
              }

              const newSelectedCourses = [...selectedCourses];

              newSelectedCourses[arrayIndex] = {
                courseCode,
                enabledStudentGroups: newEnabledStudentGroups,
                term,
              };

              return newSelectedCourses;
            })
          }
        />
      ))}
    </div>
  );
}

namespace Styles {
  export const container = css({
    display: "grid",
    alignItems: "stretch",
    gap: "$3",
    gridTemplateColumns: "1fr",
    "@media (min-width: 840px)": {
      gridTemplateColumns: "1fr 1fr",
    },
  });
}
