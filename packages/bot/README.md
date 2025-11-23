# @tego/bot

High-performance desktop automation library for Node.js, powered by Rust core with N-API bindings.

> **💡 Recommended**: Use **[@tego/botjs](https://www.npmjs.com/package/@tego/botjs)** instead! It provides a type-safe TypeScript wrapper with enhanced APIs, better documentation, and additional utility functions.

## Features

- 🚀 **High performance** - Written in Rust for maximum speed and efficiency
- 🎯 **API compatible** - Inspired by robotjs API design for easy migration
- 🔒 **Memory safe** - Rust's type system guarantees memory safety
- 🌍 **Cross-platform** - Supports Windows, macOS, and Linux
- 📦 **Zero dependencies** - No additional Node.js dependencies required
- 🧪 **Well tested** - Comprehensive unit and integration test coverage

## Installation

```bash
npm install @tego/bot
# or
pnpm add @tego/bot
# or
yarn add @tego/bot
```

**However, we strongly recommend using [@tego/botjs](https://www.npmjs.com/package/@tego/botjs) for better TypeScript support and additional features:**

```bash
npm install @tego/botjs
```

## Building from Source

```bash
cd packages/bot
npm run build
```

## API Documentation

### Mouse Operations

```typescript
import { Mouse } from '@tego/bot';

const mouse = new Mouse();

// Move mouse to coordinates
mouse.moveMouse(100, 200);

// Smooth movement
mouse.moveMouseSmooth(300, 400);
mouse.moveMouseSmoothWithSpeed(500, 600, 5.0); // Custom speed

// Click mouse buttons
mouse.mouseClick('left');           // Left click
mouse.mouseClick('right', true);    // Right double-click
mouse.mouseClick('middle');         // Middle click
mouse.mouseClick();                 // Default: left click

// Get mouse position
const pos = mouse.getMousePos();
console.log(`Mouse at: ${pos.x}, ${pos.y}`);

// Press/release mouse buttons
mouse.mouseToggle('down', 'left');  // Press left button
mouse.mouseToggle('up', 'left');    // Release left button
mouse.mouseToggle('down');          // Default: press left button

// Drag mouse
mouse.dragMouse(500, 600);

// Scroll mouse
mouse.scrollMouse(0, 3);  // Scroll down 3 units
mouse.scrollMouse(2, 0);  // Scroll right 2 units

// Set mouse operation delay (milliseconds)
mouse.setMouseDelay(50);
```

### Keyboard Operations

```typescript
// Method 1: Using class instance
import { Keyboard } from '@tego/bot';

const keyboard = new Keyboard();

// Tap key (press and release)
keyboard.keyTap('a');
keyboard.keyTap('enter');
keyboard.keyTap('c', ['control']);        // Ctrl+C
keyboard.keyTap('v', ['control', 'shift']); // Ctrl+Shift+V

// Press/release keys
keyboard.keyToggle('a', 'down');           // Press 'a'
keyboard.keyToggle('a', 'up');             // Release 'a'
keyboard.keyToggle('shift', 'down', ['control']); // Ctrl+Shift down

// Type text
keyboard.typeString('Hello, World!');

// Type with delay (characters per minute)
keyboard.typeStringDelayed('Hello', 300); // 300 CPM

// Set keyboard operation delay (milliseconds)
keyboard.setKeyboardDelay(10);
```

```typescript
// Method 2: Using global functions (Recommended)
import { keyTap, keyToggle, typeString, typeStringDelayed, unicodeTap, setKeyboardDelay } from '@tego/bot';

// Tap keys
keyTap('a');
keyTap('enter');
keyTap('c', ['control']);                 // Ctrl+C
keyTap('v', ['control', 'shift']);        // Ctrl+Shift+V

// Press/release
keyToggle('shift', 'down');               // Press Shift
keyToggle('shift', 'up');                 // Release Shift

// Type text
typeString('Hello, World!');
typeStringDelayed('Hello', 300);          // 300 CPM

// Unicode characters (e.g., emoji)
unicodeTap(0x1f600);                      // 😀

// Set delay
setKeyboardDelay(10);
```

### Screen Operations

```typescript
import { getScreen, getScreenSize, getPixelColor, bitmapColorAt } from '@tego/bot';
import type { Bitmap } from '@tego/bot';
import fs from 'fs';

// Get screen instance
const screen = getScreen();

// Capture entire screen
const fullScreen: Bitmap = await screen.capture();
fs.writeFileSync('screenshot.png', fullScreen.image);
console.log(`Captured: ${fullScreen.width}x${fullScreen.height}`);

// Capture screen region (x, y, width, height)
const region: Bitmap = await screen.capture(100, 100, 800, 600);
fs.writeFileSync('region.png', region.image);

// Get screen size
const size = getScreenSize();
console.log(`Screen size: ${size.width}x${size.height}`);

// Get pixel color at coordinates (returns hex string like "#FF0000")
const color = await getPixelColor(100, 200);
console.log(`Pixel color: ${color}`);

// Get color from bitmap at coordinates
const bitmapColor = bitmapColorAt(region, 50, 50);
console.log(`Color at (50, 50) in bitmap: ${bitmapColor}`);
```

## Complete Example

```typescript
import { Mouse, Keyboard, getScreen, moveMouse, keyTap, typeString } from '@tego/bot';
import fs from 'fs';

async function automationExample() {
    // Using class instances
    const mouse = new Mouse();
    const keyboard = new Keyboard();

    // Move mouse and click
    mouse.moveMouseSmooth(500, 300);
    mouse.mouseClick('left');

    // Type text
    keyboard.typeString('Hello from @tego/bot!');
    keyboard.keyTap('enter');

    // Or use global functions
    moveMouse(600, 400);
    keyTap('enter');
    typeString('Using global functions');

    // Capture screen
    const screen = getScreen();
    const screenshot = await screen.capture();
    fs.writeFileSync('automation.png', screenshot.image);

    console.log('Automation completed!');
}

automationExample();
```

## Supported Keys

### Modifier Keys
- `control` / `ctrl` - Control key
- `shift` - Shift key
- `alt` - Alt key
- `command` / `cmd` / `meta` - Command/Meta key

### Function Keys
- `f1` - `f12` - F1 through F12

### Special Keys
- `enter` / `return` - Enter key
- `escape` / `esc` - ESC key
- `backspace` - Backspace key
- `tab` - Tab key
- `space` - Space key
- `delete` / `del` - Delete key
- `up` / `down` / `left` / `right` - Arrow keys
- `home` / `end` - Home/End keys
- `pageup` / `page_down` - Page Up/Down keys

### Mouse Buttons
- `left` - Left button
- `right` - Right button
- `middle` - Middle button

## Comparison with robotjs

| Feature | robotjs | @tego/bot | @tego/botjs |
|---------|---------|-----------|-------------|
| Performance | Medium (C++ bindings) | ⚡ Extremely high (Rust native) | ⚡ Extremely high (Rust native) |
| Maintenance | ❌ No longer maintained | ✅ Actively maintained | ✅ Actively maintained |
| Memory Safety | ⚠️ C++ | ✅ Rust | ✅ Rust |
| API Design | ✅ Simple | ✅ Compatible | ✅ Enhanced |
| Cross-platform | ✅ | ✅ | ✅ |
| Type Safety | ⚠️ Runtime checks | ✅ Compile-time guarantees | ✅ Full TypeScript support |
| Test Coverage | ⚠️ Limited | ✅ Comprehensive | ✅ Comprehensive |
| Additional APIs | ❌ | ❌ | ✅ Enhanced screen utilities |

## Why Use @tego/botjs Instead?

[@tego/botjs](https://www.npmjs.com/package/@tego/botjs) is the recommended wrapper that provides:

- **Full TypeScript support** with complete type definitions
- **Enhanced screen APIs** including `captureScreen()`, `captureScreenRegion()`, and more
- **Better documentation** with extensive examples
- **Utility functions** for common automation tasks
- **Improved developer experience** with better error messages

```typescript
// @tego/botjs provides cleaner APIs:
import { captureScreen, captureScreenRegion } from '@tego/botjs';

const screenshot = await captureScreen();
const region = await captureScreenRegion(0, 0, 800, 600);
```

## Testing

Run tests:

```bash
# Rust unit tests
cd packages/bot
cargo test

# Build and test Node.js bindings
npm run build

# JavaScript tests (in botjs package)
cd ../botjs
pnpm test

# Integration tests (requires system interaction)
ENABLE_INTEGRATION_TESTS=true pnpm test:integration
```

## System Requirements

### macOS
- macOS 10.13+ 
- Screen recording permission required (System Preferences > Security & Privacy > Screen Recording)

### Windows
- Windows 10+
- No additional configuration needed

### Linux
- X11 or Wayland
- Required system dependencies:
  ```bash
  # Ubuntu/Debian
  sudo apt-get install -y \
    build-essential \
    pkg-config \
    libwayland-dev \
    libxcb1-dev \
    libxrandr-dev \
    libdbus-1-dev \
    libpipewire-0.3-dev \
    libegl1-mesa-dev \
    libgles2-mesa-dev \
    libgbm-dev \
    libxi-dev \
    libxtst-dev
  
  # Fedora
  sudo dnf install \
    gcc \
    pkg-config \
    wayland-devel \
    libxcb-devel \
    libXrandr-devel \
    dbus-devel \
    pipewire-devel \
    mesa-libEGL-devel \
    mesa-libGLES-devel \
    libgbm-devel \
    libXi-devel \
    libXtst-devel
  ```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit Issues and Pull Requests.

## Related Projects

- [@tego/botjs](https://www.npmjs.com/package/@tego/botjs) - **Recommended TypeScript wrapper** with enhanced APIs
- [@tego/bot-agent](https://www.npmjs.com/package/@tego/bot-agent) - AI-powered CLI for generating automation scripts
- [robotjs](https://github.com/octalmage/robotjs) - Original Node.js automation library
- [enigo](https://github.com/enigo-rs/enigo) - Rust keyboard and mouse control library
- [xcap](https://github.com/nashaofu/xcap) - Rust screen capture library

## 📚 Additional Resources

### API References

- **[AutoHotkey API Reference](./docs/autohotkey-api-reference.md)** - Inspiration for feature expansion
- **[Hammerspoon API Reference](./docs/hammerspoon-api-reference.md)** - macOS automation API reference
- **[Python Automation Libraries](./docs/python-automation-libraries.md)** - Similar libraries in Python ecosystem
