[**Tego Bot API Documentation v0.1.1**](../README.md)

***

[Tego Bot API Documentation](../globals.md) / Mouse

# Variable: Mouse

> `const` **Mouse**: *typeof* `Mouse` = `bot.Mouse`

Defined in: [index.ts:52](https://github.com/tegojs/bot/blob/aa4091be1d7458b4935a672a5a88161f598afbb1/packages/botjs/src/index.ts#L52)

Mouse automation class for controlling mouse movements and clicks

## Example

```typescript
import { Mouse } from "@tego/botjs";

const mouse = new Mouse();
mouse.moveMouse(100, 200);
mouse.mouseClick('left');
mouse.dragMouse(500, 500);
```
