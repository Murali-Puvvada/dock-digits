import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AppEntry } from "./types/appEntry";
import { LaunchResult } from "./types/launchResult";
import "./App.css";
import { Box, Settings as SettingsIcon } from "lucide-react";
import { isEnabled } from "@tauri-apps/plugin-autostart";

import { listen } from "@tauri-apps/api/event";

import DockSync from "./DockSync";
import Settings from "./Settings";

async function fetchDockApps() {
  const apps = await invoke<AppEntry[]>("get_dock_apps");
  return apps;
}

async function launchApp(app: AppEntry) {
  if (!app) return;
  const result = await invoke<LaunchResult>("launch_app", {
    bundleId: app.bundleId ?? null,
    path: app.path ?? null,
  });

  if (!result.success) {
    alert(result.message);
  }
}

function App() {
  const [dockApps, setDockApps] = useState<AppEntry[]>([]);
  const [loading, setLoading] = useState(false);
  const [launchAtLogin, setLaunchAtLogin] = useState(false);
  const [showDockIcon, setShowDockIcon] = useState(false);
  const [modifiers, setModifiers] = useState<string[]>(["option"]);
  const [showSettings, setShowSettings] = useState(false);

  // Internal refresh logic (action)
  const performRefresh = async () => {
    setLoading(true);
    try {
      const apps = await fetchDockApps();
      setDockApps(apps);
    } catch (err) {
      console.error("Failed to fetch dock apps:", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    async function init() {
      await performRefresh();
      const enabled = await isEnabled();
      setLaunchAtLogin(enabled);

      // Fetch persistent config
      try {
        const config = await invoke<{
          show_dock_icon: boolean;
          modifiers: string[];
        }>("get_config");
        setShowDockIcon(config.show_dock_icon);
        setModifiers(config.modifiers);
      } catch (err) {
        console.error("Failed to fetch config:", err);
      }
    }

    init();

    // Listen for events from Rust
    const unlistenLogin = listen<boolean>(
      "launch-at-login-updated",
      (event) => {
        setLaunchAtLogin(event.payload);
      },
    );

    const unlistenRefresh = listen("dock-apps-refreshed", () => {
      performRefresh();
    });

    const unlistenDockIcon = listen<boolean>("dock-icon-updated", (event) => {
      setShowDockIcon(event.payload);
    });

    const unlistenOpenSettings = listen("open-settings", () => {
      setShowSettings(true);
    });

    return () => {
      unlistenLogin.then((f) => f());
      unlistenRefresh.then((f) => f());
      unlistenDockIcon.then((f) => f());
      unlistenOpenSettings.then((f) => f());
    };
  }, []);

  return (
    <div className="h-screen w-screen bg-transparent flex flex-col">
      <div className="flex-1 flex flex-col bg-zinc-900/90 border border-zinc-800 shadow-2xl overflow-hidden backdrop-blur-md">
        {/* Window Header */}
        <div className="flex items-center justify-between px-5 py-4 border-b border-zinc-800 bg-zinc-900/50">
          {/* Layout Placeholder (replacing traffic lights) */}
          <div className="w-[52px]" aria-hidden="true" />

          {/* Title */}
          <div className="text-center">
            <h1 className="text-zinc-100 font-semibold text-sm tracking-wide">
              {showSettings ? "Settings" : "Dock Digits"}
            </h1>
            <p className="text-zinc-500 text-xs mt-0.5">
              {showSettings
                ? "Customize your experience"
                : "Launch apps via keyboard"}
            </p>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setShowSettings(!showSettings)}
              className={`p-2 rounded-lg transition-all ${
                showSettings
                  ? "bg-zinc-700 text-white"
                  : "bg-zinc-800/60 text-zinc-400 hover:text-zinc-100 hover:bg-zinc-800"
              }`}
              title={showSettings ? "Back to Apps" : "Settings"}
            >
              <SettingsIcon className="w-4 h-4" />
            </button>
            <DockSync loading={loading} />
          </div>
        </div>

        {/* Dynamic Content Area */}
        <div className="flex-1 flex flex-col min-h-0">
          {showSettings ? (
            <Settings
              modifiers={modifiers}
              onModifiersChange={setModifiers}
              launchAtLogin={launchAtLogin}
              showDockIcon={showDockIcon}
              onBack={() => setShowSettings(false)}
            />
          ) : (
            <div className="flex-1 overflow-y-auto overflow-x-hidden custom-scrollbar divide-y divide-zinc-800/50 animate-in fade-in slide-in-from-left-4 duration-300">
              {dockApps.length === 0 ? (
                <div className="px-5 py-12 text-center text-zinc-500 text-sm">
                  {loading
                    ? "Loading your dock apps..."
                    : "No apps found. Try syncing dock."}
                </div>
              ) : (
                dockApps.map((app) => {
                  return (
                    <div
                      key={app.position}
                      className="flex items-center gap-4 px-5 py-4 hover:bg-zinc-800/60 transition-all cursor-pointer group active:bg-zinc-800/90"
                    >
                      {/* Number */}
                      <span className="text-zinc-500 text-sm font-medium w-6 text-center group-hover:text-zinc-300 transition-colors">
                        {app.position}
                      </span>

                      {/* App Icon */}
                      <div
                        className={`w-12 h-12 rounded-xl flex items-center justify-center shadow-lg transform group-hover:scale-105 transition-transform duration-200 overflow-hidden ${
                          app.iconPath
                            ? "bg-transparent"
                            : "bg-linear-to-br from-zinc-600 to-zinc-800"
                        }`}
                      >
                        {app.iconPath ? (
                          <img
                            src={app.iconPath}
                            alt={`${app.name} icon`}
                            className="w-full h-full object-contain drop-shadow-md"
                          />
                        ) : (
                          <Box
                            className="w-6 h-6 text-zinc-100 shadow-sm"
                            strokeWidth={2}
                          />
                        )}
                      </div>

                      {/* App Info */}
                      <div className="flex-1">
                        <h3 className="text-zinc-100 font-medium tracking-wide group-hover:text-white transition-colors">
                          {app.name}
                        </h3>
                        <p className="text-zinc-500 text-xs mt-0.5 max-w-[200px] truncate">
                          {app.path}
                        </p>
                      </div>

                      {/* Keyboard Shortcut */}
                      <div
                        onClick={() => launchApp(app)}
                        className="flex items-center gap-1 bg-zinc-800/80 px-3 py-1.5 rounded-md border border-zinc-700/50 group-hover:border-zinc-500/50 group-hover:bg-zinc-700/80 transition-all shadow-sm"
                      >
                        {modifiers.map((m) => (
                          <span
                            key={m}
                            className="text-zinc-400 text-sm group-hover:text-zinc-300"
                          >
                            {m === "control"
                              ? "⌃"
                              : m === "option"
                                ? "⌥"
                                : m === "command"
                                  ? "⌘"
                                  : "⇧"}
                          </span>
                        ))}
                        <span className="text-zinc-400 text-sm group-hover:text-zinc-300 font-medium">
                          {app.position}
                        </span>
                      </div>
                    </div>
                  );
                })
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
