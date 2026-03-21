import { invoke } from "@tauri-apps/api/core";
import Toggle from "./components/Toggle";

async function toggleLaunchAtLogin() {
  try {
    await invoke("toggle_launch_at_login");
  } catch (err) {
    console.error("Failed to toggle launch at login:", err);
  }
}

function LaunchAtLogin({ isEnabled }: { isEnabled: boolean }) {
  return (
    <div className="px-5 py-5 flex flex-col gap-3 group">
      <div className="flex items-center justify-between">
        <div className="flex flex-col">
          <h4 className="text-zinc-100 font-semibold transition-colors group-hover:text-white">
            Launch at Login
          </h4>
          <p className="text-zinc-500 text-sm max-w-[240px]">
            Open Dock Digits automatically when you log in
          </p>
        </div>
        <Toggle
          isEnabled={isEnabled}
          onToggle={toggleLaunchAtLogin}
          ariaLabel="Launch at Login"
        />
      </div>
    </div>
  );
}

export default LaunchAtLogin;
