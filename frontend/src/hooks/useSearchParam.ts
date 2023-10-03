import { useSearchParams } from "react-router-dom";

export function useSearchParam(name: string, defaultValue: string) {
  const [searchParams, setSearchParams] = useSearchParams();

  const rawSearchParam = searchParams.get(name);

  const setSearchParam = (newValue: string) => {
    setSearchParams((prev) => {
      prev.set(name, newValue);

      return prev;
    });
  };

  if (rawSearchParam == null) {
    setSearchParam(defaultValue);
  }

  const searchParam = rawSearchParam ?? defaultValue;
  return [searchParam, setSearchParam] as const;
}
