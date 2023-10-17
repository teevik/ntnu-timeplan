import { useEffect, useState } from "react";
import { css } from "./theme";
import { Apple, CalendarMonth } from "@mui/icons-material";

interface AddToCalendarProps {
  calendarQuery: string;
  isEnabled: boolean;
}

export function AddToCalendar(props: AddToCalendarProps) {
  const { calendarQuery, isEnabled } = props;

  const apiUrl = "https://ntnu-timeplan-api.fly.dev";
  const apiWebcalUrl = "webcal://ntnu-timeplan-api.fly.dev";

  const calendarUrl = `${apiUrl}/calendar.ics?query=${calendarQuery}`;
  const webcalCalendarUrl = `${apiWebcalUrl}/calendar.ics?query=${calendarQuery}`;

  const [lastTimeCopied, setLastTimeCopied] = useState<number>();

  useEffect(() => {
    if (lastTimeCopied != undefined) {
      const timeout = setTimeout(() => {
        setLastTimeCopied(undefined);
      }, 1500);

      return () => clearTimeout(timeout);
    }
  }, [lastTimeCopied]);

  function onClickCopyICal() {
    navigator.clipboard.writeText(calendarUrl);
    setLastTimeCopied(Date.now());
  }

  const recentlyCopied = lastTimeCopied != undefined;

  return (
    <div className={Styles.container()}>
      <h1 className={Styles.title()}>Add to calendar</h1>

      <a
        href={isEnabled ? webcalCalendarUrl : undefined}
        className={Styles.button({ state: isEnabled ? "enabled" : "disabled" })}
      >
        <Apple />
        Add to Apple Calendar
      </a>
      <button
        onClick={onClickCopyICal}
        disabled={!isEnabled}
        className={Styles.button({ state: isEnabled ? "enabled" : "disabled" })}
      >
        <CalendarMonth />
        {recentlyCopied ? "Copied!" : "Copy ICal subscription URL"}
      </button>
    </div>
  );
}

namespace Styles {
  export const container = css({
    display: "flex",
    flexDirection: "column",
    gap: "$3",

    padding: "$4 $4",
    marginTop: "$4",

    backgroundColor: "color-mix(in lch, $background, white 5%)",
  });

  export const title = css({
    fontSize: "$4",
    fontWeight: "normal",
    margin: 0,
  });

  export const button = css({
    display: "flex",
    alignItems: "center",
    gap: "$2",
    padding: "$2 $3",
    alignSelf: "start",

    color: "$background",
    textDecoration: "none",
    fontSize: "$3",
    fontWeight: "normal",
    cursor: "pointer",

    background: "$secondary",
    border: "none",

    variants: {
      state: {
        enabled: {},

        disabled: {
          opacity: 0.2,
          cursor: "default",
        },
      },
    },
  });
}
