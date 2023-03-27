import { useMemo } from "react";
import { SelectedCourseState, swrOptions } from "./App";
import { CalendarQuery } from "../../api/bindings/CalendarQuery";
import useSWR from "swr";
import { Link } from "@mui/material";
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

  return (
    <Link href={url} target="_blank" sx={{ wordBreak: "break-all" }}>
      {url}
    </Link>
  );
}
