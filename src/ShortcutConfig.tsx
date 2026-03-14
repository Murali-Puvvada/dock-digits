import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

const MODIFIER_OPTIONS = [
  { label: "⌃ Control", value: "control" },
  { label: "⌥ Option", value: "option" },
  { label: "⌘ Command", value: "command" },
  { label: "⇧ Shift", value: "shift" },
];

async function applyModifiers(modifiers: string[]) {
  await invoke("set_shortcut_modifiers", { modifiers });
}

interface ShortcutConfigProps {
  initialModifiers: string[];
  onApplied?: (modifiers: string[]) => void;
}

function ShortcutConfig({ initialModifiers, onApplied }: ShortcutConfigProps) {
  const [selected, setSelected] = useState<string[]>(initialModifiers);
  const [status, setStatus] = useState<"idle" | "saving" | "saved" | "error">(
    "idle",
  );

  const toggle = (value: string) => {
    setSelected((prev) =>
      prev.includes(value)
        ? prev.length > 1
          ? prev.filter((m) => m !== value)
          : prev // keep at least one
        : [...prev, value],
    );
  };

  const handleApply = async () => {
    setStatus("saving");
    try {
      await applyModifiers(selected);
      setStatus("saved");
      onApplied?.(selected);
      setTimeout(() => setStatus("idle"), 1500);
    } catch (err) {
      console.error("Failed to apply modifiers:", err);
      setStatus("error");
      setTimeout(() => setStatus("idle"), 2000);
    }
  };

  return (
    <div className="px-5 py-5 flex flex-col gap-3">
      <div>
        <h4 className="text-zinc-100 font-semibold">Shortcut Modifier</h4>
        <p className="text-zinc-500 text-sm">
          Choose which modifier key(s) trigger your dock shortcuts
        </p>
      </div>

      <div className="flex flex-wrap gap-2">
        {MODIFIER_OPTIONS.map(({ label, value }) => {
          const active = selected.includes(value);
          return (
            <button
              key={value}
              onClick={() => toggle(value)}
              className={`px-3 py-1.5 rounded-md text-sm font-medium border transition-all cursor-pointer ${
                active
                  ? "bg-emerald-500/20 border-emerald-500/60 text-emerald-400"
                  : "bg-zinc-800/60 border-zinc-700/50 text-zinc-400 hover:border-zinc-500/50 hover:text-zinc-300"
              }`}
            >
              {label}
            </button>
          );
        })}
      </div>

      <button
        onClick={handleApply}
        disabled={status === "saving"}
        className={`self-start px-4 py-1.5 rounded-md text-sm font-medium transition-all cursor-pointer ${
          status === "saved"
            ? "bg-emerald-600 text-white"
            : status === "error"
              ? "bg-red-600/80 text-white"
              : "bg-zinc-700 text-zinc-200 hover:bg-zinc-600"
        }`}
      >
        {status === "saving"
          ? "Applying…"
          : status === "saved"
            ? "✓ Applied"
            : status === "error"
              ? "Failed"
              : "Apply"}
      </button>
    </div>
  );
}

export default ShortcutConfig;
