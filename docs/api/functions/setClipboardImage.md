[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setClipboardImage

# Function: setClipboardImage()

> **setClipboardImage**(`imageBuffer`): `void`

Defined in: [index.ts:627](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L627)

Set image to clipboard from a PNG-encoded buffer

## Parameters

### imageBuffer

`Buffer`

PNG-encoded image buffer

## Returns

`void`

## Example

```typescript
import { setClipboardImage } from "@tego/botjs";
import fs from "fs";

const imageData = fs.readFileSync('image.png');
setClipboardImage(imageData);
console.log('Image copied to clipboard');
```
