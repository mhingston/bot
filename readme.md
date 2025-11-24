<div align="center">
  <img src="assets/logo.png" alt="TegoBot Logo" width="400"/>

  <p>
    <strong>High-performance</strong> desktop automation library for <strong>Node.js</strong>.
    <br />
    Powered by <strong>Rust</strong> for extreme speed and memory safety.
  </p>

  <p>
    <a href="https://www.npmjs.com/package/@tego/botjs"><img src="https://img.shields.io/npm/v/@tego/botjs?style=flat-square&logo=npm&color=cb3837" alt="npm version" /></a>
    <a href="https://www.npmjs.com/package/@tego/botjs"><img src="https://img.shields.io/npm/dm/@tego/botjs?style=flat-square&logo=npm&color=cb3837" alt="npm downloads" /></a>
    <a href="https://github.com/tegojs/bot"><img src="https://img.shields.io/badge/code-3.5k%20lines-blue?style=flat-square" alt="Lines of code" /></a>
    <a href="https://github.com/tegojs/bot/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/tegojs/bot/ci.yml?branch=main&style=flat-square&logo=github&label=CI" alt="CI status" /></a>
    <a href="https://github.com/tegojs/bot/issues"><img src="https://img.shields.io/github/issues/tegojs/bot?style=flat-square&logo=github&color=yellow" alt="GitHub issues" /></a>
    <a href="https://github.com/tegojs/bot"><img src="https://img.shields.io/github/stars/tegojs/bot?style=flat-square&logo=github&color=blue" alt="GitHub stars" /></a>
    <a href="https://github.com/tegojs/bot/blob/main/LICENSE"><img src="https://img.shields.io/github/license/tegojs/bot?style=flat-square&color=green" alt="License" /></a>
    <a href="https://nodejs.org"><img src="https://img.shields.io/node/v/@tego/botjs?style=flat-square&logo=node.js&color=339933" alt="Node version" /></a>
    <a href="https://github.com/tegojs/bot/commits/main"><img src="https://img.shields.io/github/last-commit/tegojs/bot?style=flat-square&logo=git&color=orange" alt="Last commit" /></a>
  </p>
</div>

---

- **🚀 Extreme performance** – Rust core optimized for maximum speed & efficiency
- **🔒 Memory safe** – Rust's type system guarantees memory safety
- **🎯 API compatible** – Similar API design to robotjs for easy migration
- **🌍 Cross-platform** – Supports Windows, macOS, and Linux
- **📦 Zero dependencies** – No additional Node.js dependencies required
- **🧪 Well tested** – Comprehensive test coverage

---

## 🚀 Quick Start

You can add **Tego Bot** to your project via the `@tego/botjs` package:

```bash
pnpm add @tego/botjs

# Or: npm/yarn/bun add @tego/botjs
```

### Minimal Example

```ts
import { moveMouse, mouseClick, keyTap, typeString, captureScreen } from '@tego/botjs';

// Move mouse and click
moveMouse(100, 200);
mouseClick('left');

// Type text
typeString('Hello from Tego Bot!');
keyTap('enter');

// Capture screen
const screenshot = await captureScreen();
// screenshot.image contains PNG buffer
```

---

## 📖 API Documentation

### Mouse Operations

```ts
import { moveMouse, moveMouseSmooth, mouseClick, getMousePos, dragMouse, scrollMouse } from '@tego/botjs';

// Move mouse to coordinates
moveMouse(100, 200);

// Smooth movement
moveMouseSmooth(300, 400);
moveMouseSmooth(500, 600, 5.0); // with custom speed

// Click
mouseClick('left');           // Left click
mouseClick('right', true);    // Right double click
mouseClick('middle');         // Middle click

// Get mouse position
const pos = getMousePos();
console.log(`Mouse at: ${pos.x}, ${pos.y}`);

// Drag
dragMouse(500, 600);

// Scroll
scrollMouse(0, 3);  // Scroll down
scrollMouse(2, 0);  // Scroll right
```

