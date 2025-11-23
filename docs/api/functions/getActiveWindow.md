[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getActiveWindow

# Function: getActiveWindow()

> **getActiveWindow**(): `WindowInfo`

Defined in: [index.ts:651](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L651)

Get information about the currently active (focused) window

## Returns

`WindowInfo`

WindowInfo object with title, process, position, and dimensions

## Example

```typescript
import { getActiveWindow } from "@tego/botjs";

const win = getActiveWindow();
console.log(`Active window: ${win.title}`);
console.log(`Process: ${win.processPath} (PID: ${win.processId})`);
console.log(`Position: (${win.x}, ${win.y})`);
console.log(`Size: ${win.width}x${win.height}`);
```
