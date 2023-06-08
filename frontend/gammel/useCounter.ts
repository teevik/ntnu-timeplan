import { useCallback, useState } from "react";

export function useCounter(
  initialValue: () => number,
  min: number,
  max: number
) {
  const [count, setCount] = useState(initialValue);

  const increment = useCallback(
    () => setCount((x) => (x < max ? x + 1 : x)),
    [setCount]
  );
  const decrement = useCallback(
    () => setCount((x) => (x > min ? x - 1 : x)),
    [setCount]
  );
  const reset = () => setCount(initialValue);

  return {
    value: count,
    increment,
    decrement,
    reset,
  };
}
