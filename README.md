# Dock Digits

Bring Windows-style positional app launching to macOS.

Dock Digits lets you launch or focus your favorite apps using numbered keyboard shortcuts — similar to `Win + 1`, `Win + 2` on Windows.

Fast. Lightweight. Keyboard-first.

---

## ✨ Features

- 🔢 Launch apps using position-based shortcuts (e.g. `Option + 1`)
- 🚀 Automatically focuses app if already running
- 📂 Drag-and-drop app ordering
- 🧠 Persistent configuration
- 🖥 Runs as a lightweight background utility
- 🔄 Optional launch at login

---

## 🧩 How It Works

1. Add apps to your Dock Digits list.
2. Arrange them in your preferred order.
3. Use numbered shortcuts to launch or focus them.

| Shortcut        | Action                     |
|-----------------|----------------------------|
| Option + 1      | Open first app in list     |
| Option + 2      | Open second app in list    |
| Option + 3      | Open third app in list     |

If the app is:

- Not running → it launches  
- Already running → it comes to the front  

---

## 🛠 Tech Stack

- **Frontend:** React + TypeScript  
- **Framework:** Tauri  
- **Backend:** Rust  
- **Platform:** macOS  

---

## 🚀 Getting Started

### Prerequisites

- macOS
- Node.js (v18+)
- Rust
- Tauri CLI

### Install Dependencies

```bash
git clone https://github.com/Murali-Puvvada/dock-digits
cd dock-digits
bun install
bun tauri dev