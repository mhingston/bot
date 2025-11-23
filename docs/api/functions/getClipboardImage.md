[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getClipboardImage

# Function: getClipboardImage()

> **getClipboardImage**(): `Buffer`

Defined in: [index.ts:608](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L608)

Get image from clipboard as a PNG-encoded buffer

## Returns

`Buffer`

PNG-encoded image buffer

## Example

```typescript
import { getClipboardImage } from "@tego/botjs";
import fs from "fs";

const imageBuffer = getClipboardImage();
fs.writeFileSync('clipboard.png', imageBuffer);
```
