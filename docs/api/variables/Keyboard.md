[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / Keyboard

# Variable: Keyboard

> `const` **Keyboard**: *typeof* `Keyboard` = `bot.Keyboard`

Defined in: [index.ts:37](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L37)

Keyboard automation class for simulating keyboard input

## Example

```typescript
import { Keyboard } from "@tego/botjs";

const keyboard = new Keyboard();
keyboard.keyTap('a');
keyboard.typeString('Hello World');
keyboard.keyTap('c', ['control']); // Ctrl+C
```
