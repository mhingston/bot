[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getScreenSize

# Function: getScreenSize()

> **getScreenSize**(): `ScreenSize`

Defined in: [index.ts:519](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L519)

Get the dimensions of the primary screen

## Returns

`ScreenSize`

Object containing width and height in pixels

## Example

```typescript
import { getScreenSize } from "@tego/botjs";

const size = getScreenSize();
console.log(`Screen resolution: ${size.width}x${size.height}`);
```
