import { OrderedMap } from "immutable";
import { SelectedCourseState } from "./App";
import { useEffect, useMemo, useState } from "react";
import { DateTime } from "luxon";
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
    const activitiesPromise = Array.from(selectedCourses.entries()).map(
      ([courseCode, courseState]) => {
        const activities = fetch(
          `${baseUrl}/activities?courseCode=${courseCode}&courseTerm=${courseState.term}&semester=${semester}`,
          { signal: abortController.signal }
        )
          .then((response) => response.json() as Promise<Activity[]>)
          .then((activities) =>
            activities.filter((activity) =>
              courseState.enabledStudentGroups.some((targetStudentGroup) =>
                activity.studentGroups.includes(targetStudentGroup)
              )
            )
          );
        return activities;
      }
    );

    Promise.all(activitiesPromise).then((activities) => {
      setActivities(activities.flat());
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
            DateTime.fromISO(a.start),
            DateTime.fromISO(b.start)
          )
        ),
    [activities, week.value]
  );

  const renderBody = () => {
    if (activities.length == 0) {
      return <Typography>No courses selected</Typography>;
    } else if (activitiesInWeek.length == 0) {
      return <Typography>No activities in this week</Typography>;
    } else {
      return (
        <Stack direction="column" gap={2}>
          {activitiesInWeek.map((activity) => {
            const start = DateTime.fromISO(activity.start);
            const end = DateTime.fromISO(activity.end);

            return (
              <Card key={activity.id} sx={{ maxWidth: 400 }}>
                <CardHeader
                  title={activity.title}
                  subheader={activity.courseCode}
                />

                <CardContent>
                  <Typography>
                    {activity.rooms[0]?.buildingName} -{" "}
                    {activity.rooms[0]?.name}
                  </Typography>

                  <Typography>
                    {start.weekdayLong}{" "}
                    {start.toLocaleString({
                      hour: "2-digit",
                      minute: "2-digit",
                      hour12: false,
                    })}{" "}
                    -{" "}
                    {end.toLocaleString({
                      hour: "2-digit",
                      minute: "2-digit",
                      hour12: false,
                    })}
                  </Typography>
                </CardContent>
              </Card>
            );
          })}
        </Stack>
      );
    }
  };

  return (
    <>
      <Typography variant="h3" mt={6}>
        Timetable preview
      </Typography>

      <Stack direction="row" alignItems="center" gap={1} mt={1} mb={1}>
        <IconButton onClick={week.decrement}>
          <ChevronLeft />
        </IconButton>
        <Typography>Week {week.value}</Typography>
        <IconButton onClick={week.increment}>
          <ChevronRight />
        </IconButton>
      </Stack>

      {renderBody()}
    </>
  );
}
