[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / scrollMouse

# Function: scrollMouse()

> **scrollMouse**(`x`, `y`): `void`

Defined in: [index.ts:200](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L200)

Scroll the mouse wheel in horizontal and/or vertical directions

## Parameters

### x

`number`

Horizontal scroll amount (positive = right, negative = left)

### y

`number`

Vertical scroll amount (positive = down, negative = up)

## Returns

`void`

## Example

```typescript
import { scrollMouse } from "@tego/botjs";

// Scroll down
scrollMouse(0, 3);

// Scroll up
scrollMouse(0, -3);

// Scroll right
scrollMouse(2, 0);
```
