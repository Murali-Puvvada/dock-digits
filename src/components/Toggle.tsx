interface ToggleProps {
  isEnabled: boolean;
  onToggle: () => void;
  loading?: boolean;
  ariaLabel?: string;
}

function Toggle({ isEnabled, onToggle, loading = false, ariaLabel }: ToggleProps) {
  return (
    <button
      onClick={onToggle}
      disabled={loading}
      role="switch"
      aria-checked={isEnabled}
      aria-label={ariaLabel}
      className={`relative inline-flex h-6 w-11 items-center rounded-full transition-all duration-200 focus:outline-hidden focus:ring-2 focus:ring-emerald-500/20 focus:ring-offset-2 focus:ring-offset-zinc-900 cursor-pointer ${
        isEnabled ? "bg-emerald-600" : "bg-zinc-700"
      } ${loading ? "opacity-50 cursor-not-allowed" : ""}`}
    >
      <span
        className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform duration-200 ease-in-out ${
          isEnabled ? "translate-x-6" : "translate-x-1"
        }`}
      />
    </button>
  );
}

export default Toggle;
