[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setKeyboardDelay

# Function: setKeyboardDelay()

> **setKeyboardDelay**(`ms`): `void`

Defined in: [index.ts:386](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L386)

Set the delay between keyboard operations in milliseconds

## Parameters

### ms

`number`

Delay in milliseconds

## Returns

`void`

## Example

```typescript
import { setKeyboardDelay, keyTap } from "@tego/botjs";

// Set 10ms delay between key presses
setKeyboardDelay(10);

// These will have 10ms delay between them
keyTap('h');
keyTap('i');
```
