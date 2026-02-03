import { describe, it, expect } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useSelection } from "./useSelection";

interface TestItem {
  id: string;
  name: string;
}

const items: TestItem[] = [
  { id: "1", name: "Item 1" },
  { id: "2", name: "Item 2" },
  { id: "3", name: "Item 3" },
];

describe("useSelection", () => {
  it("starts with empty selection", () => {
    const { result } = renderHook(() => useSelection(items));
    expect(result.current.selectedCount).toBe(0);
    expect(result.current.isAllSelected).toBe(false);
    expect(result.current.isIndeterminate).toBe(false);
  });

  it("toggles a single item", () => {
    const { result } = renderHook(() => useSelection(items));

    act(() => result.current.toggle("1"));
    expect(result.current.isSelected("1")).toBe(true);
    expect(result.current.selectedCount).toBe(1);

    act(() => result.current.toggle("1"));
    expect(result.current.isSelected("1")).toBe(false);
    expect(result.current.selectedCount).toBe(0);
  });

  it("selects all items with toggleAll", () => {
    const { result } = renderHook(() => useSelection(items));

    act(() => result.current.toggleAll());
    expect(result.current.isAllSelected).toBe(true);
    expect(result.current.selectedCount).toBe(3);
    expect(result.current.isIndeterminate).toBe(false);
  });

  it("deselects all when toggleAll is called with all selected", () => {
    const { result } = renderHook(() => useSelection(items));

    act(() => result.current.toggleAll());
    expect(result.current.isAllSelected).toBe(true);

    act(() => result.current.toggleAll());
    expect(result.current.selectedCount).toBe(0);
    expect(result.current.isAllSelected).toBe(false);
  });

  it("shows indeterminate when some but not all are selected", () => {
    const { result } = renderHook(() => useSelection(items));

    act(() => result.current.toggle("1"));
    expect(result.current.isIndeterminate).toBe(true);
    expect(result.current.isAllSelected).toBe(false);
  });

  it("clears all selections", () => {
    const { result } = renderHook(() => useSelection(items));

    act(() => result.current.toggle("1"));
    act(() => result.current.toggle("2"));
    expect(result.current.selectedCount).toBe(2);

    act(() => result.current.clear());
    expect(result.current.selectedCount).toBe(0);
  });

  it("handles undefined items", () => {
    const { result } = renderHook(() => useSelection(undefined));
    expect(result.current.isAllSelected).toBe(false);
    expect(result.current.isIndeterminate).toBe(false);
  });

  it("handles empty items array", () => {
    const { result } = renderHook(() => useSelection([]));
    expect(result.current.isAllSelected).toBe(false);
    expect(result.current.isIndeterminate).toBe(false);
  });
});
