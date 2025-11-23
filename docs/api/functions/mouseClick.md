[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / mouseClick

# Function: mouseClick()

> **mouseClick**(`button?`, `double?`): `void`

Defined in: [index.ts:132](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L132)

Click the mouse button at the current cursor position

## Parameters

### button?

`string`

Mouse button: "left", "right", or "middle" (default: "left")

### double?

`boolean`

Whether to perform a double click (default: false)

## Returns

`void`

## Example

```typescript
import { mouseClick } from "@tego/botjs";

// Single left click
mouseClick('left');

// Double right click
mouseClick('right', true);

// Single middle click
mouseClick('middle');
```
