[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseToggle

# Function: mouseToggle()

> **mouseToggle**(`down`, `button?`): `void`

Defined in: [index.ts:155](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L155)

Toggle mouse button state (press down or release up)

## Parameters

### down

`string`

"down" to press the button, "up" to release it

### button?

`string`

Mouse button: "left", "right", or "middle" (default: "left")

## Returns

`void`

## Example

```typescript
import { mouseToggle } from "@tego/botjs";

// Press and hold left button
mouseToggle('down', 'left');

// Perform some actions while button is held...

// Release left button
mouseToggle('up', 'left');
```
