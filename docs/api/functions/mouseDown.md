[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseDown

# Function: mouseDown()

> **mouseDown**(`button`): `void`

Defined in: [index.ts:846](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L846)

Press and hold a mouse button down

## Parameters

### button

Mouse button to hold: "left", "right", or "middle" (default: "left")

`"right"` | `"middle"` | `"left"`

## Returns

`void`

## Example

```typescript
import { mouseDown, mouseUp, moveMouse } from "@tego/botjs";

// Hold left button
mouseDown("left");

// Perform drag operation
moveMouse(500, 500);

// Release left button
mouseUp("left");
```
