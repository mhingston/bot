[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / setMouseDelay

# Function: setMouseDelay()

> **setMouseDelay**(`delay`): `void`

Defined in: [index.ts:238](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L238)

Set the delay between mouse operations in milliseconds

## Parameters

### delay

`number`

Delay in milliseconds (applied after each mouse operation)

## Returns

`void`

## Example

```typescript
import { setMouseDelay, moveMouse } from "@tego/botjs";

// Set 50ms delay between operations
setMouseDelay(50);

// These will have 50ms delay between them
moveMouse(100, 100);
moveMouse(200, 200);
```
