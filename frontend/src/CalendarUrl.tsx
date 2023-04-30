import { useMemo } from "react";
import { SelectedCourseState, swrOptions } from "./App";
import { CalendarQuery } from "../../api/bindings/CalendarQuery";
import useSWR from "swr";
import { Button, Stack, Typography } from "@mui/material";
import { baseUrl } from "./useFetch";
import { Apple, CalendarMonth } from "@mui/icons-material";

const encodeCalendarQueryFetcher = ([url, body]: [string, BodyInit]) =>
  fetch(baseUrl + url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body,
  }).then((res) => res.text());

function useEncodeCalendarQuery(queries: CalendarQuery[]) {
  const jsonQueries = useMemo(() => JSON.stringify(queries), [queries]);

  const query = useSWR(
    ["/encode-calendar-query", jsonQueries],
    encodeCalendarQueryFetcher,
    swrOptions
  ).data;

  return query;
}
interface CalendarUrlProps {
  semester: string;
  selectedCourses: Immutable.OrderedMap<string, SelectedCourseState>;
}

export function CalendarUrl(props: CalendarUrlProps) {
  const { semester, selectedCourses } = props;

  const queries = useMemo(() => {
    const queries: CalendarQuery[] = [];
    for (const [courseCode, selectedCourse] of selectedCourses) {
      queries.push({
        identifier: {
          courseCode,
          semester,
          courseTerm: selectedCourse.term,
        },
        studentGroups: Array.from(selectedCourse.enabledStudentGroups.values()),
      });
    }

    return queries;
  }, [selectedCourses]);

  const encodedQuery = useEncodeCalendarQuery(queries);

  const baseUrl = "https://ntnu-timeplan-api.fly.dev";
  const webcalUrl = "webcal://ntnu-timeplan-api.fly.dev";

  const calendarEndpoint = `/calendar.ics?query=${encodedQuery}`;

  const icsFileUrl = baseUrl + calendarEndpoint;
  const appleCalendarUrl = webcalUrl + calendarEndpoint;

  return (
    <>
      <Typography variant="h3" mt={6}>
        Add to calendar
      </Typography>

      <Stack mt={2} direction="row" gap={2}>
        <Button
          variant="outlined"
          startIcon={<Apple />}
          href={appleCalendarUrl}
        >
          Apple calendar
        </Button>
        <Button
          variant="outlined"
          startIcon={<CalendarMonth />}
          href={icsFileUrl}
        >
          ICS url
        </Button>
      </Stack>
    </>
  );
}
