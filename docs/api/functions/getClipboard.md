[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getClipboard

# Function: getClipboard()

> **getClipboard**(): `string`

Defined in: [index.ts:558](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L558)

Get text content from the system clipboard

## Returns

`string`

Current clipboard text content

## Example

```typescript
import { getClipboard } from "@tego/botjs";

const text = getClipboard();
console.log(`Clipboard contains: ${text}`);
```
