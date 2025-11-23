[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getMousePos

# Function: getMousePos()

> **getMousePos**(): `MousePosition`

Defined in: [index.ts:217](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L217)

Get the current mouse cursor position

## Returns

`MousePosition`

Object containing x and y coordinates

## Example

```typescript
import { getMousePos } from "@tego/botjs";

const pos = getMousePos();
console.log(`Mouse is at: ${pos.x}, ${pos.y}`);
```
