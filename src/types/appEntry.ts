export interface AppEntry {
    id: string;               // Unique identifier (UUID)
    name: string;             // Display name
    bundleId: string;         // macOS bundle identifier
    path: string;             // Full app path
    iconPath?: string;        // Optional icon path
    position: number;         // Position index (1-based)
    disabled: boolean;        // Whether shortcut is disabled
  }