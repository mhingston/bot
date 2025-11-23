[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / getScreen

# Function: getScreen()

> **getScreen**(): `Screen`

Defined in: [index.ts:502](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L502)

Get the global Screen instance for capture operations

## Returns

`Screen`

Screen object

## Example

```typescript
import { getScreen } from "@tego/botjs";

const screen = getScreen();
const bitmap = await screen.capture(0, 0, 800, 600);
```
