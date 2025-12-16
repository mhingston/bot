import { invoke } from "@tauri-apps/api/core";
import { Loader2 } from "lucide-react";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { WindowItem, type WindowItemData } from "./WindowItem";

interface SwitcherModeProps {
  query: string;
  onHide: () => void;
  isActive: boolean;
}

export function SwitcherMode({ query, onHide, isActive }: SwitcherModeProps) {
  const [windows, setWindows] = useState<WindowItemData[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const listRef = useRef<HTMLDivElement>(null);

  const loadWindows = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const windowElements = await invoke<WindowItemData[]>(
        "get_window_elements",
      );
      // Filter out the aumate app itself
      const filtered = windowElements.filter(
        (w) =>
          !w.title.toLowerCase().includes("aumate") &&
          !w.app_name.toLowerCase().includes("aumate"),
      );
      setWindows(filtered);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Load windows on mount
  useEffect(() => {
    loadWindows();
  }, [loadWindows]);

  // Filter windows by query
  const filteredWindows = useMemo(() => {
    return windows.filter((w) => {
      if (!query.trim()) return true;
      const searchLower = query.toLowerCase();
      return (
        w.title.toLowerCase().includes(searchLower) ||
        w.app_name.toLowerCase().includes(searchLower)
      );
    });
  }, [windows, query]);

  // Auto-scroll to keep selected item visible
  useEffect(() => {
    if (listRef.current && filteredWindows.length > 0) {
      const selectedElement = listRef.current.children[
        selectedIndex
      ] as HTMLElement;
      if (selectedElement) {
        selectedElement.scrollIntoView({
          block: "nearest",
          behavior: "smooth",
        });
      }
    }
  }, [selectedIndex, filteredWindows.length]);

  // Reset selection when filtered list changes
  useEffect(() => {
    if (selectedIndex >= filteredWindows.length) {
      setSelectedIndex(Math.max(0, filteredWindows.length - 1));
    }
  }, [filteredWindows.length, selectedIndex]);

  // Switch to window
  const handleSwitchToWindow = useCallback(
    async (windowId: number) => {
      try {
        await invoke("switch_to_window", { windowId });
        onHide();
      } catch (err) {
        console.error("Failed to switch window:", err);
      }
    },
    [onHide],
  );

  // Close window
  const handleCloseWindow = useCallback(async (windowId: number) => {
    try {
      await invoke("close_desktop_window", { windowId });
      // Remove from local list
      setWindows((prev) => prev.filter((w) => w.window_id !== windowId));
    } catch (err) {
      console.error("Failed to close window:", err);
    }
  }, []);

  // Keyboard navigation
  useEffect(() => {
    if (!isActive) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+P for up
      if (e.ctrlKey && e.key === "p") {
        e.preventDefault();
        setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
        return;
      }

      // Ctrl+N for down
      if (e.ctrlKey && e.key === "n") {
        e.preventDefault();
        setSelectedIndex((prev) =>
          prev < filteredWindows.length - 1 ? prev + 1 : prev,
        );
        return;
      }

      // Ctrl+W to close window
      if (e.ctrlKey && e.key === "w") {
        e.preventDefault();
        if (filteredWindows[selectedIndex]) {
          handleCloseWindow(filteredWindows[selectedIndex].window_id);
        }
        return;
      }

      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          setSelectedIndex((prev) =>
            prev < filteredWindows.length - 1 ? prev + 1 : prev,
          );
          break;
        case "ArrowUp":
          e.preventDefault();
          setSelectedIndex((prev) => (prev > 0 ? prev - 1 : prev));
          break;
        case "Enter":
          e.preventDefault();
          if (filteredWindows[selectedIndex]) {
            handleSwitchToWindow(filteredWindows[selectedIndex].window_id);
          }
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [
    isActive,
    filteredWindows,
    selectedIndex,
    handleSwitchToWindow,
    handleCloseWindow,
  ]);

  if (isLoading) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        <Loader2 className="w-5 h-5 animate-spin mr-2" />
        Loading windows...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex-1 flex items-center justify-center text-red-400 px-4 text-center">
        <div>
          <p className="font-medium">Failed to load windows</p>
          <p className="text-sm text-muted-foreground mt-1">{error}</p>
        </div>
      </div>
    );
  }

  if (filteredWindows.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center text-muted-foreground">
        {query.trim() ? "No windows match your search" : "No windows found"}
      </div>
    );
  }

  return (
    <>
      <div ref={listRef} className="flex-1 overflow-y-auto">
        {filteredWindows.map((window, index) => (
          <WindowItem
            key={window.window_id}
            window={window}
            selected={index === selectedIndex}
            onClick={() => handleSwitchToWindow(window.window_id)}
          />
        ))}
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between px-4 py-2 border-t border-white/10 text-xs text-muted-foreground">
        <div className="flex items-center gap-4">
          <span className="flex items-center gap-1">
            <kbd className="px-1.5 py-0.5 bg-muted rounded">↑</kbd>
            <kbd className="px-1.5 py-0.5 bg-muted rounded">↓</kbd>
            <span>Navigate</span>
          </span>
          <span className="flex items-center gap-1">
            <kbd className="px-1.5 py-0.5 bg-muted rounded">Enter</kbd>
            <span>Switch</span>
          </span>
          <span className="flex items-center gap-1">
            <kbd className="px-1.5 py-0.5 bg-muted rounded">Ctrl+W</kbd>
            <span>Close</span>
          </span>
        </div>
        <span className="text-sky-400/60">Window Switcher</span>
      </div>
    </>
  );
}

export type { WindowItemData };
export default SwitcherMode;
