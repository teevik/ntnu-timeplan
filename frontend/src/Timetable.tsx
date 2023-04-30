import { OrderedMap } from "immutable";
import { SelectedCourseState } from "./App";
import { useEffect, useMemo, useState } from "react";
import { DateTime, WeekNumbers } from "luxon";
import {
  Card,
  CardContent,
  CardHeader,
  IconButton,
  Pagination,
  Stack,
  Typography,
} from "@mui/material";
import { ChevronLeft, ChevronRight } from "@mui/icons-material";
import { useCounter } from "./useCounter";
import { Activity } from "../../api/bindings/Activity";
import { Course } from "../../api/bindings/Course";
import { baseUrl } from "./useFetch";
import { CourseIdentifier } from "../../api/bindings/CourseIdentifier";

function compareLuxonDates(a: DateTime, b: DateTime) {
  return a.toMillis() - b.toMillis();
}

interface TimetableProps {
  semester: string;
  selectedCourses: OrderedMap<string, SelectedCourseState>;
  courses: Record<string, Course>;
}

export function Timetable(props: TimetableProps) {
  const { semester, selectedCourses, courses } = props;
  let week = useCounter(() => DateTime.now().weekNumber, 1, 53);

  const [activities, setActivities] = useState<Activity[]>([]);

  useEffect(() => {
    const abortController = new AbortController();
    const pog = Array.from(selectedCourses.entries()).map(
      ([courseCode, courseState]) => {
        const activities = fetch(
          `${baseUrl}/activities?courseCode=${courseCode}&courseTerm=${courseState.term}&semester=${semester}`,
          { signal: abortController.signal }
        ).then((response) => response.json() as Promise<Activity[]>);

        return activities;
      }
    );

    Promise.all(pog).then((pogger) => {
      const ting = pogger.flat();
      setActivities(ting);
    });

    return () => abortController.abort();
  }, [selectedCourses, semester]);

  const activitiesInWeek = useMemo(
    () =>
      activities
        .filter(
          (activity) =>
            DateTime.fromISO(activity.start).weekNumber === week.value
        )
        .sort((a, b) =>
          compareLuxonDates(
            DateTime.fromISO(b.start),
            DateTime.fromISO(a.start)
          )
        ),
    [activities, week.value]
  );

  return (
    <>
      <Stack direction="row" alignItems="center" gap={1}>
        <IconButton onClick={week.decrement}>
          <ChevronLeft />
        </IconButton>
        <Typography>Week {week.value}</Typography>
        <IconButton onClick={week.increment}>
          <ChevronRight />
        </IconButton>
      </Stack>

      <Stack direction="column" gap={2}>
        {activitiesInWeek.map((activity) => (
          <Card sx={{ maxWidth: 400 }}>
            <CardHeader
              title={activity.title}
              subheader={activity.courseCode}
            />

            <CardContent>{DateTime.fromISO(activity.start).to}</CardContent>
          </Card>
        ))}
      </Stack>
    </>
  );
}
