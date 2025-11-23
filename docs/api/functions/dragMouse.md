[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / dragMouse

# Function: dragMouse()

> **dragMouse**(`x`, `y`): `void`

Defined in: [index.ts:176](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L176)

Drag the mouse from current position to target coordinates

## Parameters

### x

`number`

Target X coordinate in pixels

### y

`number`

Target Y coordinate in pixels

## Returns

`void`

## Example

```typescript
import { moveMouse, dragMouse } from "@tego/botjs";

// Move to start position
moveMouse(100, 100);

// Drag to end position
dragMouse(500, 500);
```
