import { RefreshCw } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

async function handleRefreshClick() {
  try {
    // This will emit 'dock-apps-refreshed' which our listener handles
    await invoke("refresh_dock_apps");
  } catch (err) {
    console.error("Failed to trigger refresh:", err);
  }
}

function DockSync({ loading }: { loading: boolean }) {
  return (
    <button
      onClick={handleRefreshClick}
      disabled={loading}
      className={`flex items-center gap-2 bg-emerald-600 hover:bg-emerald-500 text-zinc-100 px-4 py-2 rounded-lg text-sm font-medium transition-all shadow-md active:scale-95 ${loading ? "opacity-70 cursor-not-allowed" : ""}`}
    >
      <RefreshCw className={`w-4 h-4 ${loading ? "animate-spin" : ""}`} />
      {loading ? "Syncing..." : "Dock Sync"}
    </button>
  );
}

export default DockSync;
