import { invoke } from "@tauri-apps/api/core";

async function toggleLaunchAtLogin() {
  try {
    await invoke("toggle_launch_at_login");
  } catch (err) {
    console.error("Failed to toggle launch at login:", err);
  }
}

function LaunchAtLogin({ isEnabled }: { isEnabled: boolean }) {
  return (
    <div className="flex items-center justify-between px-5 py-5">
      <div>
        <h4 className="text-zinc-100 font-semibold">Launch at Login</h4>
        <p className="text-zinc-500 text-sm">
          Open Dock Digits automatically when you log in
        </p>
      </div>
      <button
        onClick={toggleLaunchAtLogin}
        className={`relative w-12 h-7 rounded-full transition-colors shrink-0 cursor-pointer ${
          isEnabled ? "bg-emerald-500" : "bg-zinc-700"
        }`}
        role="switch"
        aria-checked={isEnabled}
      >
        <span
          className={`absolute top-0.5 left-0.5 w-6 h-6 bg-white rounded-full shadow-md transition-transform duration-200 ${
            isEnabled ? "translate-x-5" : "translate-x-0"
          }`}
        />
      </button>
    </div>
  );
}

export default LaunchAtLogin;
