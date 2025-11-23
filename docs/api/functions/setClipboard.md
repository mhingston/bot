[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setClipboard

# Function: setClipboard()

> **setClipboard**(`text`): `void`

Defined in: [index.ts:575](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L575)

Set text content to the system clipboard

## Parameters

### text

`string`

Text to copy to clipboard

## Returns

`void`

## Example

```typescript
import { setClipboard } from "@tego/botjs";

setClipboard('Hello from @tego/bot!');
setClipboard('user@example.com');
```
