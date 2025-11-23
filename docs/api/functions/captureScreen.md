[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / captureScreen

# Function: captureScreen()

> **captureScreen**(): `Promise`\<`ScreenCapture`\>

Defined in: [index.ts:434](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L434)

Capture the entire screen as a PNG image

## Returns

`Promise`\<`ScreenCapture`\>

Promise resolving to screen capture with PNG buffer

## Example

```typescript
import { captureScreen } from "@tego/botjs";
import fs from "fs";

const screenshot = await captureScreen();
fs.writeFileSync('screenshot.png', screenshot.image);
console.log(`Captured ${screenshot.width}x${screenshot.height} screenshot`);
```
