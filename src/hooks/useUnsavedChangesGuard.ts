import { useEffect } from "react";
import { useBlocker } from "react-router-dom";

export function useUnsavedChangesGuard(when: boolean | (() => boolean)) {
  const shouldBlock = typeof when === "function" ? when : () => when;
  const blocker = useBlocker(shouldBlock);

  // Browser close / refresh
  useEffect(() => {
    const handler = (e: BeforeUnloadEvent) => {
      if (shouldBlock()) {
        e.preventDefault();
      }
    };
    window.addEventListener("beforeunload", handler);
    return () => window.removeEventListener("beforeunload", handler);
  }, [shouldBlock]);

  return blocker;
}
