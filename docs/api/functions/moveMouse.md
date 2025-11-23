[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / moveMouse

# Function: moveMouse()

> **moveMouse**(`x`, `y`): `void`

Defined in: [index.ts:86](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L86)

Move the mouse cursor to the specified coordinates instantly

## Parameters

### x

`number`

X coordinate in pixels

### y

`number`

Y coordinate in pixels

## Returns

`void`

## Example

```typescript
import { moveMouse } from "@tego/botjs";

// Move to absolute position
moveMouse(100, 200);
```
