[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / Screen

# Variable: Screen

> `const` **Screen**: *typeof* `Screen` = `bot.Screen`

Defined in: [index.ts:66](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L66)

Screen capture class for taking screenshots and getting pixel colors

## Example

```typescript
import { Screen } from "@tego/botjs";

const screen = new Screen();
const bitmap = await screen.capture(0, 0, 800, 600);
console.log(`Captured ${bitmap.width}x${bitmap.height} region`);
```