### Keyboard Operations

```ts
import { keyTap, keyToggle, typeString, typeStringDelayed, unicodeTap } from '@tego/botjs';

// Tap a key
keyTap('a');
keyTap('enter');
keyTap('c', ['control']);        // Ctrl+C
keyTap('v', ['control', 'shift']); // Ctrl+Shift+V

// Toggle key
keyToggle('a', 'down');  // Press 'a'
keyToggle('a', 'up');    // Release 'a'

// Type text
typeString('Hello, World!');

// Type with delay (characters per minute)
typeStringDelayed('Hello', 300); // 300 CPM

// Tap Unicode character
unicodeTap(0x1F600); // 😀
```

### Screen Operations

```ts
import { captureScreen, captureScreenRegion, getScreenSize, getPixelColor, screen } from '@tego/botjs';
import fs from 'fs';

// Capture entire screen
const screenshot = await captureScreen();
fs.writeFileSync('screenshot.png', screenshot.image);

// Capture region
const region = await captureScreenRegion(100, 100, 800, 600);
fs.writeFileSync('region.png', region.image);

// Get screen size
const size = getScreenSize();
console.log(`Screen: ${size.width}x${size.height}`);

// Get pixel color (returns hex string)
const color = await getPixelColor(100, 200);
console.log(`Color: ${color}`); // e.g., "#FF0000"

// Using screen object
const bitmap = await screen.capture(0, 0, 800, 600);
const pixelColor = bitmap.colorAt(100, 200);
```

### Configuration

```ts
import { setMouseDelay, setKeyboardDelay, updateScreenMetrics } from '@tego/botjs';

// Set delays (in milliseconds)
setMouseDelay(50);
setKeyboardDelay(10);

// Update screen metrics (refresh monitor information)
updateScreenMetrics();
```

---

## 🎯 Supported Keys

### Modifier Keys
- `control` / `ctrl` – Control key
- `shift` – Shift key
- `alt` – Alt key
- `command` / `cmd` / `meta` – Command/Meta key

### Function Keys
- `f1` – `f12` – F1 to F12

### Special Keys
- `enter` / `return` – Enter key
- `escape` / `esc` – ESC key
- `backspace` – Backspace key
- `tab` – Tab key
- `space` – Space key
- `delete` / `del` – Delete key
- `up` / `down` / `left` / `right` – Arrow keys
- `home` / `end` – Home/End keys
- `pageup` / `page_down` – Page Up/Down keys

### Mouse Buttons
- `left` – Left button
- `right` – Right button
- `middle` – Middle button

---

## 🛠️ Building

To build the library from source:

```bash
# Install dependencies
pnpm install

# Build Rust code and generate Node.js bindings
pnpm build

# Or build only Rust code
pnpm rs:build
```

---

## 📋 System Requirements

### macOS
- macOS 10.13+
- Screen recording permission required (System Preferences > Security & Privacy > Screen Recording)

### Windows
- Windows 10+
- No additional configuration needed

### Linux
- X11 or Wayland
- May require system dependencies:
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev
  
  # Fedora
  sudo dnf install libxcb-devel libXrandr-devel dbus-devel
  ```

---

## 🧪 Testing

```bash
# Run Rust tests
pnpm test

# Build and test Node.js bindings
pnpm build
```

---

## 📊 Comparison with robotjs

| Feature | robotjs | Tego Bot (@tego/botjs) |
|---------|---------|------------------------|
| Performance | Medium (C++ bindings) | ⚡ Extremely high (Rust native) |
| Maintenance | ❌ No longer maintained | ✅ Actively maintained |
| Memory Safety | ⚠️ C++ | ✅ Rust |
| API Design | ✅ Simple | ✅ Compatible |
| Cross-platform | ✅ | ✅ |
| Type Safety | ⚠️ Runtime checks | ✅ Compile-time guarantees |

---

## 📄 License

Licensed under the [MIT License](LICENSE).

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---
