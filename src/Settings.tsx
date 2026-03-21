import { ArrowLeft } from "lucide-react";
import ShortcutConfig from "./ShortcutConfig";
import LaunchAtLogin from "./LaunchAtLogin";
import AutoUpdater from "./AutoUpdater";
import DockIcon from "./DockIcon";

interface SettingsPageProps {
  modifiers: string[];
  onModifiersChange: (modifiers: string[]) => void;
  launchAtLogin: boolean;
  showDockIcon: boolean;
  onBack: () => void;
}

function Settings({
  modifiers,
  onModifiersChange,
  launchAtLogin,
  showDockIcon,
  onBack,
}: SettingsPageProps) {
  return (
    <div className="flex-1 overflow-y-auto overflow-x-hidden custom-scrollbar animate-in fade-in slide-in-from-right-4 duration-300">
      <ShortcutConfig
        initialModifiers={modifiers}
        onApplied={onModifiersChange}
      />
      <LaunchAtLogin isEnabled={launchAtLogin} />
      <DockIcon isEnabled={showDockIcon} />
      <AutoUpdater />
      <div className="px-5 pb-8">
        <button
          onClick={onBack}
          className="w-full flex items-center justify-center gap-2 px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 rounded-lg transition-colors text-sm font-medium"
        >
          <ArrowLeft className="w-4 h-4" />
          Back to App List
        </button>
      </div>
    </div>
  );
}

export default Settings;
