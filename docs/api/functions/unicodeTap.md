[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / unicodeTap

# Function: unicodeTap()

> **unicodeTap**(`codePoint`): `void`

Defined in: [index.ts:365](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L365)

Tap a Unicode character by its code point

## Parameters

### codePoint

`number`

Unicode code point (e.g., 0x1F600 for 😀)

## Returns

`void`

## Example

```typescript
import { unicodeTap } from "@tego/botjs";

// Type emoji
unicodeTap(0x1F600); // 😀
unicodeTap(0x2764);  // ❤
unicodeTap(0x1F44D); // 👍
```
