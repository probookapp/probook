import { useEffect } from "react";
import { useBlocker } from "react-router-dom";

export function useUnsavedChangesGuard(when: boolean) {
  const blocker = useBlocker(when);

  // Browser close / refresh
  useEffect(() => {
    if (!when) return;
    const handler = (e: BeforeUnloadEvent) => {
      e.preventDefault();
    };
    window.addEventListener("beforeunload", handler);
    return () => window.removeEventListener("beforeunload", handler);
  }, [when]);

  return blocker;
}
