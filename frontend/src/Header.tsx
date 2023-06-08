import {
  CheckIcon,
  ChevronDownIcon,
  ChevronUpIcon,
} from "@radix-ui/react-icons";
import { css } from "./theme";
import * as Select from "@radix-ui/react-select";
import { SemestersWithCurrent } from "./api/apiSchemas";
import { Label } from "@radix-ui/react-label";

interface HeaderProps {
  semesters: SemestersWithCurrent;
  selectedSemester: string;
  setSelectedSemester: (semester: string) => void;
}

export function Header(props: HeaderProps) {
  const { semesters, selectedSemester, setSelectedSemester } = props;

  const selectedSemesterName = semesters.semesters[selectedSemester].name;

  const semestersEntries = Object.entries(semesters.semesters);

  return (
    <header className={Styles.container()}>
      <div className={Styles.content()}>
        <a href="/" className={Styles.title()}>
          <h1 className={Styles.title()}>NTNU Timeplan</h1>
        </a>

        <Label className={Styles.Select.container()}>
          Semester
          <Select.Root
            value={selectedSemester}
            onValueChange={setSelectedSemester}
          >
            <Select.Trigger className={Styles.Select.trigger()}>
              <Select.Value>{selectedSemesterName}</Select.Value>
              <Select.Icon>
                <ChevronDownIcon />
              </Select.Icon>
            </Select.Trigger>
            <Select.Portal>
              <Select.Content className={Styles.Select.content()}>
                <Select.ScrollUpButton className={Styles.Select.scrollButton()}>
                  <ChevronUpIcon />
                </Select.ScrollUpButton>

                <Select.Viewport className={Styles.Select.viewport()}>
                  {semestersEntries.map(([key, semester]) => (
                    <Select.Item
                      key={key}
                      value={key}
                      className={Styles.Select.item()}
                    >
                      <Select.ItemText>{semester.name}</Select.ItemText>
                      <Select.ItemIndicator
                        className={Styles.Select.selectedIndicator()}
                      >
                        <CheckIcon />
                      </Select.ItemIndicator>
                    </Select.Item>
                  ))}
                </Select.Viewport>

                <Select.ScrollDownButton
                  className={Styles.Select.scrollButton()}
                >
                  <ChevronDownIcon />
                </Select.ScrollDownButton>
              </Select.Content>
            </Select.Portal>
          </Select.Root>
        </Label>
      </div>
    </header>
  );
}

namespace Styles {
  export const container = css({
    display: "flex",
    justifyContent: "center",

    backgroundColor: "color-mix(in lch, $background, white 10%)",
  });

  export const content = css({
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    width: "min($maxWidth, 100vw)",
    padding: "$3 $4",
  });

  export const title = css({
    margin: 0,
    color: "$foreground",
    textDecoration: "none",
  });

  export namespace Select {
    export const container = css({
      display: "flex",
      width: "140px",
      flexDirection: "column",
      gap: "$1",

      fontSize: "$2",
      fontWeight: 500,
    });

    export const trigger = css({
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      padding: "$2 $2",
      gap: "$2",

      lineHeight: 1,
      fontSize: `$2`,

      border: "none",
      borderRadius: "4px",
      backgroundColor: "$secondary",
      color: "$background",
    });

    export const content = css({
      overflow: "hidden",

      backgroundColor: "$secondary",
      color: "$background",
      borderRadius: "6px",
      boxShadow: "$3",
    });

    export const viewport = css({
      padding: "$2",
    });

    export const item = css({
      position: "relative",
      display: "flex",
      alignItems: "center",
      height: "25px",
      padding: "$3 $4",
      userSelect: "none",

      fontSize: "$2",

      borderRadius: "3px",

      "&[data-highlighted]": {
        backgroundColor: "color-mix(in lch, $secondary, black 10%)",
      },
    });

    export const selectedIndicator = css({
      position: "absolute",
      left: 0,
      width: "25px",
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
    });

    export const scrollButton = css({
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      height: "25px",
      backgroundColor: "$secondary",
      cursor: "default",
    });
  }
}
