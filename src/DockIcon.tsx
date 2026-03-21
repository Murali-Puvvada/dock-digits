import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import Toggle from "./components/Toggle";

interface DockIconConfigProps {
  isEnabled: boolean;
}

function DockIcon({ isEnabled }: DockIconConfigProps) {
  const [loading, setLoading] = useState(false);

  const handleToggle = async () => {
    setLoading(true);
    try {
      await invoke("set_dock_visibility");
    } catch (err) {
      console.error("Failed to toggle dock icon:", err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="px-5 py-5 flex flex-col gap-3 group">
      <div className="flex items-center justify-between">
        <div className="flex flex-col">
          <h4 className="text-zinc-100 font-semibold transition-colors group-hover:text-white">
            Show Dock Icon
          </h4>
          <p className="text-zinc-500 text-sm max-w-[240px]">
            Show the app icon in your macOS Dock
          </p>
        </div>

        <Toggle
          isEnabled={isEnabled}
          onToggle={handleToggle}
          loading={loading}
          ariaLabel="Show Dock Icon"
        />
      </div>
    </div>
  );
}

export default DockIcon;
