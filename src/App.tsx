import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AppEntry } from "./types/appEntry";
import { LaunchResult } from "./types/launchResult";
import "./App.css";

async function fetchDockApps() {
  const apps = await invoke<AppEntry[]>("get_dock_apps");
  return apps;
}

async function launchApp(app: AppEntry) {
  const result = await invoke<LaunchResult>("launch_app", {
    bundleId: app.bundleId ?? null,
    path: app.path ?? null,
  });

  console.log(result);

  if (!result.success) {
    alert(result.message);
  }
}

function App() {
  const [dockApps, setDockApps] = useState<AppEntry[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    async function init() {
      const apps = await fetchDockApps();
      setDockApps(apps);
    }

    init();
  }, []);

  async function refreshDock() {
    setLoading(true);
    const apps = await fetchDockApps();
    setDockApps(apps);
    setLoading(false);
  }

  return (
    <main className="container">
      <h1>Dock Digits</h1>

      <button onClick={refreshDock} disabled={loading}>
        {loading ? "Refreshing..." : "Refresh Dock"}
      </button>

      <ul>
        {dockApps.map((app) => (
          <li key={app.position}>
            {app.position}. {app.name}
          </li>
        ))}
      </ul>
    </main>
  );
}

export default App;
