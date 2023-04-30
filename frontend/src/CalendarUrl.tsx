import { useMemo } from "react";
import { SelectedCourseState, swrOptions } from "./App";
import { CalendarQuery } from "../../api/bindings/CalendarQuery";
import useSWR from "swr";
import { Button, Link, Stack, Typography } from "@mui/material";
import { baseUrl } from "./useFetch";

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
  const url = `${baseUrl}/calendar.ics?query=${encodedQuery}`;
  // const previewUrl = `https://larrybolt.github.io/online-ics-feed-viewer/#feed=${url}f&cors=false`;
  // const previewUrl = `https://larrybolt.github.io/online-ics-feed-viewer/#feed=${url}&cors=false&title=My%20Feed&hideinput=false`;

  const query = new URLSearchParams({ feed: url, cors: "false" }).toString();
  const previewUrl = `https://larrybolt.github.io/online-ics-feed-viewer/#${query}`;

  return (
    <Stack alignItems="flex-start">
      <Button href={previewUrl} target="_blank">
        Preview
      </Button>

      <Stack direction="row" gap={1}>
        <Typography>URL:</Typography>

        <Link href={url} target="_blank" sx={{ wordBreak: "break-all" }}>
          {url}
        </Link>
      </Stack>
    </Stack>
  );
}
