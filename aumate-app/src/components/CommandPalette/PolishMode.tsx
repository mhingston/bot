import { Copy, Sparkles, Square, X } from "lucide-react";
import { forwardRef, useCallback, useEffect, useRef, useState } from "react";
import Markdown from "react-markdown";
import { polishExpression } from "@/lib/openai";
import { useSettingsStore } from "@/stores/settingsStore";

interface PolishModeProps {
  query: string;
  isActive: boolean;
}

export const PolishMode = forwardRef<HTMLDivElement, PolishModeProps>(
  function PolishMode({ query, isActive }, _ref) {
    const [polishResult, setPolishResult] = useState("");
    const [polishError, setPolishError] = useState("");
    const [isPolishing, setIsPolishing] = useState(false);
    const abortControllerRef = useRef<AbortController | null>(null);
    const polishScrollRef = useRef<HTMLDivElement>(null);
    const { settings } = useSettingsStore();

    // Cancel polishing request
    const cancelPolishing = useCallback(() => {
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
        abortControllerRef.current = null;
      }
      setIsPolishing(false);
    }, []);

    // Polish expression
    const doPolish = useCallback(async () => {
      if (!query.trim() || isPolishing) return;

      const { expression_polishing } = settings;

      setIsPolishing(true);
      setPolishResult("");
      setPolishError("");

      abortControllerRef.current = new AbortController();

      const result = await polishExpression({
        apiUrl: expression_polishing.api_url,
        apiKey: expression_polishing.api_key,
        model: expression_polishing.model,
        systemPrompt: expression_polishing.system_prompt,
        userInput: query,
        signal: abortControllerRef.current.signal,
        onChunk: (chunk) => {
          setPolishResult((prev) => prev + chunk);
        },
      });

      setIsPolishing(false);
      abortControllerRef.current = null;

      if (result.error) {
        setPolishError(result.error);
      }
    }, [query, settings, isPolishing]);

    // Clear polish results
    const clearPolishResults = useCallback(() => {
      setPolishResult("");
      setPolishError("");
    }, []);

    // Copy polished expression to clipboard (only the polished text, not adjustments)
    const copyToClipboard = useCallback(async () => {
      if (polishResult) {
        const polishedOnly = extractPolishedExpression(polishResult);
        await navigator.clipboard.writeText(polishedOnly);
      }
    }, [polishResult]);

    // Keyboard navigation
    useEffect(() => {
      if (!isActive) return;

      const handleKeyDown = (e: KeyboardEvent) => {
        // Ctrl+P to scroll up
        if (e.ctrlKey && e.key === "p") {
          e.preventDefault();
          if (polishScrollRef.current) {
            polishScrollRef.current.scrollBy({ top: -100, behavior: "smooth" });
          }
          return;
        }

        // Ctrl+N to scroll down
        if (e.ctrlKey && e.key === "n") {
          e.preventDefault();
          if (polishScrollRef.current) {
            polishScrollRef.current.scrollBy({ top: 100, behavior: "smooth" });
          }
          return;
        }

        // Ctrl+C to copy polished expression if no text selected
        if (e.ctrlKey && e.key === "c") {
          if (polishResult) {
            const selection = window.getSelection();
            const hasSelection = selection && selection.toString().length > 0;
            if (!hasSelection) {
              e.preventDefault();
              copyToClipboard();
            }
          }
          return;
        }

        switch (e.key) {
          case "Enter":
            e.preventDefault();
            doPolish();
            break;
        }
      };

      window.addEventListener("keydown", handleKeyDown);
      return () => window.removeEventListener("keydown", handleKeyDown);
    }, [isActive, polishResult, copyToClipboard, doPolish]);

    // Clear results when query changes
    useEffect(() => {
      if (query === "") {
        clearPolishResults();
      }
    }, [query, clearPolishResults]);

    return (
      <>
        <div
          ref={polishScrollRef}
          className="flex-1 overflow-y-auto command-list"
        >
          {!polishResult && !polishError && !isPolishing && (
            <div className="px-4 py-8 text-center text-muted-foreground">
              <Sparkles className="w-8 h-8 mx-auto mb-3 text-purple-400/50" />
              <p>Enter text and press Enter to polish</p>
              <p className="text-xs mt-1 text-muted-foreground/60">
                AI will improve your expression and explain the changes
              </p>
            </div>
          )}

          {isPolishing && !polishResult && (
            <div className="px-4 py-8 text-center text-muted-foreground">
              <Sparkles className="w-8 h-8 mx-auto mb-3 text-purple-400 animate-pulse" />
              <p>Polishing your expression...</p>
            </div>
          )}

          {polishError && (
            <div className="px-4 py-4">
              <div className="p-3 bg-red-500/10 border border-red-500/20 rounded-lg">
                <p className="text-sm text-red-400">{polishError}</p>
              </div>
            </div>
          )}

          {polishResult && (
            <div className="px-4 py-4 space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-xs font-medium text-purple-400">
                  Result
                </span>
                <div className="flex items-center gap-1">
                  <button
                    type="button"
                    onClick={copyToClipboard}
                    className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-white/5 rounded transition-colors"
                    title="Copy to clipboard"
                  >
                    <Copy className="w-4 h-4" />
                  </button>
                  <button
                    type="button"
                    onClick={clearPolishResults}
                    className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-white/5 rounded transition-colors"
                    title="Clear"
                  >
                    <X className="w-4 h-4" />
                  </button>
                </div>
              </div>
              <div className="p-3 bg-white/5 rounded-lg text-sm text-foreground prose prose-invert prose-sm max-w-none prose-p:my-2 prose-ul:my-2 prose-li:my-0.5 prose-headings:my-2 prose-strong:text-purple-300">
                <Markdown>{polishResult}</Markdown>
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between px-4 py-2 border-t border-white/10 text-xs text-muted-foreground">
          <div className="flex items-center gap-4">
            <span className="flex items-center gap-1">
              <kbd className="px-1.5 py-0.5 bg-muted rounded">Enter</kbd>
              <span>Polish</span>
            </span>
            {polishResult && (
              <>
                <span className="flex items-center gap-1">
                  <kbd className="px-1.5 py-0.5 bg-muted rounded">Ctrl+P/N</kbd>
                  <span>Scroll</span>
                </span>
                <span className="flex items-center gap-1">
                  <kbd className="px-1.5 py-0.5 bg-muted rounded">Ctrl+C</kbd>
                  <span>Copy</span>
                </span>
              </>
            )}
            {isPolishing && (
              <button
                type="button"
                onClick={cancelPolishing}
                className="flex items-center gap-1 text-red-400 hover:text-red-300"
              >
                <Square className="w-3 h-3" />
                <span>Stop</span>
              </button>
            )}
          </div>
          <span className="text-purple-400/60">Expression Polishing</span>
        </div>
      </>
    );
  },
);

// Extract just the polished expression from the result (between **Polished:** and **Adjustments:**)
export function extractPolishedExpression(result: string): string {
  // Try to find the polished section
  const polishedMatch = result.match(
    /\*\*Polished:\*\*\s*([\s\S]*?)(?=\*\*Adjustments:\*\*|$)/i,
  );
  if (polishedMatch) {
    return polishedMatch[1].trim();
  }

  // Fallback: try without markdown formatting
  const plainMatch = result.match(/Polished:\s*([\s\S]*?)(?=Adjustments:|$)/i);
  if (plainMatch) {
    return plainMatch[1].trim();
  }

  // If no pattern found, return the whole result
  return result.trim();
}
