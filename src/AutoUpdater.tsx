import { useState, useEffect } from "react";
import { check } from "@tauri-apps/plugin-updater";
import { listen } from "@tauri-apps/api/event";
import { RefreshCw, CheckCircle2, AlertCircle } from "lucide-react";

export default function AutoUpdater() {
  const [checking, setChecking] = useState(false);
  const [updateAvailable, setUpdateAvailable] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string>("Up to date");

  const checkForUpdates = async (silent = false) => {
    if (checking) return;
    setChecking(true);
    setError(null);
    if (!silent) setStatus("Checking for updates...");

    try {
      const update = await check();
      if (update) {
        setUpdateAvailable(update);
        setStatus(`Version ${update.version} available`);
      } else {
        setUpdateAvailable(null);
        setStatus("Up to date");
      }
    } catch (err) {
      console.error(err);
      setError(`Error: ${err}`);
      setStatus("Error checking updates");
    } finally {
      setChecking(false);
    }
  };

  const installUpdate = async () => {
    if (!updateAvailable) return;
    try {
      setStatus("Downloading and installing...");
      await updateAvailable.downloadAndInstall();
    } catch (err) {
      console.error(err);
      setError("Failed to install update");
    }
  };

  useEffect(() => {
    // Listen for manual check triggered from tray
    const unlisten = listen("check-for-updates", () => {
      checkForUpdates();
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="px-5 py-6 border-b border-zinc-800/50">
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <h4 className="text-zinc-100 font-semibold">Check for Updates</h4>
          <p className="text-zinc-500 text-sm">
            Keep Dock Digits up to date with the latest features
          </p>
        </div>

        <div className="flex items-center gap-3">
          {updateAvailable ? (
            <button
              onClick={installUpdate}
              className="px-5 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-sm font-medium transition-all shadow-lg shadow-emerald-900/20 active:scale-95"
            >
              Install Now
            </button>
          ) : (
            <button
              onClick={() => checkForUpdates()}
              disabled={checking}
              className="px-5 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-sm font-medium transition-all shadow-lg shadow-emerald-900/20 disabled:opacity-50 disabled:bg-zinc-800 disabled:text-zinc-500 active:scale-95"
            >
              {checking ? (
                <div className="flex items-center gap-2">
                  <RefreshCw className="w-3.5 h-3.5 animate-spin" />
                  <span>Checking...</span>
                </div>
              ) : (
                "Check Now"
              )}
            </button>
          )}
        </div>
      </div>

      {/* Status/Error Messages */}
      {(error || (updateAvailable && status)) && (
        <div className="mt-4 animate-in fade-in slide-in-from-top-2 duration-300">
          {error ? (
            <div className="flex items-center gap-2 text-red-400 text-[11px] bg-red-400/5 p-2 rounded-md border border-red-400/10">
              <AlertCircle className="w-3.5 h-3.5" />
              <span>{error}</span>
            </div>
          ) : updateAvailable ? (
            <div className="flex items-center gap-2 text-emerald-400 text-[11px] bg-emerald-400/5 p-2 rounded-md border border-emerald-400/10">
              <CheckCircle2 className="w-3.5 h-3.5" />
              <span>{status}</span>
            </div>
          ) : null}
        </div>
      )}
    </div>
  );
}
